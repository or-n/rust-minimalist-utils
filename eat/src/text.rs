use super::eat::*;

#[derive(Debug)]
pub enum CharError {
    Missing,
    Mismatch { expected: char, got: char },
}

impl Eat<&str, (), ()> for char {
    fn eat(i: &str, _data: ()) -> Result<(&str, char), ()> {
        let c = i.chars().next().ok_or(())?;
        Ok((&i[c.len_utf8()..], c))
    }
}

impl Drop<&str, ()> for char {
    fn drop(self, i: &str) -> Result<&str, ()> {
        let c = i.chars().next().ok_or(())?;
        if c != self {
            return Err(());
        }
        Ok(&i[c.len_utf8()..])
    }
}

impl<'b> Drop<&str, ()> for &'b str {
    fn drop(self, mut i: &str) -> Result<&str, ()> {
        for c in self.chars() {
            i = c.drop(i)?;
        }
        Ok(i)
    }
}
