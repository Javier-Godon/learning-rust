//signed (+ and -) and unsigned (only +) integers
//signed integers: i8, i16, i32, i64, i128, isize
//unsigned integers: u8, u16, u32, u64, u128, usize
//floating point numbers: f32, f64
//boolean: bool
//character: char

pub fn primitives() {
    let x: i32 = -42; // signed integer
    let y: u64 = 100; // unsigned integer
    println!("x: {}, y: {}", x, y);
    println!("x + y: {}", x + y as i32); // casting u64 to i32
    let pi: f64 =3.141592653589793238462643383279502884197; // floating point number
    println!("pi: {}", pi);
    let is_sunny: bool = true; // boolean
    println!("is_sunny: {}", is_sunny);
    let letter: char = 'A'; // character
    println!("first letter of the alphabet: {}", letter);
}
