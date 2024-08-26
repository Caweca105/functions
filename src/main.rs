fn main() {
    // println!("Hello, world!");
    // another_function(5);
    // print_labeled_measurement(5, 'h');
    // let x = five();
    // println!("The value of x is: {x}");
    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    // println!("The value of x is: {x}");
    // println!("The value of y is: {y}");
    // println!("The value of z is: {z}");
    println!("The measurement is: {x}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}