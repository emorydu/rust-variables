use std::io;

fn main() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constants value is: {THREE_HOURS_IN_SECONDS}");


    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    // let mut spaces = "   ";
    // spaces = spaces.len();


    // Scalar types (integer, floating, boolean, character)
    // integer: i8, u8, i16, u16, `i32`, u32, i64, u64, i128, u128, isize, usize
    // Decimal: 98_222, Hex: 0xff, Octal: 0o77, Binary: 0b1111_0000, Byte: b'A'
    // floating: f64, f32
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x: {x}, y: {y}");
    // Numerical Operation
    // addition
    let sum = 5 + 10;
    println!("sum: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("product: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("quotient: {quotient}");
    println!("truncated: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");
    // boolean
    let t = true;
    println!("t: {t}");
    let f: bool = false;
    println!("f: {f}");
    // character
    let c = 'z';
    println!("c: {c}");
    let z: char = 'ℤ';
    println!("z: {z}");
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {heart_eyed_cat}");


    // Compound types (tuple, array)
    // tuple
    let tup: (i32, f64, u8) = (500, 6.3, 1);
    println!("{:?}", tup);
    let (x, y, z) = tup;
    println!("x: {x}, y: {y}, z: {z}");

    let x: (i32, f64, u8) = (500, 6.3, 1);
    let five_hundred = x.0;
    let six_point_three = x.1;
    let one = x.2;
    println!("five_hundred: {five_hundred}, six_point_three: {six_point_three}, one: {one}");

    // array
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
