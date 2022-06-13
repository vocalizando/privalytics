use sha2::{Sha256, Digest};

pub trait HashPath {
    fn hash_hex(&self) -> String;
}

impl HashPath for &str {
    fn hash_hex(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self);
        let result = hasher.finalize();

        hex::encode(result.as_slice())
    }
}
