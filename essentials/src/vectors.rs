pub fn basics() {
    let mut my_vec = Vec::new();
    println!("Empty Vector : {:?}", my_vec);
    my_vec.push(1);
    my_vec.push(2);
    my_vec.push(3);
    println!("Pushed elements 1 , 2 , 3 : {:?}", my_vec);
    my_vec.pop();
    println!("Popped value: {}", 3);
    println!("Popped element at last index : {:?}", my_vec);
    my_vec.remove(1);
    println!("Removed value: {}", 2);
    println!("Removed element at index 1 : {:?}", my_vec);
    println!("Size of vector is :{}", my_vec.len());
    println!("Does my vector contains 1 : {}", my_vec.contains(&1));
}

#[allow(unused)]
pub fn iterators() {
    // defines a mutable vector
    let mut my_vec = vec![1, 2, 3, 4, 5];
    // define the value to be removed
    let value = 2;
    // get the index of the value in the vector
    let index = my_vec.iter().position(|&r| r == value).unwrap();
    // call the built-in remove method
    my_vec.remove(index);
    // print the updated vector
    println!("Updated Vector: {:?}", my_vec);
}

#[allow(unused)]
pub fn whiling() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{top}");
    }
}

#[allow(unused)]
pub fn looping() {
    // define a vector of size 5
    let my_vec = vec![1, 2, 3, 4, 5];
    // using loop
    let mut index = 0;
    for i in my_vec.iter(){ // it works even if .iter() is not written
        println!("Element at index {}:{} ", index, i);
        index = index + 1;
    }
}

#[allow(unused)]
pub fn loop_mutate(){
    // define a vector of size 5
    let mut my_vec = vec![1, 2, 3, 4, 5];
    println!("Initial Vector : {:?}", my_vec);
    for x in my_vec.iter_mut(){
        *x *= 3;
    }
    // print the updated vector
    println!("Updated Vector : {:?}", my_vec);
}

#[allow(unused)]
pub fn slices() {
    let my_vec = vec![1, 2, 3, 4, 5];
    let slice:&[i32] = &my_vec[2..4];
    println!("Slice of the vector : {:?}",slice);
}

#[allow(unused)]
pub fn remove(v: &mut Vec<u32>) -> &mut Vec<u32> {
    let mi = v.len() / 2;
    v.pop();
    v.remove(mi - 1);
    let mut sum = 0;
    for x in v.iter() {
        sum += x;
    }

    v.push(sum);
    v
}
