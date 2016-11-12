//! Rust Vec which can contain arbitrary types inrernally casts inputs to ``Box<UnsafeAny>``.

extern crate unsafe_any as uany;
use uany::{UnsafeAny, UnsafeAnyExt};
use std::any::Any;


pub unsafe trait AsAny<A: ?Sized + UnsafeAnyExt> {
    fn asany(self) -> Box<A>;
}

unsafe impl<T: UnsafeAny> AsAny<UnsafeAny> for T {
    fn asany(self) -> Box<UnsafeAny> { Box::new(self) }
}


pub struct TypeVec<A: ?Sized = UnsafeAny> where A: UnsafeAnyExt {
    pub data: Vec<Box<A>>,
}

/// Vec which can contain arbitrary types.
impl TypeVec {

    pub fn new() -> Self {
        TypeVec { data: vec![] }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        TypeVec { data: Vec::with_capacity(capacity) }
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional)
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional)
    }

    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.data.is_empty()
    }

}


impl<A: UnsafeAnyExt + ?Sized> TypeVec<A> {

    pub fn insert<T>(&mut self, index: usize, element: T)
        where T: Any + AsAny<A> {

        self.data.insert(index, element.asany());

    }

    pub fn get<T: Any>(&self, index: usize) -> Option<&T> {
        self.data.get(index).map(|v| unsafe {
            v.downcast_ref_unchecked::<T>()
        })
    }

    pub fn get_mut<T: Any>(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index).map(|mut v| unsafe {
            v.downcast_mut_unchecked::<T>()
        })
    }

    pub fn push<T>(&mut self, value: T)
        where T: Any + AsAny<A> {

        self.data.push(value.asany());
    }


    pub fn pop<T: Any>(&mut self) -> Option<T> {

        self.data.pop().map(move |v| unsafe {
            *v.downcast_unchecked::<T>()
        })
    }

}

#[cfg(test)]
mod tests {

    use super::TypeVec;

    #[test]
    fn test_readme() {
        // example on readme

        let mut v = TypeVec::new();
        assert!(v.is_empty());

        // Can push whatever values.
        v.push(1);
        v.push(2.2);
        v.push("xxx");
        v.push(vec![1, 2, 3]);

        // Give type hint to specify return type.
        assert_eq!(v.get::<i32>(0), Some(&1));
        assert_eq!(v.get::<f64>(1), Some(&2.2));
        assert_eq!(v.get::<&str>(2), Some(&"xxx"));
        assert_eq!(v.get::<Vec<i32>>(3), Some(&vec![1, 2, 3]));

        // get_mut
        assert_eq!(v.get_mut::<Vec<i32>>(3), Some(&mut vec![1, 2, 3]));

        // pop
        assert_eq!(v.pop::<Vec<i32>>(), Some(vec![1, 2, 3]));
        assert_eq!(v.pop::<&str>(), Some("xxx"));
        assert_eq!(v.pop::<f64>(), Some(2.2));
        assert_eq!(v.pop::<i32>(), Some(1));
    }

    #[test]
    fn test_capacity() {
        let mut v = TypeVec::with_capacity(50);
        assert_eq!(v.capacity(), 50);

        v.reserve(10);
        assert!(v.capacity() >= 50);

        let mut v2 = TypeVec::new();
        v2.push(1);

        v2.reserve_exact(100);
        assert!(v2.capacity() >= 101);

        v2.shrink_to_fit();
        assert!(v2.capacity() < 100);
    }

    #[test]
    fn test_numeric() {

        // Vec //
        let mut n = vec![];
        n.insert(0, 1);
        n.insert(1, 2);
        assert_eq!(n.get(0), Some(&1));
        assert_eq!(n.pop(), Some(2));

        // TypeVec //
        let mut v = TypeVec::new();

        assert!(v.is_empty());

        v.insert(0, 42);
        v.insert(1, 1.1);
        assert_eq!(v.get::<i32>(0), Some(&42));
        assert_eq!(v.get::<f64>(1), Some(&1.1));

        assert_eq!(v.get_mut::<i32>(0), Some(&mut 42));
        assert_eq!(v.get_mut::<f64>(1), Some(&mut 1.1));

        assert_eq!(v.pop::<f64>(), Some(1.1));
        assert!(!v.is_empty());

        assert_eq!(v.pop::<i32>(), Some(42));
        assert!(v.is_empty());
    }

    #[test]
    fn test_string() {
        let mut v = TypeVec::new();
        v.insert(0, 42);
        v.insert(1, "xxx");

        assert_eq!(v.len(), 2);

        assert_eq!(v.get::<i32>(0), Some(&42));
        assert_eq!(v.get::<&str>(1), Some(&"xxx"));

        v.push("yyy");

        assert_eq!(v.pop::<&str>(), Some("yyy"));
        assert_eq!(v.pop::<&str>(), Some("xxx"));
        assert_eq!(v.pop::<i32>(), Some(42));

        v.clear();
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_vec_mix() {
        let mut v = TypeVec::new();

        assert!(v.is_empty());

        v.insert(0, vec![1, 2, 3]);
        v.insert(1, vec![3.1, 3.2, 3.3]);
        v.push("xyz");

        assert_eq!(v.get::<Vec<i32>>(0), Some(&vec![1, 2, 3]));
        assert_eq!(v.get::<Vec<f64>>(1), Some(&vec![3.1, 3.2, 3.3]));
        assert_eq!(v.get::<&str>(2), Some(&"xyz"));

        assert_eq!(v.pop::<&str>(), Some("xyz"));
        assert_eq!(v.pop::<Vec<f64>>(), Some(vec![3.1, 3.2, 3.3]));
        assert_eq!(v.pop::<Vec<i32>>(), Some(vec![1, 2, 3]));
    }

    #[test]
    fn test_custom() {

        #[derive(Debug, PartialEq)]
        struct Vi32(i32);

        #[derive(Debug, PartialEq)]
        struct Vf64(f64);

        let mut v = TypeVec::new();
        v.insert(0, Vi32(42));
        v.insert(1, Vf64(1.1));

        assert_eq!(v.get::<Vi32>(0), Some(&Vi32(42)));
        assert_eq!(v.get::<Vf64>(1), Some(&Vf64(1.1)));

        assert_eq!(v.get_mut::<Vi32>(0), Some(&mut Vi32(42)));
        assert_eq!(v.get_mut::<Vf64>(1), Some(&mut Vf64(1.1)));

        assert_eq!(v.pop::<Vf64>(), Some(Vf64(1.1)));
        assert_eq!(v.len(), 1);
    }
}
