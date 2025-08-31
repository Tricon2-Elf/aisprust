use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, LitInt, Path, parse_macro_input};

#[proc_macro_derive(Packet, attributes(packet))]
pub fn derive_packet(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut packet_id = None;

    for attr in &input.attrs {
        if attr.path().is_ident("packet") {
            // Parse attribute like #[packet(1234)]
            let arg: LitInt = attr.parse_args().expect("expected #[packet(<int>)]");
            packet_id = Some(arg.base10_parse::<u16>().expect("packet ID must be a u32"));
            break;
        }
    }

    let id = match packet_id {
        Some(id) => id,
        None => panic!("Missing #[packet(<id>)] attribute"),
    };

    let expanded = quote! {
        impl Packet for #name {
            const ID: u16 = #id;
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn packet_register(input: TokenStream) -> TokenStream {
    let paths = parse_macro_input!(input with syn::punctuated::Punctuated::<Path, syn::Token![,]>::parse_terminated);

    let variants = paths.iter().map(|path| {
        let last = path.segments.last().unwrap().ident.to_string();
        let variant = format_ident!("Id{}", last);
        quote! { #variant(#path) = <#path as Packet>::ID }
    });

    let from_impls = paths.iter().map(|path| {
        let last = path.segments.last().unwrap().ident.to_string();
        let variant = format_ident!("Id{}", last);
        quote! {

            impl From<#path> for PacketId {
                fn from(pkt: #path) -> Self {
                    PacketId::#variant(pkt)
                }
            }
        }
    });

    let match_from_bytes = paths.iter().map(|path| {
        let last = path.segments.last().unwrap().ident.to_string();
        let variant = format_ident!("Id{}", last);
        quote! {
            <#path as Packet>::ID => {
                let pkt: #path = match deserializer::from_bytes::<#path>(payload) {
                    Ok(pkt) => pkt,
                    Err(e) => return Err(ParseError::SerdeDeserialize(e)),
                };
                Ok(PacketId::#variant(pkt))
            }
        }
    });

    let match_to = paths.iter().map(|path| {
        let last = path.segments.last().unwrap().ident.to_string();
        let variant = format_ident!("Id{}", last);
        quote! {
            PacketId::#variant(pkt) => {
                out.extend(&<#path as Packet>::ID.to_le_bytes());
                out.extend(&match serializer::to_bytes(&pkt) {
                    Ok(bts) => bts,
                    Err(e) => return Err(ParseError::SerdeSerialize(e)),
                });
            }
        }
    });

    let match_id = paths.iter().map(|path| {
        let last = path.segments.last().unwrap().ident.to_string();
        let variant = format_ident!("Id{}", last);
        quote! {
            PacketId::#variant(_) => <#path as Packet>::ID
        }
    });

    let expanded = quote! {
        #[derive(Debug)]
        #[repr(u16)]
        pub enum PacketId {
            #(#variants),*
        }

        impl PacketId {


            pub fn from_bytes(input: &[u8]) -> Result<Self, ParseError> {
                if input.len() < 2 {
                    return Err(ParseError::Truncated);
                }
                let id = u16::from_le_bytes([input[0], input[1]]);
                let payload = &input[2..];

                match id {
                    #(#match_from_bytes,)*
                    _ => Err(ParseError::UnknownPacket(id)),
                }
            }

            pub fn to_bytes(&self) -> Result<Vec<u8>, ParseError> {
                let mut out = Vec::new();
                match self {
                    #(#match_to),*
                }
                Ok(out)
            }

            pub fn id(&self) -> u16 {
                match self {
                    #(#match_id),*
                }
            }
        }

        #(#from_impls)*
    };

    expanded.into()
}

// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
