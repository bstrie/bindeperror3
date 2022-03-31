use sha2::{Sha256, Digest};

fn main() {
    let mut hasher = Sha256::new();
    hasher.update(include_bytes!(env!("CARGO_BIN_FILE_MYBINDEP")));
    println!("{:?}", hasher.finalize());
}
