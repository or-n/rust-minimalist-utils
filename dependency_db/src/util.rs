use eat::*;

pub struct U32(pub u32);

impl Eat<&[u8], (), ()> for U32 {
    fn eat(i: &[u8], _data: ()) -> Result<(&[u8], Self), ()> {
        let (i, b0) = u8::eat(i, ())?;
        let (i, b1) = u8::eat(i, ())?;
        let (i, b2) = u8::eat(i, ())?;
        let (i, b3) = u8::eat(i, ())?;
        let x = u32::from_be_bytes([b0, b1, b2, b3]);
        Ok((i, U32(x)))
    }
}

pub struct SeqN<T>(pub Vec<T>);

impl<'a, T> Eat<&'a [u8], (), ()> for SeqN<T>
where
    T: Eat<&'a [u8], (), ()>,
{
    fn eat(i: &'a [u8], _data: ()) -> Result<(&'a [u8], Self), ()> {
        let (mut i, n) = U32::eat(i, ())?;
        let mut r = Vec::new();
        for _ in 0..n.0 {
            let (new_i, x) = T::eat(i, ())?;
            i = new_i;
            r.push(x);
        }
        Ok((i, SeqN(r)))
    }
}
