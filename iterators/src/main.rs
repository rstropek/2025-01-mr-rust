fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let my_iter = numbers.iter().skip(2).take(2).map(|x| x * 2).filter(|x| *x > 7);
    let my_iter = my_iter.rev();
    for number in my_iter {
        println!("{}", number);
    }

    let numbers = [1, 2, 3, 4, 5];
    let result: Vec<_> = numbers.iter().skip(2).take(2).map(|x| x * 2).filter(|x| *x > 7).rev().collect();
    
    for number in result.iter().rev() {
        println!("{}", number);
    }
    
    let mut numbers = ["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string()];
    for number in &mut numbers {
        println!("{}", number);
        number.push('!');
    }
    for number in numbers {
        println!("{}", number);
    }

}
