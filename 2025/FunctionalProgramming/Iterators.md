# Iterators:

Rust Iterators are central to functional programming in Rust, and they make data processing more declarative
and expressive. 

Iterators allows you to perform some task on a sequence of items in turn.

## Key Concepts :

1. Iterator Trait:
    * **Iterator** is a type in rust, and it implements the `Iterator` trait, which defines a series of
      methods used to process items one at a time.

    * The `Iterator` requires the `next()` method which returns `Option<T>` Which means the Iterator yields
      an item `T` wrapped in `Some<T>` on each iteration and returns `None` when its finished.

    * All Iterators implement a trait names `Iterator` that is defined in the standard library. the
      definition of the trait looks as below:

    ```rust 
        pub trait Iterator {
            fn next(&mut self) -> Option<Self::Item>;
        }

    ```
    * Methods that call `next()` are called **consuming adapters** because calling them uses up the
      iterator.
    ```rust 
        #[cfg(test)]
        mod tests {
            #[test]
            fn iterator_sum() {
                let v1 = vec![1, 2, 3];

                let v1_iter = v1.iter();

                let total: i32 = v1_iter.sum();

                assert_eq!(total, 6);
            }
        }
    ```
    - The `sum` method, takes ownership of the iterator and iterates through the items by repeatedly calling
      next, thus consuming the iterator. As it iterates through, it adds each item to a running total and 
      returns the total when iteration is complete.
      
2. Iterator Methods: Rust provides many iterator methods that can be chained together to process data in a
   clean and efficient manner. Common traits that are implemented are:
   * `map()` : Applies a function to each element in the iterator.
   * `filter()`: Returns an iterator that only yields elements that satisfy the condition. 
   * `collect()`: Consumes the iterator and collects the items into a collection like `Vec`.
   * `for_each()`: Applies a Closure to each item in the iterator.
   * `fold()`: Accumulates a value by applying a Closure to each item.

3. Chaining Iterators:  One of the powerful feature of iterator in Rust is you can chain multiple iterator
   methods together to perform complex transformations and operations.

4. Laziness:  Iterators are Lazy in Rust.
   Computation on iterator is not actually performed until they are consumed. This allows efficient and
   on-demand processing.

5. Methods that produce other iterators: **Iterator Adapters** are methods defined on the `Iterator` trait
   that don't consume the iterator, instead they produce different iterators by changing some aspect of the
   original iterator.
   `map()`: it takes a closure to call on each item as the items are iterated through. `map()` method
   returns a new iterator that produces the modified items. Note the closure will only get called, and it
   has to be consume the iterator. ( since iterator adapters are lazy ). 
   And `map()` is generally chained with `collect()` method which consumes the iterator and collects the
   resultant values into  collection data type.

##  Basic Usages:

1. Basic Iterator Usage:  Use Iterator on a vector `Vec` and print over its elements:

```rust 
    fn main() {
        let numbers = vec![1,2,3,4,5];
        //Create an Iterator and iterate over the vector 
        let  mut iter = numbers.iter();

        while let Some(num) = iter.next() {
            println!("{}",num);
        }
    }
```
    - `number.iter()`: this generates an iterator over the Vector of numbers.
    - `next()`" 

2. Using `map()` to transform Items.
    The `map()` method allows us to apply a function to each iterm in a iterator.

```rust 
    fn main() {
        let numbers = vec![1,2,3,4,5];
        //Create an Iterator and iterate over the vector 
        let  doubled: Vec<i32> = numbers.iter().map(|&x| x*2 ).collect();
        println!("{:?}",doubled); //prints [1,4,6, 8, 10]
    }
```
    - `map(|&x| x * 2)` applies a closure to each element of the iterator, doubling the values.
    - `collect(0)` is used to consume the iterator and gather the results into a Vec<i32>.

3. Using `filter()` to Select Items :
    You can filter elements from an iterator based on a condition using the `filter()` method.

```rust 

    fn main() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        // Using `filter` to select only even numbers
        let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();

        println!("{:?}", evens); // Prints [2, 4, 6, 8, 10]
    }
```
    - `filter(|&&x| x % 2 == 0)` filters out the odd numbers.
    - The `&&` is used because `iter()` creates references to the elements, and we need to dereference them.

4. Using fold to Accumulate a Value:
    The `fold()` method is used to reduce an iterator to a single value by applying a function cumulatively.

```rust 
    fn main() {
        let numbers = vec![1, 2, 3, 4, 5];

        // Using `fold` to compute the sum of the numbers
        let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
        println!("Sum: {}", sum); // Prints 15
    }
```
    - `fold(0, |acc, &x| acc + x)` starts with an initial accumulator value of `0` and then applies the 
       closure to add each number to the accumulator.

5. Using `for` with Iterators:
    A more concise and idiomatic way to loop over an iterator in Rust is using a `for` loop.

```rust
    fn main() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        // Using a for loop to iterate over the elements
        for num in numbers.iter() {
            println!("{}", num); // Prints 1, 2, 3, 4, 5
        }
    }
```
    - This is equivalent to manually calling next(), but it's cleaner and more Rust-like.

6. Chaining Multiple Iterator Methods:
    Iterators in Rust can be chained together to perform multiple operations on the data.

```rust 
    fn main() {
        let numbers = vec![1, 2, 3, 4, 5];
        
        // Chaining map and filter to create a new iterator
        let result: Vec<i32> = numbers
            .iter()
            .filter(|&&x| x % 2 == 0) // filter even numbers
            .map(|&x| x * x)           // square each number
            .collect();
        
        println!("{:?}", result); // Prints [4, 16]
    }
```
    - `filter()` selects only the even numbers from the original list.
    - `map()` squares those even numbers.
    - `collect()` gathers the final results into a Vec<i32>.


Summing up:

- Iterators in Rust are powerful tools for processing data lazily and efficiently.

- Methods like `map()`, `filter()`, and `fold()` allow you to transform, filter, and reduce data in a
  declarative way.

- Iterators are lazy, meaning they don’t perform any computation until the items are actually consumed 
  (e.g., with collect() or for loops).

- Chaining multiple iterator methods allows you to build complex transformations in a concise and readable
  manner.

-----------------------------------
Book:
-----------------------------------

