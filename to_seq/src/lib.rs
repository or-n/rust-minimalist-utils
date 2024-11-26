pub trait ToSeq<Output, Data, Error> {
    fn to_seq(self, o: Output, data: Data) -> Result<(), Error>;
}

impl ToSeq<&mut Vec<u8>, (), ()> for &[u8] {
    fn to_seq(self, o: &mut Vec<u8>, _data: ()) -> Result<(), ()> {
        o.extend_from_slice(self);
        Ok(())
    }
}

impl ToSeq<&mut Vec<u8>, (), ()> for u32 {
    fn to_seq(self, o: &mut Vec<u8>, _data: ()) -> Result<(), ()> {
        self.to_be_bytes().to_seq(o, ())?;
        Ok(())
    }
}

impl ToSeq<&mut Vec<u8>, (), ()> for &Vec<u8> {
    fn to_seq(self, o: &mut Vec<u8>, _data: ()) -> Result<(), ()> {
        o.extend(self);
        Ok(())
    }
}
