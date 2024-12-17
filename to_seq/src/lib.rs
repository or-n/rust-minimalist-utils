pub trait ToSeq<Output, Data, Error> {
    fn to_seq(self, o: Output) -> Result<Output, Error>;
}

impl ToSeq<&mut Vec<u8>, (), ()> for &[u8] {
    fn to_seq(self, o: &mut Vec<u8>) -> Result<&mut Vec<u8>, ()> {
        o.extend_from_slice(self);
        Ok(o)
    }
}

impl ToSeq<&mut Vec<u8>, (), ()> for u32 {
    fn to_seq(self, o: &mut Vec<u8>) -> Result<&mut Vec<u8>, ()> {
        self.to_be_bytes().to_seq(o)?;
        Ok(o)
    }
}

impl ToSeq<&mut Vec<u8>, (), ()> for &Vec<u8> {
    fn to_seq(self, o: &mut Vec<u8>) -> Result<&mut Vec<u8>, ()> {
        o.extend(self);
        Ok(o)
    }
}
