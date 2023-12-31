
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is : {x}");

    // floating point numbers
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32
    
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;

    let quotiend = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    let remainder = 43 % 5;

    // booleans
    let t = true;
    let f: bool = false;

    // chars
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // tuple
    // they cannot modify their size (can´t grow or shrink)
    // the tuple type can be omitted
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // we can use destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // we can access tuple items individually
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // arrays
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5] // same as having [3, 3, 3, 3, 3]

    let first = a[0];
    let second = a[1];

}

