//! A helper trait to convert values to `usize`.
//!
//! This is most useful for code that works with array indices. The trait allows
//! to pass anything into the function that can be resolved to a `usize`.

/// A trait used to convert a type into a `usize`.
pub trait ToUsize {
    /// Converts self into `usize`.
    fn to_usize(&self) -> usize;
}

impl ToUsize for u8 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl ToUsize for u16 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

// 16bit platforms use a `usize` of 16bit. Converting using a cast would fail on
// those systems.
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
impl ToUsize for u32 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

// 32bit platforms use a `usize` of 32bit. Converting using a cast would fail on
// those systems.
#[cfg(target_pointer_width = "64")]
impl ToUsize for u64 {
    #[inline]
    fn to_usize(&self) -> usize {
        *self as usize
    }
}

impl ToUsize for usize {
    #[inline]
    fn to_usize(&self) -> usize {
        *self
    }
}
