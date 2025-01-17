mod strings;
mod vectors;
mod structs;
mod enums;
mod traits;
mod generics;
mod leetcode;
mod lend_borrow;
mod iterators;
mod loops;
mod hash_map;
mod functional;
mod set;
// Just import for simplified tests
// use vectors::basics;

/// This is a Doc comment outside the function
fn main() {
    lend_borrow::run();
    leetcode::run();

    generics::functions();
    generics::generic_structs();

    traits::traits_example();

    enums::enums_run();
    enums::enums_run_methods();
    enums::enums_run_match();
    enums::enums_options();
    enums::out_of_bands();
    enums::result_example();

    vectors::basics();

    structs::test_structs();
    structs::test_tuple_struct();
    structs::distance();

    conditionals();
    casts();
    loops();
    matches();
    iflet();
    variables();
    borrows();
    factorial(10);
    strings();
    tuples();
    slices();
    arrays();
    output();
    triangle(5);
    fibonacci(3);
}

fn fibonacci(term: i32) -> i32 {
    match term {
        0 => 0,
        1 => 1,
        _ => fibonacci(term - 1) + fibonacci(term - 2),
    }
}

fn triangle(n: i32) {
    for i in 0..n {
        for _ in 0..i + 1 {
            print!("&");
        }
        println!("{}", "");
    }
}

fn borrows() {
    // Shared borrows
    let x = 10;
    let a = &x;
    println!("a: {}", a);

    // Mutable borrows
    let mut y = 10;
    let b = &mut y;
    println!("b: {}", b);
}

fn casts() {
    let a = 15;
    let b = (a as f64) / 2.0;
    println!("b: {}", b);
}

fn loops() {
    for i in 0..5 {
        println!("{}", i);
    }

    for (count, variable) in (7..10).enumerate() {
        println!("count = {}, variable = {}", count, variable);
    }

    let mut i = 4;

    loop {
        println!("i:{}", i);
        if i == 5 {
            break;
        }
        i = i + 1;
    }
}

fn factorial(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 0 {
        return 1;
    }

    let mut fac = 1;

    for i in 1..n + 1 {
        fac = fac * i;
    }

    return fac;
}

fn matches() {
    // define a variable
    let x = 5;
    // define match expression
    match x {
        1 => println!("Java"),
        2 => println!("Python"),
        3 => println!("C++"),
        4 => println!("C#"),
        5 => println!("Rust"),
        6 => println!("Kotlin"),
        _ => println!("Some other value"),
    };
}

fn iflet() {
    // define a scrutinee expression
    let course = ("Rust", "beginner", "course");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner", "course") = course {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
        // do not execute this block
        println!("Value unmatched");
    }

    // define a scrutinee expression
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner", c) = course {
        println!("Wrote first two values in pattern to be matched with the scrutinee expression : {}", c);
    } else {
        // do not execute this block
        println!("Value unmatched");
    }
}

fn conditionals() {
    //define a variable
    let learn_language = "Rust";
    // if..elseif..else construct
    if learn_language == "Rust" {
        println!("You are learning Rust language!");
    } else if learn_language == "Java" {
        println!("You are learning Java language!");
    } else {
        println!("You are learning some other language!");
    }

    //define a variable
    let learn_language = "Rust";
    // short hand construct
    let res = if learn_language == "Rust" { "You are learning Rust language!" } else { "You are learning some other language!" };
    println!("{}", res);
}

fn tuples() {
    //define a tuple
    let _person_data = ("Alex", 48, "35kg", "6ft");
    // define a tuple with type annotated
    let person_data: (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");
    // accessing values
    println!("{}", person_data.0);

    let (w, x, y, z) = person_data;
    println!("{}", w);
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    //print the value of tuple
    println!("Tuple - Person Data : {:?}", person_data);

    // Tuples //

    // A tuple is a fixed-size set of values of possibly different types
    let x: (i32, &str, f64) = (1, "hello", 3.4);

    // Destructuring `let`
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c); // 1 hello 3.4

    // Indexing
    println!("{}", x.1); // hello
}

fn slices() {
    //define an array of size 4
    let arr: [i32; 4] = [1, 2, 3, 4];
    //define the slice
    let slice_array1: &[i32] = &arr;
    let slice_array2: &[i32] = &arr[0..2];
    // print the slice of an array
    // first index is inclusive
    println!("Slice of an array: {:?}", slice_array1);
    println!("Slice of an array: {:?}", slice_array2);
}

fn strings() {
    let str: &str = "sample string";
    println!("{}", str);

    // String literals
    let x: &str = "hello world!";
    println!("{}", x);

    // A `String` – a heap-allocated string
    // Stored as a `Vec<u8>` and always hold a valid UTF-8 sequence,
    // which is not null terminated.
    let s: String = "hello world".to_string();

    // A string slice – an immutable view into another string
    // This is basically an immutable pair of pointers to a string – it does’t
    // actually contain the contents of a string, just a pointer to
    // the begin and a pointer to the end of a string buffer,
    // statically allocated or contained in another object (in this case, `s`).
    // The string slice is like a view `&[u8]` into `Vec<T>`.
    let s_slice: &str = &s;
    println!("{} {}", s, s_slice);
}

fn arrays() {
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("\nPrint using a debug trait");
    println!("Array: {:?}", arr);
    println!("Length of array: {}", arr.len());

    // Vectors/arrays //

    // A fixed-size array
    let _four_ints: [i32; 4] = [1, 2, 3, 4];

    // A dynamic array (vector)
    let mut vector: Vec<i32> = vec![1, 2, 3, 4];
    vector.push(5);

    // A slice – an immutable view into a vector or array
    // This is much like a string slice, but for vectors
    let slice: &[i32] = &vector;

    // Use `{:?}` to print something debug-style
    println!("{:?} {:?}", vector, slice); // [1, 2, 3, 4, 5] [1, 2, 3, 4, 5]
}

fn variables() {
    let (course, category) = ("Rust", "beginner"); // assign multiple values
    println!("This is a {} course in {}.", category, course); // print the value
}

fn output() {
    //! This a doc comment that is inside the function
    //!
    // Default
    let w = "Hello, world";
    println!("{}!", w);

    // Positional placeholder
    println!("{0}{1}", w, "!");

    // Named arguments
    println!("{phase}{excl}", phase = w, excl = "!");

    // Placeholder traits
    println!("Number : 10 \nBinary:{:b} Hexadecimal:{:x} Octal:{:o}", 10, 10, 10);

    // Basic math
    println!("{} + {} = {}", 10, 10, 10 + 10);

    // Placeholder for debug traits
    println!("{:?}", ("This is a Rust Course", 101));

    // Printing styles

    // Prints an output but as an error
    eprint!("Rust Programming");
    eprintln!(" Course");

    println!("{}", 1);
    println!("{}{}", 2, 2);
    println!("{}{}{}", 3, 3, 3);
    println!("{}{}{}{}", 4, 4, 4, 4);
    println!("{}{}{}{}{}", 5, 5, 5, 5, 5);
}
