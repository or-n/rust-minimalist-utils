pub trait Spit<Output, Error> {
    fn spit(self, o: Output) -> Result<Output, Error>;
}

impl Spit<Vec<u8>, ()> for &[u8] {
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        o.extend_from_slice(self);
        Ok(o)
    }
}

impl Spit<Vec<u8>, ()> for u32 {
    fn spit(self, o: Vec<u8>) -> Result<Vec<u8>, ()> {
        self.to_be_bytes().spit(o)
    }
}

impl Spit<Vec<u8>, ()> for &Vec<u8> {
    fn spit(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        o.extend(self);
        Ok(o)
    }
}
