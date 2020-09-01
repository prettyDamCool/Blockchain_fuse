pub trait Hashable {
    fn bytes (&self) -> Vec<u8>;

    fn hash (&self) -> Vec<u8> { //reffrence to function
        crypto_hash::digest(crypto_hash::Algorithm::SHA512, &self.bytes())
    }
}
