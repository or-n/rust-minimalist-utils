use super::Deps;
use crate::util::ToSeq;

impl ToSeq<&mut Vec<u8>, (), ()> for Deps<u32> {
    fn to_seq(self, o: &mut Vec<u8>, _data: ()) -> Result<(), ()> {
        for (key, value) in self.table {
            key.to_seq(o, ())?;
            let (file, deps) = value;
            (file.len() as u32).to_seq(o, ())?;
            file.to_seq(o, ())?;
            (deps.len() as u32).to_seq(o, ())?;
            for dep in deps {
                dep.to_seq(o, ())?;
            }
        }
        Ok(())
    }
}
