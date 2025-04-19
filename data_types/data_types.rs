//*  FLOATING-POINT TYPES
fn main() {
    let x = 2.0 //f64
    let y: f32 = 3.0 //f32
}

//* NUMERIC OPERATIONS
fn mid() {
    //addition
    let sum = 5 + 10;

    //subtraction
    let difference = 95.5 - 4.3;
    
    //multiplication
    let product = 4 * 30;

    //division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    //remainder
    let remainder = 43 % 5;
}

//* THE BOOLEAN TYPE
fn midTwo() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}

//* THE CHARACTER TYPE
fn midThree() {
    let c = "z";
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

//* THE TUPLE TYPE
fn midFour() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

fn midFive() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
// This program first creates a tuple and binds it to the variable tup. It then uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. This is called destructuring because it breaks the single tuple into three parts. Finally, the program prints the value of y, which is 6.4.

fn midSix() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
// Access each element of the tuple using their respective indices.

//* THE ARRAY TYPE
fn midSeven() {
    let a = [1, 2, 3, 4, 5];

    let b = [i32, 5] = [1, 2, 3, 4 5];

    let c = [3; 5]; // let c = [3, 3, 3, 3, 3];

    //* Acessing array elements

    let first = a[0];
    let second = a[1];
}



