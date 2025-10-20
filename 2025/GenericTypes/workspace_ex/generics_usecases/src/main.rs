/*
    Cache Tutorial in Rust

    This program demonstrates how to build a simple generic caching mechanism
    using Rust traits, enums, and generics.

    Overview:
    - The `Cacheable` trait defines a contract for types that can compute values from keys.
      It requires associated types `Key` and `Value` and a method `compute` to generate values.

    - The `Cache` struct uses a `HashMap` internally to store computed values associated with keys.

    - The `CacheResult` enum represents whether a cache lookup was a hit (value found) or miss (value not found).

    - When a key is queried with `get_or_compute`, if it exists in the cache, the cached value is returned as a Hit.
      Otherwise, the value is computed, stored in the cache, and a Miss is returned with the original key.

    - This design shows:
      * How to use traits with associated types and trait bounds,
      * How to use generics and enums for flexible APIs,
      * The use of PhantomData to tie generics without storing data,
      * Basic caching logic and result reporting.

    Example:
    - The program implements `SquareCalculator` which computes squares of numbers.
    - The main function queries the cache multiple times and prints whether each query was a hit or miss.

    This example is a foundation for more complex caching and memoization patterns in Rust.
*/

use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

//Step 1: Define a `Cacheable` trait:
/* Trait defining cacheable types.
 * Requires a key type (must be Eq, Hash, Clone) and a value type.
 * Also requires a compute method to generate a value from a key.
 */

pub trait Cacheable {
    type Key: Eq + Hash + Clone;
    type Value: Clone;

    fn compute(key: &Self::Key) -> Self::Value;
}

/* Step 2:
 * Represents the result of querying the cache.
 * Either a Hit containing the cached value or a Miss containing the key.
 */
#[derive(Debug)]
pub enum CacheResult<V, K> {
    Hit(V),
    Miss(K),
}

impl<V, E> CacheResult<V, E> {
    /// Returns true if this is a Hit.
    pub fn is_hit(&self) -> bool {
        matches!(self, CacheResult::Hit(_))
    }

    /// Unwraps the Hit value or panics if Miss.
    pub fn unwrap(self) -> V {
        match self {
            CacheResult::Hit(v) => v,
            CacheResult::Miss(_) => panic!("Tried to unwrap a Miss"),
        }
    }
}

/* Step 3: Define `Cache` Struct and method `new`
 * Generic cache storing values of types implementing Cacheable.
 * Uses a HashMap internally to store cached key-value pairs.
 * PhantomData marks the association with the Cacheable type.
 */
//use std::collections::HashMap;
//use std::marker::PhantomData;
pub struct Cache<T: Cacheable> {
    store: HashMap<T::Key, T::Value>,
    _marker: PhantomData<T>,
}

impl<T: Cacheable> Cache<T> {
    /// Creates a new empty cache.
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            _marker: PhantomData,
        }
    }
}

/* Step 4: Implement `get_or_compute` */

impl<T: Cacheable> Cache<T> {
    /// get a reference to the value for the key if cached (hit),
    /// or compute, store, and return miss with the key.
    pub fn get_or_compute(&mut self, key: T::Key) -> CacheResult<T::Value, T::Key> {
        if let Some(value) = self.store.get(&key) {
            CacheResult::Hit(value.clone())
        } else {
            // Value missing : compute and insert
            let value = T::compute(&key);
            self.store.insert(key.clone(), value.clone());

            //Return Miss with the key ( by the value )
            CacheResult::Miss(key)
        }
    }
}

/* Step 5: Implement `get` */
impl<T: Cacheable> Cache<T> {
    /// Returns a reference to the cached value for a key, if present.
    pub fn get(&self, key: &T::Key) -> Option<&T::Value> {
        self.store.get(key)
    }
}

/* Step 6: Implement `Cacheavle` for concrete type (`SquareCalculator`)
 * A simple calculator that squares numbers.
 */
struct SquareCalculator;

impl Cacheable for SquareCalculator {
    type Key = u32;
    type Value = u64;

    fn compute(key: &Self::Key) -> Self::Value {
        (*key as u64) * (*key as u64)
    }
}

/* step 7: Implement `print_cache_result`
 * Generic function to print cache results.
 * Uses Debug to display key and value.
 */
fn print_cache_result<K, V>(key: K, result: CacheResult<V, K>)
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    match result {
        CacheResult::Hit(value) => println!("Cache hit: {:?} => {:?}", key, value),
        CacheResult::Miss(k) => println!("Cache miss for key: {:?}", k),
    }
}

/* Step 8: The `main` function to test everything:
 */

fn main() {
    let mut cache = Cache::<SquareCalculator>::new();

    let keys = vec![2, 3, 2];

    for key in keys {
        let result = cache.get_or_compute(key);
        print_cache_result(key, result);
    }

    println!("Final cache state: {:?}", cache.get(&2));
}
