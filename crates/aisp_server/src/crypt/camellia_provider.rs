use crate::crypt::crypt_provider::CryptProvider;

struct CamelliaProvider {}

impl CamelliaProvider {}

impl CryptProvider for CamelliaProvider {
    fn set_key(&mut self, key: &[u8]) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn encrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn decrypt(&mut self, data: &mut [u8]) -> Result<(), std::io::Error> {
        Ok(())
    }
}
