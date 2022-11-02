use blake2::digest::{Update, VariableOutput};
use blake2::Blake2bVar;

pub fn getBlake2Hash(toHash: &[u8], length: Option<u32>) -> Vec<u8> {
    let len = length.unwrap_or(32);
    let mut hasher = Blake2bVar::new(len as usize).unwrap();
    hasher.update(toHash);
    //write hash to buffer
    let mut buf: Vec<u8> = vec![0; len as usize];
    hasher.finalize_variable(&mut buf).unwrap();
    buf
}
