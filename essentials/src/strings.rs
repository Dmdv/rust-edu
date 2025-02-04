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

#[allow(unused)]
fn reverse_string(str: String) -> String {
    let s = str.as_str();
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut res = vec!['\0'; len];

    for i in 0..len / 2 {
        res[i] = chars[len - 1 - i]
    }
    
    res.into_iter().collect()
}

fn reserse_string_simpl(str: String) -> String {
    str.chars().rev().collect()
}