//! Rust Vec which can contain arbitrary types inrernally casts inputs to ``Box<UnsafeAny>``.

use uany::{UnsafeAny, UnsafeAnyExt};

pub unsafe trait AsAny<A: ?Sized + UnsafeAnyExt> {
    fn asany(self) -> Box<A>;
}

unsafe impl<T: UnsafeAny> AsAny<UnsafeAny> for T {
    fn asany(self) -> Box<UnsafeAny> {
        Box::new(self)
    }
}
