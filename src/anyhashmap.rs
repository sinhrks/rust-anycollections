//! Rust HashMap which can contain arbitrary types inrernally casts inputs to ``Box<UnsafeAny>``.

use uany::{UnsafeAny, UnsafeAnyExt};
use std::any::Any;

use std::collections::HashMap;
use std::hash::Hash;

use cast::AsAny;

pub struct AnyHashMap<K: Hash + Eq, A: ?Sized = UnsafeAny> where A: UnsafeAnyExt {
    pub data: HashMap<K, Box<A>>,
}

/// HashMap which can contain arbitrary types.
impl<K: Hash + Eq> AnyHashMap<K> {

    pub fn new() -> Self {
        AnyHashMap { data: HashMap::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        AnyHashMap { data: HashMap::with_capacity(capacity) }
    }

    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional)
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

impl<K: Hash + Eq, A: UnsafeAnyExt + ?Sized> AnyHashMap<K, A> {

    pub fn get<V>(&self, k: &K) -> Option<&V> where V: Any + AsAny<A> {
        self.data.get(k).map(|v| unsafe {
            v.downcast_ref_unchecked::<V>()
        })
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.data.contains_key(k)
    }

    pub fn get_mut<V>(&mut self, k: &K) -> Option<&mut V> where V: Any + AsAny<A> {
        self.data.get_mut(k).map(|mut v| unsafe {
            v.downcast_mut_unchecked::<V>()
        })
    }

    pub fn insert<V>(&mut self, k: K, v: V) -> Option<V> where V: Any + AsAny<A> {
        self.data.insert(k, v.asany()).map(move |v| unsafe {
            *v.downcast_unchecked::<V>()
        })
    }
}

#[cfg(test)]
mod tests {

    use super::AnyHashMap;

    #[test]
    fn test_readme() {
        // example on readme

        // Only to specify key type to create AnyHashMap
        let mut m: AnyHashMap<i32> = AnyHashMap::new();
        m.insert(10, 10);
        m.insert(20, 22.2);
        m.insert(30, "xxx");
        m.insert(40, vec![1, 2, 3]);

        // Give type hint to specify return type.
        assert_eq!(m.get::<i32>(&10), Some(&10));
        assert_eq!(m.get::<f64>(&20), Some(&22.2));
        assert_eq!(m.get::<&str>(&30), Some(&"xxx"));
        assert_eq!(m.get::<Vec<i32>>(&40), Some(&vec![1, 2, 3]));

        assert_eq!(m.get_mut::<i32>(&10), Some(&mut 10));
        assert_eq!(m.get_mut::<f64>(&20), Some(&mut 22.2));
        assert_eq!(m.get_mut::<&str>(&30), Some(&mut "xxx"));
        assert_eq!(m.get_mut::<Vec<i32>>(&40), Some(&mut vec![1, 2, 3]));
    }

    #[test]
    fn test_capacity() {
        let mut m: AnyHashMap<i32> = AnyHashMap::with_capacity(50);
        assert!(m.capacity() >= 50);

        m.reserve(10);
        assert!(m.capacity() >= 50);

        m.insert(1, 1);

        m.shrink_to_fit();
        assert!(m.capacity() < 50);
    }

    #[test]
    fn test_numeric() {
        let mut m: AnyHashMap<i32> = AnyHashMap::new();
        let res = m.insert(1, 10);
        assert_eq!(res, None);

        let res = m.insert(2, 22.2);
        assert_eq!(res, None);

        assert!(m.contains_key(&1));
        assert!(m.contains_key(&2));
        assert!(!m.contains_key(&3));

        let res = m.insert(3, "xxx");
        assert_eq!(res, None);

        let res = m.insert(1, 11);
        assert_eq!(res, Some(10));

        assert_eq!(m.get::<i32>(&1), Some(&11));
        assert_eq!(m.get::<f64>(&2), Some(&22.2));
        assert_eq!(m.get::<&str>(&3), Some(&"xxx"));

        assert_eq!(m.get_mut::<i32>(&1), Some(&mut 11));
        assert_eq!(m.get_mut::<f64>(&2), Some(&mut 22.2));
        assert_eq!(m.get_mut::<&str>(&3), Some(&mut "xxx"));
    }
}
