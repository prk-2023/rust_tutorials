The program is a **generic in-memory cache**, its purpose is to store and retrieve the results of expensive
computations based on key.

Program defines a caching system using **Traits** and **generics** and handles various types of
**Key::Value** computations:

1. The `Cacheable` Trait: 

This trait defines the contract for any type that can be cached. It uses associated types to specify:
    - `Key`: The type used to lookup a value must be comparable and hashable i.e `Eq + Hash + Clone`.
    - `Value`: The computed result type.
    - `compute(key)`: A static function that defines **how to calculate** the `Value` from a `Key`.

2. Struct `Cache`: 

Its the core of the system:
    - Its a generic over a type `T` that implements Cacheable ( struct Cache<T: Cacheable> ).
    - it Internally uses a `HashMap` (store: HashMap<T::Key, T::Value>) to store the cached results.
    - `new()` method to init a empty cache.
    - `get_or_compute()` this is primary function:
      * It checks internal `HashMap` for a given `Key`.
      * If the key is present (Cache Hit), it returns the stored reference to the `Value` wrapped in a
        `CacheResult::Hit`.
      * If the key is missing (Cache Miss), it calls the `T::compute()` func to calculate the value, inserts
        the new *key-value* pair into the HashMap, and returns the original key wrapped in a
        `CacheResult::Miss`.

3. The `SquareCalculator` Implementation

This is a concrete example demonstrating how to use the caching system.
    - It implements Cacheable where Key is `u32` and Value is `u64`.
    - Its compute function calculates the square of a `u32` and returns it as a `u64`.

4. The main Function Execution Flow

The `main` function executes the following steps:
    - **Initialization**: Creates a new Cache specifically for the
      `SquareCalculator` logic: `let mut cache = Cache::<SquareCalculator>::new();.

    - Input: Defines a sequence of keys: vec![2, 3, 2].
    - Loop & Caching: It iterates through the keys, calling cache.get_or_compute(key) for each one:
    - **Key 2 (First time)**: It's a **miss**. The program computes `$2^2=4$`, stores `(2, 4)` in the cache,
      and prints a "Cache miss" message.
    - Key 3: It's a miss. The program computes `$3^2=9$`, stores `(3, 9)` in the cache, and prints a 
      "Cache miss" message.

    - Key 2 (Second time): It's a hit. The program retrieves 4 from the cache without recalculation and 
      prints a "Cache hit" message.Final Check: It uses the cache.get(&2) method to check the final state of
      the cache for key 2 and prints the stored value (4).

The program demonstrates **memoization** caching the result of a calculation ( `SquareCalculator::compute` )
to avoid recomputing it if the same input (`Key`) is seen again.


------------------------------------------------------------------

The program is a clean design demonstrating how to use traits, generics, and enums to build a cache in Rust.

Here's a quick summary and a few thoughts on your example:

---

### What code demonstrates well:

* **Trait with associated types:** 
    `Cacheable` defines `Key` and `Value` with required traits and a computation function.

* **CacheResult enum:** 
    Differentiates cache hits and misses clearly, carrying values or keys appropriately.

* **Generics with constraints:** 
    Proper bounds on `Key` (`Eq + Hash + Clone`) and `Value` (`Clone`) enable caching logic without 
    ownership issues.

* **PhantomData usage:** 
    Marks the type parameter `T` for the cache without storing an actual value.

* **Safe borrowing and ownership:** 
    `get_or_compute` returns owned values (cloned) to avoid dangling references.

* **Reusability:** 
    The `Cache` can work with any `Cacheable` type.

* **Utility function:** 
    `print_cache_result` cleanly formats output.

* **Concrete implementation:** 
    SquareCalculator` nicely demonstrates usage with a simple squared number computation.

---

### Just a few points/ideas:

1. **Cloning values:**

   * The design clones values both when returning hits and when inserting computed values. 
     This is necessary because the `CacheResult` returns owned values, not references, avoiding lifetime 
     issues.

   * For large values, cloning could be expensive. One could return references instead with careful lifetime 
     management, but that complicates the API.

2. **Return type of `get_or_compute`:**

   * Returning owned values (`CacheResult<T::Value, T::Key>`) makes the API simple to use but less efficient
     if cloning is expensive.

   * If you want to optimize, you could consider returning references (`&T::Value`) for hits and owned keys 
     for misses, but this requires juggling lifetimes carefully.

3. **Cache eviction / size limits:**

   * Your cache grows indefinitely as there is no eviction. This is fine for demonstration, but production 
     caches often need max size or TTL.

4. **Thread safety:**

   * Your cache is not thread-safe (`HashMap` not synchronized). For concurrency, you might want to wrap it 
     in a `Mutex` or use a concurrent map crate.

---

### Here’s a concise explanation:

```rust
// Cacheable trait defines key-value computation contract.
pub trait Cacheable {
    type Key: Eq + Hash + Clone;
    type Value: Clone;
    fn compute(key: &Self::Key) -> Self::Value;
}

// CacheResult enum captures hit or miss outcomes.
pub enum CacheResult<V, K> {
    Hit(V),
    Miss(K),
}

// Cache stores computed values and returns CacheResults.
pub struct Cache<T: Cacheable> {
    store: HashMap<T::Key, T::Value>,
    _marker: PhantomData<T>,
}

impl<T: Cacheable> Cache<T> {
    pub fn new() -> Self { ... }

    pub fn get_or_compute(&mut self, key: T::Key) -> CacheResult<T::Value, T::Key> { ... }

    pub fn get(&self, key: &T::Key) -> Option<&T::Value> { ... }
}
```

---

If you want, I can help you evolve this example to support:

* Returning references for hits (with lifetimes).
* Adding cache eviction strategies.
* Making it thread-safe.
* More complex `Cacheable` implementations.

Just let me know! Otherwise, this example looks great as a teaching tool for generic cache design in Rust.
