# typevec

Rust Vec which can contain arbitrary types inrernally casts inputs to ``Box<UnsafeAny>``.

```rust
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
```