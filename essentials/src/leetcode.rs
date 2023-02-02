use std::borrow::Cow;

struct PrefixSum1;
struct PrefixSum2;
struct PrefixSum3;
struct PrefixSum4;

// Version 1: using for loop
impl PrefixSum1 {
    pub fn running_sum(nums: &Vec<i32>) -> Vec<i32> {
        let mut res:i32 = 0;
        let mut sums = vec!();
        for i in nums.iter() {
            res += i;
            sums.push(res);
        }
        return sums;
    }
}

// Version 2: using map
impl PrefixSum2 {
    pub fn running_sum(nums: &Vec<i32>) -> Vec<i32> {
        let mut pre_num = 0;
        nums.into_iter().map(|num| {
            pre_num += num;
            pre_num
        }).collect::<Vec<i32>>()
    }
}

// Version 3: using slice
impl PrefixSum3 {
    pub fn running_sum(nums: &[i32]) -> Vec<i32> {
        let mut pre_num = 0;
        nums.into_iter().map(|num| {
            pre_num += num;
            pre_num
        }).collect::<Vec<i32>>()
    }
}

// Version 4: using Cow
impl PrefixSum4 {
    pub fn running_sum<'a>(nums: &[i32]) -> Cow<'a, Vec<i32>> {
        let mut pre_num = 0;
        let res = nums.into_iter().map(|num| {
            pre_num += num;
            pre_num
        }).collect();

        Cow::Owned(res)
    }
}

pub fn run() {
    let nums = vec![1, 2, 3, 4];
    println!("{:?}", PrefixSum1::running_sum(&nums));
    println!("{:?}", PrefixSum2::running_sum(&nums));
    println!("{:?}", PrefixSum3::running_sum(&nums)); // coercing is used here https://doc.rust-lang.org/stable/reference/type-coercions.html#coercion-types
    println!("{:?}", PrefixSum4::running_sum(&nums));
}

// For passing vector as a reference to a function:
// https://stackoverflow.com/questions/42954008/how-to-pass-one-vec-to-multiple-functions-in-rust
// https://stackoverflow.com/questions/40006219/why-is-it-discouraged-to-accept-a-reference-to-a-string-string-vec-vec-o

// 1. One of the main reasons to use a String or a Vec is because they allow increasing or decreasing the capacity.
// However, when you accept an immutable reference, you cannot use any of those interesting methods on the Vec or String.
//
// 2. Accepting a &String, &Vec or &Box also requires the argument to be allocated on the heap before you can call the function.
// Accepting a &str allows a string literal (saved in the program data) and accepting a &[T] or &T allows a stack-allocated array or variable.
// Unnecessary allocation is a performance loss.
// This is usually exposed right away when you try to call these methods in a test or a main method:
//
// awesome_greeting(&String::from("Anna"));
// total_price(&vec![42, 13, 1337])
// is_even(&Box::new(42))

// 3. Another performance consideration is that &String, &Vec and &Box introduce an unnecessary layer of
// indirection as you have to dereference the &String to get a String and then perform a second dereference to end up at &str.


// Instead, you should accept a string slice (&str), a slice (&[T]), or just a reference (&T).
// A &String, &Vec<T> or &Box<T> will be automatically coerced (via deref coercion) to a &str, &[T] or &T, respectively.
