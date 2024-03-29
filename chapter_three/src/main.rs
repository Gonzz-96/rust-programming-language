const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Hello, world!");
    projects();
    shadowing();
    data_types();
    tuples();
    arrays();
}

fn projects() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // error if 'mut' is not present
    println!("The value of x is: {}", x);
}

fn shadowing() {
    let x = 5;
    // shadowing 'x'
    let x = x + 1;
    let x = x * 2;
    println!("\nThe value of x is: {}", x);
}

fn data_types() {
    let guess: u32 = "42".parse().expect("Not a number");

    let x = 2.0; // f64 by default, double precision
    let y: f32 = 3.0;

    // operations
    5 + 10;
    95.5 - 4.3;
    4 * 30;
    56.7 / 32.2;
    43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z = '∞';
    let heard_eyed_cat = '😻';
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring
    let (x, y, z) = tup;

    println!("THe value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}

fn arrays() {
    // arrays can't shrink or grow
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let first = a[0];
    let second = a[1];
    // this is actually a compilation error unlike a runtime error
    // as mentioned in "the rust programming language book"
    // I believe the compiler is already improved
    let element = a[index];
}

fn another_function() {
    println!("Another function");
    println!("Another functino, bro");
}
