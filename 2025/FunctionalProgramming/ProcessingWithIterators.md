# Processing Series of Items with Iterators:


## 1.0 Introduction
The iterator pattern allows you to perform some task on a sequence of items in turn. 

An iterator is responsible for the logic of iterating over each item and determining when the sequence has 
finished. When you use iterators, you don’t have to reimplement that logic yourself.

In Rust Iterators are lazy, meaning they have no effect until you call methods that consume the iterator to
use it up. Example: 

```rust 
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();
```
The method `iter()`  returns an iterator tat once created can be used in  many ways:
- Iterate over an array using a `for` loop to execute some code on each item.
```rust 
    fn main() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("Got: {val}");
        }
    }
```
Iterator handles all the logic for us cutting down on on repetitive code you could potentially mess up.

```rust 

    let v1 = vec![1, 2, 3, 4];

    let mut v1_iter = v1.iter(); //returns iterator over slice

    assert_eq!(v1_iter.next(), Some(&1));
```
- `v1_iter` has to be defined as mutable as `.next()` method mutates the state of the iterator.


## `Iterator` Trait and the `next` method:

- All iterators implement a trait named `Iterator` that is defined in the **standard library**. The
  definition of the trait looks as :

 ```rust 
    pub trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;

        // methods with default implementations elided
    }
 ```
- Notice that this definition uses some new syntax: `type Item` and `Self::Item`, which are defining an 
  associated type with this trait. ( Other topics covers associated type )

- The `Iterator` trait only requires implementors to define one method: the `next` method, which returns one
  item of the iterator at a time, wrapped in `Some`, and, when iteration is over, returns `None`.

```rust 
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}
```


