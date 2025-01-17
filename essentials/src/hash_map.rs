use std::collections::HashMap;

fn hash_map(){
    let mut letters = HashMap::new();

    for ch in "some_text".chars() {
        let counter = letters.entry(ch).or_insert(0);
        *counter += 1;
    }
}