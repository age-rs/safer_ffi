#![allow(missing_debug_implementations)]

// Without `stabby`, use plain type aliases so that these resolve to
// `PhantomData` and satisfy the `repr_transparent_non_zst_fields` lint
// (rust-lang/rust#78586) when used inside `#[repr(transparent)]` structs.
#[cfg(not(feature = "stabby"))]
pub type PhantomCovariantLifetime<'lt> = ::core::marker::PhantomData<&'lt ()>;

#[cfg(not(feature = "stabby"))]
pub type PhantomInvariant<T> = ::core::marker::PhantomData<fn(&T) -> &T>;

// With `stabby`, keep the wrapper structs so `#[stabby::stabby]` can add
// `#[repr(C)]` for ABI stability guarantees.  The `repr(transparent)` structs
// that contain these types (`char_p_ref`, `NonNullMut`) are also `repr(C)`
// under stabby, so the lint does not fire.
#[cfg(feature = "stabby")]
#[stabby::stabby]
#[derive(Default, Clone, Copy)]
pub struct PhantomCovariantLifetime<'lt>(pub ::core::marker::PhantomData<&'lt ()>);

#[cfg(feature = "stabby")]
#[stabby::stabby]
pub struct PhantomInvariant<T: ?Sized>(pub ::core::marker::PhantomData<fn(&T) -> &T>);

#[cfg(feature = "stabby")]
impl<T: ?Sized> Default for PhantomInvariant<T> {
    #[inline]
    fn default() -> Self {
        Self(::core::marker::PhantomData)
    }
}

#[cfg(feature = "stabby")]
impl<T: ?Sized> Copy for PhantomInvariant<T> {}
#[cfg(feature = "stabby")]
impl<T: ?Sized> Clone for PhantomInvariant<T> {
    #[inline]
    fn clone(self: &'_ Self) -> Self {
        *self
    }
}
