/*
    LRU Cache logic implementation, just to exercise some concepts
    before the code, let's explain what exactly is an LRU Cache

    An LRU (least recently used) cache is a type of cache that maintains
    a limited number of elements. When the cache reaches its imposed limit,
    it removes the least recently used element to make room for the new element.

    This program has the following objective:
     - If the cache is full and a new element is added, the LRU element must be popped out.

    Common uses of a program like this:
     - Cache systems to enhance performance of search and data access.
*/

// -----------------------------------------------------------------------
// Here we import HashMap<K, V> to store cache values efficiently
// Also imports VecDeque<K> to maintain the access order
// VecDeque is a two-way structure, allowing us to remove and add new
// elements more quickly than using a standard Vec. This makes sense for an LRU Cache.
use std::collections::{HashMap, VecDeque};

// Define the LRUCache struct with generic types K for key and V for value
// It contains the cache's capacity, a HashMap to store the key-value pairs,
// and a VecDeque to maintain the access order of keys.
struct LRUCache<K, V> {
    capacity: usize,     // Maximum capacity of the cache
    map: HashMap<K, V>,  // HashMap to store the actual key-value pairs
    order: VecDeque<K>,  // VecDeque to track the access order of keys
}

// Implement the LRUCache struct for the given key (K) and value (V) types
impl<K: Clone + Eq + std::hash::Hash, V> LRUCache<K, V> {

    // Constructor for creating a new LRUCache instance
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,             // Set the cache's capacity
            map: HashMap::new(),  // Initialize the HashMap to store key-value pairs
            order: VecDeque::new(), // Initialize the VecDeque to track key access order
        }
    }

    // Get a value from the cache
    // If the key exists, move it to the most recently used position in the VecDeque
    // and return the value.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Remove the key from the VecDeque (to maintain recent access order)
            self.order.retain(|k| k != key);
            // Add the key to the back of the VecDeque to mark it as recently accessed
            self.order.push_back(key.clone());
            // Return the value associated with the key
            self.map.get(key)
        } else {
            None // If the key doesn't exist, return None
        }
    }

    // Insert a key-value pair into the cache
    pub fn put(&mut self, key: K, value: V) {
        // If the key already exists, we first remove it from the access order
        if self.map.contains_key(&key) {
            self.order.retain(|k| k != &key);
        } else if self.map.len() == self.capacity {
            // If the cache is full, remove the least recently used (LRU) element
            // by popping from the front of the VecDeque
            if let Some(oldest) = self.order.pop_front() {
                // Remove the oldest element from the HashMap as well
                self.map.remove(&oldest);
            }
        }

        // Add the new key to the back of the VecDeque (most recently used)
        self.order.push_back(key.clone());
        // Insert the new key-value pair into the HashMap
        self.map.insert(key, value);
    }
}

// Main function to demonstrate the LRUCache functionality
fn main() {
    let mut cache = LRUCache::new(2); // Create an LRU cache with a capacity of 2

    // Insert two elements into the cache
    cache.put(1, "A");
    cache.put(2, "B");

    // Retrieve and print the value for key 1 (expected output: "A")
    println!("{:?}", cache.get(&1)); // Some("A")

    // Insert a new element (this will evict the least recently used element, which is key 2)
    cache.put(3, "C");

    // Try to retrieve the evicted key (expected output: None)
    println!("{:?}", cache.get(&2)); // None

    // Retrieve and print the value for key 3 (expected output: "C")
    println!("{:?}", cache.get(&3)); // Some("C")
}
