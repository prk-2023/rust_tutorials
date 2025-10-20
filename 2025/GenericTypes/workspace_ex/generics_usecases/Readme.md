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
