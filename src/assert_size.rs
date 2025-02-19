/// Asserts that types are equal in size.
///
/// When performing operations such as pointer casts or dealing with [`usize`]
/// versus [`u64`] versus [`u32`], the size of your types matter. That is where
/// this macro comes into play.
///
/// # Alternatives
///
/// There also exist [`assert_size_eq_val`](macro.assert_size_eq_val.html) and
/// [`assert_size_eq_ptr`](macro.assert_size_eq_ptr.html). Instead of specifying
/// types to compare, values' sizes can be directly compared against each other.
///
/// # Examples
///
/// These three types, despite being very different, all have the same size:
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_size_eq!([u8; 4], (u16, u16), u32);
/// ```
///
/// The following example fails to compile because `u32` has 4 times the size of
/// `u8`:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// assert_size_eq!(u32, u8);
/// ```
///
/// [`usize`]: https://doc.rust-lang.org/std/primitive.usize.html
/// [`u64`]: https://doc.rust-lang.org/std/primitive.u64.html
/// [`u32`]: https://doc.rust-lang.org/std/primitive.u32.html
#[macro_export]
macro_rules! assert_size_eq {
    ($x:ty, $($xs:ty),+ $(,)?) => {
        const _: fn() = || {
            $(let _ = $crate::_core::mem::transmute::<$x, $xs>;)+
        };
    };
}

/// Asserts that types are equal in size.
///
/// This macro has been deprecated in favor of
/// [`assert_size_eq!`](macro.assert_size_eq.html).
#[deprecated(
    since = "1.2.0",
    note = "Please use the 'assert_size_eq' macro instead"
)]
#[macro_export(local_inner_macros)]
macro_rules! assert_eq_size {
    ($($t:tt)*) => {
        assert_size_eq!($($t)*);
    };
}

/// Asserts that values pointed to are equal in size.
///
/// # Examples
///
/// This especially is useful when coercing between pointers to different types
/// and ensuring the underlying values have the same size.
///
/// ```
/// # #[macro_use] extern crate static_assertions; fn main() {}
/// fn operation(x: &(u32, u32), y: &[u16; 4]) {
///     assert_size_eq_ptr!(x, y);
///     // ...
/// }
/// ```
///
/// The following example fails to compile because byte arrays of different
/// lengths have different sizes:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// static BYTES: &[u8; 4] = &[
///     /* ... */
///     # 0; 4
/// ];
///
/// static TABLE: &[u8; 16] = &[
///     /* ... */
///     # 0; 16
/// ];
///
/// assert_size_eq_ptr!(BYTES, TABLE);
/// ```
#[macro_export]
macro_rules! assert_size_eq_ptr {
    ($x:expr, $($xs:expr),+ $(,)?) => {
        #[allow(unknown_lints, unsafe_code, forget_copy, useless_transmute)]
        let _ = || unsafe {
            use $crate::_core::{mem, ptr};
            let mut copy = ptr::read($x);
            $(ptr::write(&mut copy, mem::transmute(ptr::read($xs)));)+
            mem::forget(copy);
        };
    }
}

/// Asserts that values pointed to are equal in size.
///
/// This macro has been deprecated in favor of
/// [`assert_size_eq_ptr!`](macro.assert_size_eq_ptr.html).
#[deprecated(
    since = "1.2.0",
    note = "Please use the 'assert_size_eq_ptr' macro instead"
)]
#[macro_export(local_inner_macros)]
macro_rules! assert_eq_size_ptr {
    ($($t:tt)*) => {
        assert_size_eq_ptr!($($t)*);
    };
}

/// Asserts that values are equal in size.
///
/// This macro doesn't consume its arguments and thus works for
/// non-[`Clone`]able values.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// struct Byte(u8);
///
/// let x = 10u8;
/// let y = Byte(42); // Works for non-cloneable types
///
/// assert_size_eq_val!(x, y);
/// assert_size_eq_val!(x, y, 0u8);
/// # }
/// ```
///
/// Even though both values are 0, they are of types with different sizes:
///
/// ```compile_fail
/// # #[macro_use] extern crate static_assertions;
/// # fn main() {
/// assert_size_eq_val!(0u8, 0u32);
/// # }
/// ```
///
/// [`Clone`]: https://doc.rust-lang.org/std/clone/trait.Clone.html
#[macro_export(local_inner_macros)]
macro_rules! assert_size_eq_val {
    ($x:expr, $($xs:expr),+ $(,)?) => {
        assert_size_eq_ptr!(&$x, $(&$xs),+);
    }
}

/// Asserts that values are equal in size.
///
/// This macro has been deprecated in favor of
/// [`assert_size_eq_val!`](macro.assert_size_eq_val.html).
#[deprecated(
    since = "1.2.0",
    note = "Please use the 'assert_size_eq_val' macro instead"
)]
#[macro_export(local_inner_macros)]
macro_rules! assert_eq_size_val {
    ($($t:tt)*) => {
        assert_size_eq_val!($($t)*);
    };
}
