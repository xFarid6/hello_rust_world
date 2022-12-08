fn main() {
    println!("Hello, world!");

    println!("I'm a Rustacean!");
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // pick a random number
    let secret_number = rand::random::<u32>() % 100 + 1;

    println!("You guessed: {}", guess);

    println!("The secret number is: {}", secret_number);

    if guess.trim() == secret_number.to_string() {
        println!("You win!");
    } else {
        println!("You lose!");
    }
}

//////////////////////////////////////////////////////////

use std::mem;

fn operators()
{
    // arithmetic
    let mut a = 2 + 3 * 4;
    println!("a = {}", a);
    a = a + 1;
    a -= 2;
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b: f64 = 2.5;
    let b_cubed: f64 = f64::powi(b, 3);
    let b_to_pi: f64 = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let b = 1 | 2; // | OR & AND ^ XOR ! NOR
                        // 01 OR 10 = 11 == 3_10
    println!("{} OR {} = {}", 1, 2, b);

    // shift
    let two_to_10 = 1 << 10; // 2^10
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0; // true
    println!("Pi < 4 is {}", pi_less_4);

    let x = 5;
    let x_is_5 = x == 5; // true
    println!("x is 5 is {}", x_is_5);
}

///////////////////////////////////////////

fn scope_and_shadowing()
{
    let a = 123;

    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777; // shadowing, not reassigning, 
                        // has the same name but is a different thing
        println!("inside, a = {}", a);
    }

    // println!("outside, b = {}", b);
    println!("outside, a = {}", a);

    let a = 888;
    println!("outside, a = {}", a);
}

///////////////////////////////////////////

fn data_types()
{
    // unsigned
    let a: u8 = 123; // 8 bits
    println!("a = {}", a);

    // signed
    let mut b: i8 = 0; // 8 bits
    println!("b = {}, b + 1 = {}", b, b + 1);
    b = 42;
    println!("b = {}", b);

    // i/u size/ptr
    let c = 123456789; // 32-bit signed i32
    println!("c = {}, size = {} bytes", c, mem::size_of_val(&c));

    let z: isize = 123; // isize/usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z * 8);

    let d: char = 'x';
    println!("d = {}, size = {} bytes", d, mem::size_of_val(&d));

    let e = 2.5; // double-precision, 8 bytes or 64 bits, f64
    println!("e = {}, size = {} bytes", e, mem::size_of_val(&e));

    // boolean
    let g = false;
    println!("g = {}, size = {} bytes", g, mem::size_of_val(&g));
}

///////////////////////////////////////////

