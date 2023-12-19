#[macro_export]
macro_rules! uninitialized {
    ($name: ident, $size: expr) => {
        let mut $name: [_; $size];
        unsafe { $name = std::mem::MaybeUninit::uninit().assume_init() }
    };
}
