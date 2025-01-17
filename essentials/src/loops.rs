fn nested_loop() {
    'outer: for i in 0..5 {
        'inner: for j in 0..5 {
            if i > 3 {
                break 'outer;
            }
        }
    }

    println!("Exited the outer loop");
}