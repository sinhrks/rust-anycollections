# anycollections

Rust ``Vec`` and ``HashMap`` which can contain arbitrary types inrernally casts inputs to ``Box<UnsafeAny>``.

## AnyVec

```rust
extern crate anycollections;
use anycollections::AnyVec;

let mut v = AnyVec::new();
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
```

## AnyHashMap

```rust
extern crate anycollections;
use anycollections::AnyHashMap;

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
```