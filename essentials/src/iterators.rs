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