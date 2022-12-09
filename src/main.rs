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

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap()
{
    let p1: Point = origin();
    let p2: Box<Point> = Box::new(origin()); // p2 is on the heap

    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); // 16 bytes
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); // 8 bytes

    let p3: Point = *p2; // p3 is on the stack
                        // the * operator dereferences the pointer (follows the pointer)
    println!("{}", p3.x);
}

///////////////////////////////////////////

fn if_statement()
{
    let temperature: i32 = 15;

    if temperature > 30 {
        println!("really hot outside");
    } else if temperature < 10 {
        println!("really cold outside");
    } else {
        println!("temperature is ok");
    }

    let day: &str = if temperature > 20 { "sunny" } else { "cloudy" };
    println!("today is {}", day);

    println!("it is {}",
        if temperature > 20 { "hot" } else if temperature < 10 { "cold" } else { "ok" });

    println!("it is {}",
        if temperature > 20 {
            if temperature > 30 { "very hot" } else { "hot" }
        } else if temperature < 10 {
            if temperature < 0 { "very cold" } else { "cold" }
        } else {
            "ok"
        });
}

fn main() 
{
    if_statement();
}

///////////////////////////////////////////

fn while_and_loop()
{
    let mut x = 1;

    while x < 1000 {
        x *= 2;

        if x == 64 { continue; }

        println!("x = {}", x);
    }

    let mut y = 1;

    loop {
        y *= 2;
        println!("y = {}", y);

        if y == 1 << 10 { break; } // 2^10
    }
}

///////////////////////////////////////////

def for_loop()
{
    for x in 1..11 {
        if x == 3 { continue; }
        if x == 8 { break; }
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: y = {}", pos, y);
    }
}

///////////////////////////////////////////

fn match_statement()
{
    let country_code: i32 = 44;

    let country: &str = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=999 => "Unknown", // inclusive range
        _ => "Invalid" // catch-all
    };

    println!("the country with code {} is {}", country_code, country);
}

///////////////////////////////////////////

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use rand::Rng;
use std::io::stdin;

enum State {
    Locked,
    Failed,
    Unlocked
}

fn main() {
    let code = String::from("1234");
    // .unwrap() is used to get the value out of the Result
    // the unwrap() method will cause the program to panic if the value is an Err variant
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    },
                    Err(_) => continue,
                }

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                state = State::Locked;
                entry.clear(); // clear the entry string
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}

//////////////////////////////////////////////

struct Point 
{
    x: f64,
    y: f64
}

struct Line
{
    start: Point,
    end: Point
}

fn structures()
{
    let p = Point { x: 3.0, y: 4.0 };
    println!("point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, ..p }; // ..p means use the rest of the fields from p
    let myline = Line { start: p, end: p2 };
}

//////////////////////////////////////////////

enum Color
{
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), // tuple
    CmykColor{cyan: u8, magenta: u8, yellow: u8, black: u8} // struct
}

fn enums()
{
    let c: Color = Color::CmykColor{cyan: 0, magenta: 128, yellow: 0, black: 255};
    let c1: Color = Color::RgbColor(0, 0, 0);
    let c2: Color = Color::Red;
    let c3: Color = Color::Green;

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _ => ()
    }
}

//////////////////////////////////////////////

union IntOrFloat // takes up the same amount of space as the largest member
{
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat)
{
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life");
            },
            IntOrFloat { f } => {
                println!("f32 = {}", f);
            }
        }
    }
}

fn main()
{
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };
    println!("iof.i = {}", value);

    process_value(IntOrFloat { f: 42.0 });
    process_value(IntOrFloat { f: 1.23 });
    process_value(IntOrFloat { i: 5 });
}

////////////////////////////////////////

fn main()
{
    let x = 4.0;
    let y = 0.0;

    // Option -> Some(v) or None
    let result = if y != 0.0 { Some(x / y) } else { None };

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide {} by {}", x, y)
    }

    if let Some(z) = result { println!("{}/{} = {}", x, y, z) }

    while let Some(z) = result { println!("{}/{} = {}", x, y, z); break; }
}

////////////////////////////////////////

use std::mem; // for mem::size_of_val

fn arrays()
{
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);

    a[0] = 321;
    println!("{:?}", a); // debug print the array
    if a != [1, 2, 3, 4, 5] {
        println!("does not match");
    }

    let b = [3u8; 5]; // 5 elements, all 3s
    for i in 0..b.len() {
        println!("{}", b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));
    println!("{:?}", b);

    let mtx: [[f32; 3]; 2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }

    // println!("name[99] = {}", name[99]); // panic!
}

fn main()
{
    arrays();
}

////////////////////////////////////////

fn use_slice(slice: &mut[i32])
{
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices()
{
    let mut data = [1, 2, 3, 4, 5];

    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}

fn main()
{
    slices();
}

////////////////////////////////////////

fn sum_and_product(x: i32, y: i32) -> (i32, i32)
{
    (x + y, x * y)
}

fn tuples()
{
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);

    // destructuring: disassembling a structure into its parts
    let (a, b) = sp;
    println!("a = {}, b = {}", a, b);

    let sp2 = sum_and_product(4, 7);
    let combined = (sp, sp2);
    println!("combined = {:?}", combined);
    println!("last elem = {}", (combined.1).1);

    // destructure a couple of tuples
    let ((c, d), (e, f)) = combined;
    let foo = (true, 42.0, -1i8);
    println!("{:?}", foo);

    // single element tuple
    let meaning = (42,);
    println!("{:?}", meaning);
}

fn main()
{
    tuples()
}

////////////////////////////////////////

fn how_many(x: i32) -> &'static str
{
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        z @ 9...11 => "lots of", // z is bound to the value of x, 9...11 is a range
        _ if (x % 2 == 0) => "some",
        _ if (x > 10) => "lots",
        _ => "some"
    }
} // x is borrowed here, but it is no longer used

pub fn pattern_matching()
{
    for x in 0..13
    {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3, 4);

    match point {
        (0, 0) => println!("origin"),
        (0, y) => println!("y axis, y = {}", y),
        (ref mut x, 0) => println!("x axis, x = {}", x), // x is borrowed here, passing it by reference
                                                            // and is made mutable here
        (x, y) => println!("({}, {})", x, y),
        (_, y) => println!("(?, {})", y)
    }
}

// if you don't care about all the values in pattern matching you can just specify
// the ones you care about and use _ for the rest, or you can use .. to ignore the rest

////////////////////////////////////////

// Option<T> is an enum with two variants: Some(T) and None
struct Point<T>
{
    x: T,
    y: T
}

struct Line<T>
{
    start: Point<T>,
    end: Point<T>
}

fn generics()
{
    let a: Point<f32> = Point { x: 0.3, y: 0.4 };
    let b: Point<i32> = Point { x: 0, y: 7 };
    let c: Point<u16, i32> = Point { x: 0, y: 7 };
    // type deduction is better than you. 

    println!("a.x = {}, b.y = {}", a.x, b.y);

    let myline = Line { start: a, end: b };
}

fn main()
{
    generics();
}