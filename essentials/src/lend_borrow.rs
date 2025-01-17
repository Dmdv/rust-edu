pub fn run() {
    let str = String::from("Hello from origin");
    take_owned_string_ref(&str);
    take_owned_string_ref(&str);
    println!("{}", str);
    take_string(str.as_str());
    take_string(str.as_str());
    println!("{}", str);

    take_owned_string(str);
    // Error
    // take_owned_string(str);

    let ret = return_string();
    println!("{}", ret);

    let ret_owned = return_owned_string();
    println!("{}", ret_owned);
}

fn take_owned_string(str: String) {
    println!("{}", str);
}

fn take_owned_string_ref(str: &String) {
    println!("{}", str);
}

fn take_string(str: &str) {
    println!("{}", str);
}

fn return_owned_string() -> String {
    String::from("Hello from returned string")
}

fn return_string() -> &'static str {
    "Hello from returned string"
}