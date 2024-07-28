#[macro_export]
macro_rules! zip_output {
    ($d: ident, $x:ident, $method:ident) => {
        impl<T: $x<Output = T>> $x for $d<T> {
            type Output = Self;

            fn $method(self, other: Self) -> Self {
                self.zip(other, $x::$method)
            }
        }
    };
}

#[macro_export]
macro_rules! zip_mut {
    ($d:ident, $x:ident, $method:ident) => {
        impl<T: $x> $x for $d<T> {
            fn $method(&mut self, other: Self) {
                self.zip_mut(other, $x::$method)
            }
        }
    };
}

#[macro_export]
macro_rules! neg {
    ($d:ident) => {
        impl<T: Neg<Output = T>> Neg for $d<T> {
            type Output = Self;

            fn neg(self) -> Self {
                Self(self.0.map(T::neg))
            }
        }
    };
}

#[macro_export]
macro_rules! scale {
    ($d:ident) => {
        impl<T: Scale<Factor>, Factor: Copy> Scale<Factor> for $d<T> {
            fn scale(self, factor: Factor) -> Self {
                Self(self.0.map(|field| field.scale(factor)))
            }
        }
    };
}

#[macro_export]
macro_rules! fold_nonempty {
    ($d:ident) => {
        impl<T> FoldNonempty<T> for $d<T> {
            fn fold_nonempty(self, f: impl Fn(T, T) -> T) -> T {
                self.0.fold_nonempty(f)
            }
        }
    };
}

#[macro_export]
macro_rules! add_sub_mul_div_neg_scale_foldnonempty {
    ($d:ident) => {
        zip_output!($d, Add, add);
        zip_output!($d, Sub, sub);
        zip_output!($d, Mul, mul);
        zip_output!($d, Div, div);
        zip_mut!($d, AddAssign, add_assign);
        zip_mut!($d, SubAssign, sub_assign);
        zip_mut!($d, MulAssign, mul_assign);
        zip_mut!($d, DivAssign, div_assign);
        neg!($d);
        scale!($d);
        fold_nonempty!($d);
    };
}

#[macro_export]
macro_rules! constant {
    ($d: ident, $n: expr, $x:ident, $method:ident) => {
        impl<T: $x + Copy> $x for $d<T> {
            fn $method() -> Self {
                Self([T::$method(); $n])
            }
        }
    };
}

#[macro_export]
macro_rules! zero_one_minvalue_maxvalue {
    ($d: ident, $n: expr) => {
        constant!($d, $n, Zero, zero);
        constant!($d, $n, One, one);
        constant!($d, $n, MinValue, min_value);
        constant!($d, $n, MaxValue, max_value);
    };
}

#[macro_export]
macro_rules! deref {
    ($d: ident, $n: expr) => {
        impl<T> Deref for $d<T> {
            type Target = [T; $n];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> DerefMut for $d<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}
