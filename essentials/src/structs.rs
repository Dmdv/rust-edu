#[allow(unused)]
pub fn test_structs() {
    // Declaration
    struct Course {
        name: String,
        age: i32,
    }

    // Implementation
    impl Course {
        fn name_and_age(&self) -> String {
            format!("{}-{}", self.name, self.age)
        }
        fn some_static_method(c: Course) -> String {
            format!("static {}-{}", c.name, c.age)
        }
    }

    // Initialization
    let course = Course {
        name: "Course".to_string(),
        age: 35
    };

    let s1 = course.name_and_age();
    println!("{}", s1);

    let s2 = Course::some_static_method(course);
    println!("{}", s2);
}

pub fn test_tuple_struct() {
    struct FruitQuantity(String, i32);

    // create an instance
    let r1 = FruitQuantity("oranges".to_string(), 12);
    // access values of a tuple struct
    println!("r1--name:{} quantity:{}", r1.0, r1.1);
    // create an instance
    let r2 = FruitQuantity("mangoes".to_string(), 13);
    // access values of a tuple struct
    println!("r2--name:{} quantity:{}", r2.0, r2.1);
}

#[allow(unused)]
pub fn distance() {
    // use std::num::sqrt;

    struct Point {
        x: i32,
        y: i32
    }

    fn test(p1: Point, p2: Point)-> f32 {
        let dist = (p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2);
        let df = dist as f32;
        df.sqrt()
    }
}