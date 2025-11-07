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
The method `iter()`  returns an iterator that once created can be used in  many ways:
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
The above code consumes up the iterator, with each `next()` eats up an item from the iterator.

### Methods that Consume the iterator:

The `Iterator` trait has  number of different traits with default implementations provided by the standard
library API documentation for the `Iterator` trait. Some of these methods call the `next` method in their
definition, which is why you're required to implement the `next` method when implementing the `Iterator`
trait.

=> Methods that take ownership of the iterator and iterates through the items by repeatedly calling `next`,
thus consuming the iterator. As it iterates through, it adds each item to a running total and return the
total and return the total when iterator is complete:

Test Illustration that use  of the `sum` method.

```rust 
#[cfg(test)]
mod tests {
    #[test]
    fn iterator_sum() {
        let v1 = vec![1,2,3]

        let v1_ter = v1.iter();

        let total: i32 = v1_iter.sum()

        assert_eq!(totalm, 6);
    }
}
```
## comparing Performance: Loops vs Iterators:

The two implementations seems to have same performance.
WRT to the book a benchmarking test show the comparison to be identical:

Rust code using iterators compiles to same assembly if you would have written by hand. 
Optimizations such as loop unrolling and eliminating bounds checking on array access apply and make the 
resultant code extremely efficient. 

==> you can use iterators and closures without fear! They make code seem like it’s higher level but don’t 
    impose a runtime performance penalty for doing so.

In general: iterators, although a high-level abstraction, get compiled down to roughly the same code as if 
you’d written the lower-level code yourself. 

Iterators are one of Rust’s zero-cost abstractions, by which we mean that using the abstraction imposes no 
additional runtime overhead. 

## Summary:

Closures and iterators are Rust features inspired by Functional programming language ideas. They contribute
to Rust's capability to clearly express high-level ideas at low-level performance. 

The implementation of Closures and Iterators are such that runtime performance is not affected. This is part
of Rust's goal to strive to provide zero-cost abstraction.
