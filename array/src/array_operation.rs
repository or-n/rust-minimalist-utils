
#[macro_export]
macro_rules! array_operation {
    ($size: expr, $f:expr, $pairs:expr) => {{
        uninitialized!(zs, $size);
        let mut index = 0;
        for (x, y) in $pairs {
            zs[index] = $f(x, y);
            index += 1;
        }
        zs
    }};
}

