use rand::Rng;

fn main() {
    let result = "42".parse::<u8>().unwrap();

    let num: u8 = rand::thread_rng().gen_range(1..=10);

    let message = match num {
        1..=4 => "lost",
        5 => "nearly",
        6..=10 => "won",
        _ => panic!("Invalid number")
    };

    println!("{message}");

    let result = match num {
        1..=4 => Result::Lost,
        5 => Result::Nearly,
        6..=10 => Result::Won,
        _ => panic!("Invalid number")
    };

    let message = match result {
        Result::Lost => "lost",
        Result::Nearly => "nearly",
        Result::Won => "won",
    };

    println!("{message}");
}

#[repr(u16)]
enum Result {
    Lost,
    Nearly,
    Won,
}
