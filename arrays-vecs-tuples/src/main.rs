use std::collections::HashSet;

fn main() {
    let mut numbers = [1, 2u16, 3, 4, 5];
    numbers[0] = 10;
    println!("{}", numbers[0]);

    let numbers = [-1; 10];

    const LEN: usize = 10;
    let numbers = [-1; LEN];

    let len = 10usize;
    // let numbers = [-1; len]; // ⚠️ ERROR

    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(6);

    let mut numbers = Vec::with_capacity(10);
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    numbers.push(4);
    numbers.push(5u8);

    let mut numbers = HashSet::new();
    numbers.insert(1);
    numbers.insert(2);
    numbers.insert(3);
    numbers.insert(4);
    numbers.insert(5u8);

    let mut my_tuple = (1, "Hello World", true);
    my_tuple.0 = 10;
    println!("{}", my_tuple.0);

    let unit = (); // void

    let mut number = Box::new(42);
    *number += 1;
    println!("{}", number);

    let my_tuple = Box::new((1, "Hello World", true));
    println!("{}", my_tuple.0);

    let numbers = Box::new([1, 2, 3, 4, 5]);
    println!("{}", numbers[0]);
    
}
