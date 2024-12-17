pub trait ToSeq<Output, Error> {
    fn to_seq(self, o: Output) -> Result<Output, Error>;
}

impl ToSeq<Vec<u8>, ()> for &[u8] {
    fn to_seq(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        o.extend_from_slice(self);
        Ok(o)
    }
}

impl ToSeq<Vec<u8>, ()> for u32 {
    fn to_seq(self, o: Vec<u8>) -> Result<Vec<u8>, ()> {
        self.to_be_bytes().to_seq(o)
    }
}

impl ToSeq<Vec<u8>, ()> for &Vec<u8> {
    fn to_seq(self, mut o: Vec<u8>) -> Result<Vec<u8>, ()> {
        o.extend(self);
        Ok(o)
    }
}
