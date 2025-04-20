fn five() -> i32 {
    15
}

fn main() {
    println!("Hello, world!");
    
    print_labeled_measurement(100, 'h');
    another_function(5);

    main_two();
    
    //function five()
    let x = five();

    println!("The value of x in five() is: {x}");

    //function plus_one()
    let z = plus_one(8);
    println!("The value of z in plus_one is: {z}");


}

fn another_function(x: i32) {
    println!("The value of x in another_function is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}")
}

fn main_two() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn plus_one(z: i32) -> i32 {
    z + 1
}