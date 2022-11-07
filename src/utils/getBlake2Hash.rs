use crate::error::ErrCode;
use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;

pub fn getBlake2Hash(to_hash: &[u8], length: Option<u32>) -> Result<Vec<u8>, ErrCode> {
    let len = length.unwrap_or(32);
    match Blake2bVar::new(len as usize) {
        Ok(mut hasher) => {
            hasher.update(to_hash);
            //write hash to buffer
            let mut buf: Vec<u8> = vec![0; len as usize];
            if let Err(_) = hasher.finalize_variable(&mut buf) {
                return Err(ErrCode::Blake2("Error writing to buffer"));
            }
            return Ok(buf);
        }
        Err(_) => {
            return Err(ErrCode::Blake2("Error allocating hasher"));
        }
    }
}
