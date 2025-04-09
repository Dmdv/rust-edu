//! A module demonstrating tree-like and dictionary data structures in Rust
//! 
//! This module provides examples and comparisons of various container types
//! from the Rust standard library, focusing on tree-based and dictionary-like
//! data structures.

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// Demonstrates the usage and characteristics of BTreeMap
/// 
/// BTreeMap is a map based on a B-tree, which provides:
/// - Ordered key-value pairs
/// - O(log n) search, insertion, and deletion
/// - Memory-efficient for large collections
pub fn btree_map_example() {
    println!("\n=== BTreeMap Example ===");
    
    // Create a new BTreeMap
    let mut btree_map = BTreeMap::new();
    
    // Insert some key-value pairs
    btree_map.insert(3, "three");
    btree_map.insert(1, "one");
    btree_map.insert(2, "two");
    
    // Keys are automatically sorted
    println!("BTreeMap contents (automatically sorted):");
    for (key, value) in &btree_map {
        println!("{}: {}", key, value);
    }
    
    // Demonstrate range queries
    println!("\nRange query (1..3):");
    for (key, value) in btree_map.range(1..3) {
        println!("{}: {}", key, value);
    }
}

/// Demonstrates the usage and characteristics of HashMap
/// 
/// HashMap is a hash-based map, which provides:
/// - Unordered key-value pairs
/// - O(1) average case for search, insertion, and deletion
/// - Faster than BTreeMap for most operations
pub fn hash_map_example() {
    println!("\n=== HashMap Example ===");
    
    // Create a new HashMap
    let mut hash_map = HashMap::new();
    
    // Insert some key-value pairs
    hash_map.insert("apple", 1);
    hash_map.insert("banana", 2);
    hash_map.insert("cherry", 3);
    
    // Keys are not ordered
    println!("HashMap contents (unordered):");
    for (key, value) in &hash_map {
        println!("{}: {}", key, value);
    }
    
    // Demonstrate fast lookups
    if let Some(value) = hash_map.get("banana") {
        println!("\nFound banana: {}", value);
    }
}

/// Demonstrates the usage and characteristics of BTreeSet
/// 
/// BTreeSet is a set based on a B-tree, which provides:
/// - Ordered unique elements
/// - O(log n) search, insertion, and deletion
/// - Memory-efficient for large collections
pub fn btree_set_example() {
    println!("\n=== BTreeSet Example ===");
    
    // Create a new BTreeSet
    let mut btree_set = BTreeSet::new();
    
    // Insert some values
    btree_set.insert(3);
    btree_set.insert(1);
    btree_set.insert(2);
    
    // Values are automatically sorted
    println!("BTreeSet contents (automatically sorted):");
    for value in &btree_set {
        println!("{}", value);
    }
}

/// Demonstrates the usage and characteristics of HashSet
/// 
/// HashSet is a hash-based set, which provides:
/// - Unordered unique elements
/// - O(1) average case for search, insertion, and deletion
/// - Faster than BTreeSet for most operations
pub fn hash_set_example() {
    println!("\n=== HashSet Example ===");
    
    // Create a new HashSet
    let mut hash_set = HashSet::new();
    
    // Insert some values
    hash_set.insert("apple");
    hash_set.insert("banana");
    hash_set.insert("cherry");
    
    // Values are not ordered
    println!("HashSet contents (unordered):");
    for value in &hash_set {
        println!("{}", value);
    }
}

/// Compares the performance characteristics of different container types
pub fn performance_comparison() {
    println!("\n=== Performance Comparison ===");
    println!("BTreeMap vs HashMap:");
    println!("- BTreeMap: O(log n) operations, ordered keys");
    println!("- HashMap: O(1) average case, unordered keys");
    println!("\nBTreeSet vs HashSet:");
    println!("- BTreeSet: O(log n) operations, ordered values");
    println!("- HashSet: O(1) average case, unordered values");
    println!("\nWhen to use which:");
    println!("- Use BTreeMap/BTreeSet when you need ordered elements");
    println!("- Use HashMap/HashSet when you need maximum performance");
    println!("- Use BTreeMap/BTreeSet when memory efficiency is important");
    println!("- Use HashMap/HashSet when you don't care about ordering");
}

/// Runs all examples and comparisons
pub fn run_all_examples() {
    btree_map_example();
    hash_map_example();
    btree_set_example();
    hash_set_example();
    performance_comparison();
} 