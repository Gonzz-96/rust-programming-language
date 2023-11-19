fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = 6; // statement

    let y = {
        let x = 3;
        // expressions do not include ending semicolons
        // if it's added, the expression will be turned
        // into a statement
        x + 1
    }; // new scopes are expressions
    
    let x = five();
    let x = plus_one(x);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}
