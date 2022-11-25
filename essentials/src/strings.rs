#[allow(unused)]
fn create() {
    let s: String = "Hello".to_string();
    let s: String = String::from("world");
    let s: String = "also this".into();
}

// Concatenate all strings starting with 'c'
#[allow(unused)]
fn test(my_str:String)-> String {
    let mut res = "".to_string();
    for found in my_str.split_whitespace() {
        if found.starts_with("c") {
            res.push_str(found);
            res.push(' ');
        }
    }

    res.pop();
    res
}