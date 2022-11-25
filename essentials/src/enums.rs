#[derive(Debug)]
enum KnightMoveString {
    Horizontal(String),
    Vertical(String),
}

pub(crate) fn enums_run() {
    let horizontal_move = KnightMoveString::Horizontal("Left".to_string());
    let vertical_move = KnightMoveString::Vertical("Down".to_string());
    println!("{:?}", horizontal_move);
    println!("{:?}", vertical_move);
}

#[allow(dead_code)]
#[derive(Debug)]
// declare an enum
enum TrafficSignal {
    Red,
    Green,
    Yellow,
}

//implement a Traffic Signal methods
impl TrafficSignal {
    // if the signal is red then return
    fn is_stop(&self) -> bool {
        return match self {
            TrafficSignal::Red => true,
            _ => false
        };
    }
}

pub fn enums_run_methods() {
    // define an enum instance
    let action = TrafficSignal::Red;
    //print the value of action
    println!("What is the signal value? - {:?}", action);
    //invoke the enum method 'is_stop' and print the value
    println!("Do we have to stop at signal? - {}", action.is_stop());
}

#[derive(Debug)]
enum KnightMove {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
#[allow(unused)]
struct Player {
    color: String,
    knight: KnightMove,
}

// print function
fn print_direction(direction: KnightMove) {
    match direction {
        KnightMove::Horizontal => {
            println!("Move in horizontal direction");
        }
        KnightMove::Vertical => {
            println!("Move in vertical direction");
        }
    }
}

pub fn enums_run_match() {
    let knight1 = KnightMove::Horizontal;
    let knight2 = KnightMove::Vertical;
    print_direction(knight1);
    print_direction(knight2);

    // instance 1
    let p1 = Player {
        color: String::from("black"),
        knight: KnightMove::Horizontal,
    };

    // instance 2
    let p2 = Player {
        color: String::from("white"),
        knight: KnightMove::Vertical,
    };

    println!("{:?}", p1);
    println!("{:?}", p2);

    assert_eq!(Days::Sunday.is_weekend(), 1);

    #[allow(dead_code)]
    enum Days {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    impl Days {
        fn is_weekend(&self) -> i32 {
            return match self {
                &Days::Saturday => 1,
                &Days::Sunday => 1,
                _ => 0
            };
        }
    }
}

pub fn enums_options() {
    fn learn_lang(my_lang: &str) -> Option<bool> {
        if my_lang == "Rust" {
            Some(true)
        } else {
            None
        }
    }

    println!("{:?}", learn_lang("Rust"));
    println!("{:?}", learn_lang("Python"));

    let x: Option<u32> = Some(2);
    println!("{:?}", x);

    // As ref

    let text: Option<String> = Some("Hello, world!".to_string());
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("still can print text: {text:?}");
    println!("{text_length:?}");

    // As mut

    // Converts from &mut Option<T> to Option<&mut> T.
    let mut x = Some(2);
    match x.as_mut() {
        Some(v) => *v = 42,
        None => {}
    }
    assert_eq!(x, Some(42));

    // Unwrap

    assert_eq!(Some("car").unwrap_or("bike"), "car");
    assert_eq!(None.unwrap_or("bike"), "bike");

    // Is some

    let my_val: Option<&str> = Some("Rust Programming!");
    assert_eq!(my_val.is_some(), true);

    struct Course {
        code: i32,
        name: String,
        level: Option<String>,
    }

    let course1 = Course {
        name: String::from("Rust"),
        level: Some(String::from("beginner")),
        code: 130,
    };

    let course2 = Course {
        name: String::from("Javascript"),
        level: None,
        code: 122,
    };

    println!("Name:{}, Level:{} ,code: {}",
             course1.name,
             course1.level.unwrap_or("Level".to_string()),
             course1.code);

    println!("Name:{}, Level:{} ,code: {}",
             course2.name,
             course2.level.unwrap_or("No level defined!".to_string()),
             course2.code);
}


pub fn out_of_bands() {
    fn lookup(str: String, index: usize) {
        let matched_index = match str.chars().nth(index) {
            // execute if match found print the value at specified index
            Some(c) => c.to_string(),
            // execute if value not found
            None => "No character at given index".to_string()
        };
        println!("{}", matched_index);
    }

    let str = String::from("Educative");
    lookup(str, 12);
}

pub fn result_example() {
    fn divisible_by_3(i: i32) -> Result<String, String> {
        if i % 3 == 0 {
            Ok("Given number is divisible by 3".to_string())
        } else {
            Err("Given number is not divisible by 3".to_string())
        }
    }

    println!("{:?}", file_found(true));
    println!("{:?}", file_found(false));

    println!("{:?}", divisible_by_3(6));
    println!("{:?}", divisible_by_3(2));

    if divisible_by_3(6).is_ok() {
        println!("The number is divisible by 3");
    } else {
        println!("The number is not divisible by 3");
    }

    if divisible_by_3(2).is_err() {
        println!("The number is not divisible by 3");
    } else {
        println!("The number is divisible by 3");
    }

    fn file_found(i: bool) -> Result<i32, bool> {
        if i {
            Ok(200)
        } else {
            Err(false)
        }
    }
}