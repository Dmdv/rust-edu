//! A module demonstrating iterators in Rust
//! 
//! This module provides examples and explanations of iterators in Rust,
//! including custom iterator implementations and common iterator adapters.

/// Demonstrates the basic concept of iterators in Rust
/// 
/// Iterators in Rust are:
/// - Lazy by default (computation happens only when needed)
/// - Zero-cost abstractions
/// - Can be chained together
/// - Provide a consistent interface for working with collections
pub fn basic_iterator_example() {
    println!("\n=== Basic Iterator Example ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Basic iteration
    println!("Basic iteration:");
    for num in &numbers {
        println!("{}", num);
    }
    
    // Using iterator methods
    println!("\nUsing iterator methods:");
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    
    // Chaining iterator methods
    println!("\nChaining iterator methods:");
    let doubled_evens: Vec<i32> = numbers.iter()
        .filter(|&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("Doubled evens: {:?}", doubled_evens);
}

/// Demonstrates common iterator adapters
/// 
/// Iterator adapters are methods that:
/// - Transform one iterator into another
/// - Don't consume the iterator
/// - Can be chained together
pub fn iterator_adapters_example() {
    println!("\n=== Iterator Adapters Example ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Map: transform each element
    println!("Map example:");
    let squares: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squares: {:?}", squares);
    
    // Filter: keep elements that satisfy a condition
    println!("\nFilter example:");
    let evens: Vec<&i32> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);
    
    // Take: take first n elements
    println!("\nTake example:");
    let first_three: Vec<&i32> = numbers.iter().take(3).collect();
    println!("First three: {:?}", first_three);
    
    // Skip: skip first n elements
    println!("\nSkip example:");
    let after_two: Vec<&i32> = numbers.iter().skip(2).collect();
    println!("After two: {:?}", after_two);
}

/// Demonstrates iterator consumers
/// 
/// Iterator consumers are methods that:
/// - Consume the iterator
/// - Return a single value
/// - Can't be chained after
pub fn iterator_consumers_example() {
    println!("\n=== Iterator Consumers Example ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Sum: add up all elements
    println!("Sum example:");
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
    
    // Count: count number of elements
    println!("\nCount example:");
    let count = numbers.iter().count();
    println!("Count: {}", count);
    
    // Any: check if any element satisfies a condition
    println!("\nAny example:");
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    println!("Has even: {}", has_even);
    
    // All: check if all elements satisfy a condition
    println!("\nAll example:");
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("All positive: {}", all_positive);
}

/// Demonstrates custom iterator implementation
/// 
/// Custom iterators can be created by:
/// - Implementing the Iterator trait
/// - Defining the Item type
/// - Implementing the next() method
pub fn custom_iterator_example() {
    println!("\n=== Custom Iterator Example ===");
    
    // Using our custom Counter iterator
    let counter = Counter::new();
    println!("Counter values:");
    for i in counter {
        println!("{}", i);
    }
    
    // Using our custom Split iterator
    let text = "hello,world,rust";
    let split = Split::new(text, ',');
    println!("\nSplit values:");
    for part in split {
        println!("{}", part);
    }
}

/// Demonstrates advanced iterator concepts
/// 
/// Advanced iterator concepts include:
/// - Iterator adapters that maintain state
/// - Working with multiple iterators
/// - Error handling in iterators
pub fn advanced_iterator_example() {
    println!("\n=== Advanced Iterator Example ===");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let letters = vec!['a', 'b', 'c', 'd', 'e'];
    
    // Zip: combine two iterators
    println!("Zip example:");
    let zipped: Vec<(i32, char)> = numbers.iter()
        .zip(letters.iter())
        .map(|(&n, &c)| (n, c))
        .collect();
    println!("Zipped: {:?}", zipped);
    
    // Enumerate: get index with each element
    println!("\nEnumerate example:");
    for (i, &num) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, num);
    }
    
    // Chain: combine multiple iterators
    println!("\nChain example:");
    let chained: Vec<&i32> = numbers.iter()
        .chain(numbers.iter())
        .collect();
    println!("Chained: {:?}", chained);
}

/// Compares different iterator patterns and their use cases
pub fn iterator_patterns_comparison() {
    println!("\n=== Iterator Patterns Comparison ===");
    println!("Basic iteration vs. iterator methods:");
    println!("- Basic iteration: simple, familiar syntax");
    println!("- Iterator methods: more powerful, functional style");
    println!("\nIterator adapters vs. consumers:");
    println!("- Adapters: transform iterators, can be chained");
    println!("- Consumers: consume iterators, return single value");
    println!("\nWhen to use which:");
    println!("- Use basic iteration for simple loops");
    println!("- Use iterator methods for complex transformations");
    println!("- Use custom iterators for domain-specific iteration");
    println!("- Use advanced patterns for complex data processing");
}

/// Runs all iterator examples and comparisons
pub fn run_all_examples() {
    basic_iterator_example();
    iterator_adapters_example();
    iterator_consumers_example();
    custom_iterator_example();
    advanced_iterator_example();
    iterator_patterns_comparison();
}

pub struct Counter {
    count: u32,
}

pub struct Split<'input> {
    input: &'input str,
    delimiter: char,
}

impl Counter {
    pub fn new() -> Counter {
        Counter {count : 0}
    }
}

impl<'input> Split<'input> {
    pub fn new(input: &'input str, delimiter: char) -> Split<'input> {
        Split { input, delimiter }
    }
}

impl<'input> Iterator for Split<'input> {
    type Item = &'input str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        match self.input.find(self.delimiter) {
            Some(index) => {
                let result = &self.input[..index];
                self.input = &self.input[index + self.delimiter.len_utf8()..];
                Some(result)
            }
            None => {
                let result = self.input;
                self.input = "";
                Some(result)
            }
        }
    }
}

// Stateful iterator

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count = self.count + 1;
            Some(self.count)
        } else {
            None
        }
    }
}

struct MyCollection {
    items: Vec<i32>,
}

impl IntoIterator for MyCollection {
    type Item = i32;
    type IntoIter = std::vec::IntoIter<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

fn impl_iterator() {
    let my_collection = MyCollection { items: vec![1, 2, 3, 4, 5] };
    for item in my_collection {
        println!("{}", item);
    }
}

fn iterator_examples(s: &str) {
    let chars: Vec<char> = s.chars().collect();

    // 1. Map: transform each element
    let uppercase: String = chars.iter().map(|c| c.to_uppercase().next().unwrap()).collect();
    println!("Uppercase: {}", uppercase);

    // 2. Filter: keep elements that satisfy a condition
    let no_vowels: String = chars.iter().filter(|&c| !"aeiouAEIOU".contains(*c)).collect();
    println!("No vowels: {}", no_vowels);

    // 3. Enumerate: get index with each element
    for (i, c) in chars.iter().enumerate() {
        println!("Character {} at index {}", c, i);
    }

    // 4. Zip: combine two iterators
    let paired: Vec<(char, char)> = chars.iter().zip(chars.iter().rev()).map(|(&a, &b)| (a, b)).collect();
    println!("Paired: {:?}", paired);

    // 5. Fold: accumulate a result
    let char_count = chars.iter().fold(0, |acc, _| acc + 1);
    println!("Character count: {}", char_count);

    // 6. Any: check if any element satisfies a condition
    let has_uppercase = chars.iter().any(|c| c.is_uppercase());
    println!("Has uppercase: {}", has_uppercase);

    // 7. All: check if all elements satisfy a condition
    let all_letters = chars.iter().all(|c| c.is_alphabetic());
    println!("All letters: {}", all_letters);

    // 8. Take: take first n elements
    let first_five: String = chars.iter().take(5).collect();
    println!("First five: {}", first_five);

    // 9. Skip: skip first n elements
    let after_five: String = chars.iter().skip(5).collect();
    println!("After five: {}", after_five);

    // 10. Chain: combine multiple iterators
    let doubled: String = chars.iter().chain(chars.iter()).collect();
    println!("Doubled: {}", doubled);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_iterator() {
        let counter = Counter::new();
        for i in counter {
            println!("{}", i);
        }

        impl_iterator();

        iterator_examples("hello");
    }
}