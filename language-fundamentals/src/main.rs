#![allow(dead_code)]

const ANSWER: u32 = 42;
const ANSWER_PLUS_ONE: i32 = add(ANSWER, 1u32);

fn main() {
    let n;
    n = 42;
    println!("{n}");
    
    let n = "43"; // Shaddowing
    println!("{n}");
    
    let user_numeric_input = "42";
    let user_numeric_input: i32 = user_numeric_input.parse().unwrap(); // Shaddowing
    println!("{user_numeric_input}");
    
    let mut n = 42;
    n += 1;
    println!("{n}");
    
    let n = 42;
    let result = add(n, 1);
    println!("{result}");
    
    let n = 2u16;
    let n = n.wrapping_add(65535);
    println!("{n}");
    
    let mut n = 42;
    n += 1;
    
    let n = n;
    println!("{n}");
    
    let mut n = n;
    n += 1;
    println!("{n}");
}

const fn add(a: u32, b: u32) -> i32 {
    a as i32 + b as i32
}

fn div(a: i32, b: i32) -> f32 {
    if b == 0 {
        0.0
    } else {
        a as f32 / b as f32
    }
}

fn find_first() -> i32 {
    let mut i = 0;
    let result = loop {
        if i % 10 == 0 {
            break i;
        }
        i += 1;
    };
    result
}