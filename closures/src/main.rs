use std::{fmt::Display, ops::Deref};

fn main() {
    fn add(a: &i32, b: &i32) -> i32 {
        *a + *b
    }

    let f = add;
    println!("{}", f(&1, &2));
    apply_and_print(f, 1, 2);

    let f = |a: &f64, b: &f64 | a - b;
    apply_and_print(f, 1.0, 2.0);

    apply_and_print(|a, b| a * b, 1, 2);

    let factor = 10;
    let factor2 = &factor;
    apply_and_print_2(Box::new(|a, b| a * b * factor2), 1, 2);
    apply_and_print_2(Box::new(|a, b| a + b + factor2), 1, 2);

    let mut result = 0;
    let mut calc_result  = |x, y| { result = x + y; };
    calc_result(1, 2);
    println!("{}", result);

    let concat = |a: &String, b: &String| a.clone() + b;
    apply_and_print(concat, String::from("Hello "), String::from("World"));
}

fn apply_and_print<T: Display>(f: fn(&T, &T) -> T, a: T, b: T) {
    println!("{} + {} = {}", a, b, f(&a, &b));
}

fn apply_and_print_2<T: Display>(f: Box<dyn Fn(T, T) -> T + '_>, a: T, b: T) {
    println!("{}", f(a, b));
}
