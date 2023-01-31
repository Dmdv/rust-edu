#[derive(Debug)]
struct Data {
    value: i32,
}

fn main() {
    let mut data: Vec<Data> = vec![
        Data { value: 1 },
        Data { value: 2 },
        Data { value: 3 },
    ];

    data.push(Data { value: 4 });

    let last: &Data = &data[2];

    do_stuff_with_list(&mut data);

    println!("{data:?}");

    println!("{:?}", data[2].value);
}

fn do_stuff_with_list(data: &mut Vec<Data>) {
    data.pop();
}
