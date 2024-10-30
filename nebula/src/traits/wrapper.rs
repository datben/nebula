pub trait ToSome: Sized {
    #[inline(always)]
    fn some(self) -> Option<Self> {
        Some(self)
    }
}

impl<T: Sized> ToSome for T {}
