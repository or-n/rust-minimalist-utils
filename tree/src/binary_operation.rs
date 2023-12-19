pub trait BinaryOperation<R> {
    fn operate(self, value: &mut R);
}
