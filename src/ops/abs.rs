/// Computes the absolute value of `Self`.
pub trait Abs {
    type Output;

    /// Returns the absolute value of `self`.
    fn abs(self) -> Self::Output;
}

macro_rules! abs_impl {
    () => ();

    ($sign:ident : $usig:ident $($next:tt)*) => (
        abs_impl! { $sign }

        impl Abs for $usig {
            type Output = $usig;

            #[inline]
            #[must_use]
            fn abs(self) -> $usig {
                self
            }
        }

        abs_impl! { $($next)* }
    );

    ($sign:ident $($next:tt)*) => (
        impl Abs for $sign {
            type Output = $sign;

            #[inline]
            #[must_use]
            fn abs(self) -> $sign {
                self.abs()
            }
        }

        abs_impl! { $($next)* }
    );
}

#[cfg(has_i128)]
abs_impl! { i128:u128 }
abs_impl! { isize:usize i64:u64 i32:u32 i16:u16 i8:u8 f32 f64 }
