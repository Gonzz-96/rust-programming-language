
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
    let difference = 95.5 4.3;
    let product = 4 * 30;

    let quotiend = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    let remainder = 43 % 5;
}

