// macro_rules! packet_register {
//     (
//         $($variant:ident($ty:path)),* $(,)?
//     ) => {
//         #[derive(Debug)]
//         pub enum PacketId {
//             $(
//                 #[allow(non_camel_case_types)]
//                 $variant($ty),
//             )*
//         }
//
//         impl PacketId {
//             pub fn from_bytes(input: &[u8]) -> Result<Self, MyError> {
//                 if input.len() < 2 {
//                     return Err(MyError::Truncated);
//                 }
//                 let id = u16::from_le_bytes([input[0], input[1]]);
//                 let payload = &input[2..];
//
//                 match id {
//                     $(
//                         <$ty as Packet>::ID => {
//                             let mut de: $ty = deserializer::from_bytes(payload);
//                             Ok(PacketId::$ty(pkt))
//                         }
//                     )*
//                     _ => Err(MyError::UnknownOpcode(id)),
//                 }
//             }
//
//             pub fn to_bytes(&self) -> Result<Vec<u8>, MyError> {
//                 let mut out = Vec::new();
//                 match self {
//                     $(
//                         PacketId::$ty(pkt) => {
//                             out.extend(&<$ty as Packet>::ID.to_le_bytes());
//                             // let mut ser = BinarySerializer::new();
//                             // pkt.serialize(&mut ser)?;
//                             // out.extend(ser.output());
//                         }
//                     )*
//                 }
//                 Ok(out)
//             }
//
//             pub fn id(&self) -> u16 {
//                 match self {
//                     $(
//                         PacketId::$ty(_) => <$ty as Packet>::ID,
//                     )*
//                 }
//             }
//         }
//     };
// }
