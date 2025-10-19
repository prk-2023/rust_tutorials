fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
////

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result}");
}

//-----------------------------------------------
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

// ---------- Trait with Associated Type ----------
pub trait Cacheable {
    type Key: Eq + Hash;
    type Value;

    fn compute(key: &Self::Key) -> Self::Value;
}

// ---------- Enum with Generics ----------
#[derive(Debug)]
pub enum CacheResult<V, E> {
    Hit(V),
    Miss(E),
}

impl<V, E> CacheResult<V, E> {
    pub fn is_hit(&self) -> bool {
        matches!(self, CacheResult::Hit(_))
    }

    pub fn unwrap(self) -> V {
        match self {
            CacheResult::Hit(v) => v,
            CacheResult::Miss(_) => panic!("Tried to unwrap a Miss"),
        }
    }
}

// ---------- Struct with Generic Types ----------
pub struct Cache<T: Cacheable> {
    store: HashMap<T::Key, T::Value>,
    _marker: PhantomData<T>,
}

impl<T: Cacheable> Cache<T> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn get_or_compute(&mut self, key: T::Key) -> CacheResult<&T::Value, &T::Key> {
        if self.store.contains_key(&key) {
            CacheResult::Hit(self.store.get(&key).unwrap())
        } else {
            let value = T::compute(&key);
            self.store.insert(key.clone(), value);
            CacheResult::Miss(&key)
        }
    }

    pub fn get(&self, key: &T::Key) -> Option<&T::Value> {
        self.store.get(key)
    }
}

// ---------- Implement Cacheable for a concrete type ----------

```
struct SquareCalculator;

impl Cacheable for SquareCalculator {
    type Key = u32;
    type Value = u64;

    fn compute(key: &Self::Key) -> Self::Value {
        (*key as u64) * (*key as u64)
    }
}

// ---------- Generic function using type parameters ----------
fn print_cache_result<K, V>(key: K, result: CacheResult<V, &K>)
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    match result {
        CacheResult::Hit(value) => println!("Cache hit: {:?} => {:?}", key, value),
        CacheResult::Miss(k) => println!("Cache miss for key: {:?}", k),
    }
}

// ---------- Main usage ----------
fn main() {
    let mut cache = Cache::<SquareCalculator>::new();

    let keys = vec![2, 3, 2];

    for key in keys {
        let result = cache.get_or_compute(key);
        print_cache_result(key, result);
    }

    println!("Final cache state: {:?}", cache.get(&2));
}

````
