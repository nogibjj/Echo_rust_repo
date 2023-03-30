# Vector

Vectors are resizable arrays. Like slices, their size is not known at compile time, but they can grow or shrink at any time.

Basic Operations
🌟🌟 
A Vec can be extended with extend method

Turn X Into Vec

🌟🌟🌟
Slicing

A Vec can be mutable. On the other hand, slices are read-only objects. To get a slice, use &.

In Rust, it’s more common to pass slices as arguments rather than vectors when you just want to provide read access. The same goes for String and &str.

🌟🌟

Indexing
```
fn main() {
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v[i])
    }

    for i in 0..5 {
       // IMPLEMENT the code here...
    }
    
    assert_eq!(v, vec![2, 3, 4, 5, 6]);

    println!("Success!");
}
```
🌟🌟🌟

Capacity

The capacity of a vector is the amount of space allocated for any future elements that will be added onto the vector. This is not to be confused with the length of a vector, which specifies the number of actual elements within the vector. If a vector’s length exceeds its capacity, its capacity will automatically be increased, but its elements will have to be reallocated.

For example, a vector with capacity 10 and length 0 would be an empty vector with space for 10 more elements. Pushing 10 or fewer elements onto the vector will not change its capacity or cause reallocation to occur. However, if the vector’s length is increased to 11, it will have to reallocate, which can be slow. For this reason, it is recommended to use Vec::with_capacity whenever possible to specify how big the vector is expected to get.

🌟🌟


Store distinct types in Vector

The elements in a vector must be the same type, for example , the code below will cause an error:

```
fn main() {
   let v = vec![1, 2.0, 3];
}
```
But we can use enums or trait objects to store distinct types.

🌟🌟
