use crate::crypt::crypto_stream::CryptProvider;

use camellia::{
    Camellia128,
    cipher::{BlockDecrypt, BlockEncrypt, KeyInit, generic_array::GenericArray},
};

// TODO: This is ghetto, im ghetto.
fn camellia_inck(this: &mut Camellia128)  {
    let state: &mut [u64; 26] = unsafe {
        // Replace 26 with the real size of k[] in the struct
        let ptr = this as *mut Camellia128 as *mut [u64; 26];
        &mut *ptr
    };


    // the games camellia implementation increates k[0] each decrypt/encrypt.
    // also uses 32 bits instead of 64 bits so we need to do some bit stuff to get the first 32
    // bits and +1 it

    let cur_k0 = state[0];

    let mut v1: u32 = (cur_k0 >> 32) as u32;
    let v2: u32     = cur_k0         as u32;

    v1 += 1;

    state[0] = ((v1 as u64) << 32) | (v2 as u64);

}

// trait CamelliaWrapper {
//     fn wrap_encrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error>;
//     fn wrap_decrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error>;
// }
//
// // impl<KeySize: cipher::array::ArraySize, const RK: usize> CamelliaWrapper for Camellia<KeySize, RK> {
// impl CamelliaWrapper for Camellia128 {
//     fn wrap_encrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error> {
//         get_mut_k()
//         // self.k[0] += 1;
//         // let internal: &mut Camellia128 = unsafe { std::mem::transmute(&self) };
//         // internal.k[0] += 1;
//         Ok(())
//     }
//     fn wrap_decrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error> {
//         Ok(())
//     }
// }

pub struct CamelliaProvider {
    ctx: Option<Camellia128>,
}

impl CamelliaProvider {
    pub fn new() -> Self {
        Self { ctx: None }
    }
}

impl CryptProvider for CamelliaProvider {
    fn initialize(&mut self, key: &[u8]) -> Result<(), std::io::Error> {
        self.ctx = Some(Camellia128::new(GenericArray::from_slice(&key)));
        Ok(())
    }

    fn reinitialize(&mut self) -> Result<(), std::io::Error> {
        self.ctx = None;
        Ok(())
    }

    fn encrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error> {
        if (data.len() % 0x10) != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "must be 0x10 block sizes",
            ));
        }

        let mut ctx = match &mut self.ctx {
            Some(ctx) => ctx,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "no cryption",
                ));
            }
        };

        for i in 0..data.len() / 0x10 {
            let mut block_data = GenericArray::from([0u8; 0x10]);
            block_data.copy_from_slice(&data[i * 0x10..(i + 1) * 0x10]);

            ctx.encrypt_block(&mut block_data);
            data[i * 0x10..(i + 1) * 0x10].copy_from_slice(&block_data);


            // println!("Key: {:#x}", get_mut_k(ctx)[0]);
            camellia_inck(ctx);
        }

        Ok(())
    }
    fn decrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error> {
        if (data.len() % 0x10) != 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "must be 0x10 block sizes",
            ));
        }

        let mut ctx = match &mut self.ctx {
            Some(ctx) => ctx,
            None => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Unsupported,
                    "no cryption",
                ));
            }
        };

        for i in 0..data.len() / 0x10 {
            let mut block_data = GenericArray::from([0u8; 0x10]);
            block_data.copy_from_slice(&data[i * 0x10..(i + 1) * 0x10]);

            ctx.decrypt_block(&mut block_data);
            data[i * 0x10..(i + 1) * 0x10].copy_from_slice(&block_data);


            // TODO: This is ghetto, im ghetto.
            camellia_inck(ctx);
        }

        Ok(())
    }
}
