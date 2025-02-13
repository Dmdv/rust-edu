
fn collections() {
    let numbers = vec![1, 2, 3, 4, 5];

    for num in &numbers {
        println!("{}", num);
    }

    for num in numbers.iter() {
        println!("{}", num);
    }

    // The original `numbers` vector is still available after the iteration
    println!("{:?}", numbers); // Output: [1, 2, 3, 4, 5]

    // but when we use `into_iter`, the original vector is moved
}

fn lazy_evaluate() {
    let numbers = vec![1, 2, 3, 4, 5];

    let numbers_iter = numbers.iter().map(|n| {
        println!("Processing {}", n);
        n * n
    });

    // The `map` operation is lazy and won't be executed until we consume the iterator using `collect`
    // Collect into a Vec<i32>
    let squared_numbers: Vec<i32> = numbers_iter.collect();

    println!("Squared numbers: {:?}", squared_numbers);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collections() {
        collections();
        lazy_evaluate();
    }
}