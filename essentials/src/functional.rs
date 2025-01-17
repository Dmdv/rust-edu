fn sum_of_square(input: &[i32]) -> i32 {
    input.iter().map(|x| x * x).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_square_with_repeated_elements() {
        let input = vec![2, 3, 2, 4, 3];
        let result = sum_of_square(&input);
        assert_eq!(result, 42); // 2^2 + 3^2 + 2^2 + 4^2 + 3^2 = 4 + 9 + 4 + 16 + 9 = 42
    }

    #[test]
    fn test_sum_of_square_empty_input() {
        let input: Vec<i32> = vec![];
        let result = sum_of_square(&input);
        assert_eq!(result, 0);
    }
}
