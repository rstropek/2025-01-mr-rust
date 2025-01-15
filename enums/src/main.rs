#![allow(unused_variables, dead_code)]

use std::fmt::Display;

enum Colors
{
    Red = 0,
    Green = 1,
    Blue = 2,
}

#[derive(PartialEq, Eq)]
struct Person
{
    name: String,
    age: u8,
}

#[derive(PartialEq, Eq)]
enum HotelRoomStatus
{
    Vacant,
    Occupied(Person),
    Maintenance(String),
}

#[derive(Copy, Clone)]
enum Player{
    X,
    O,
}

impl From<u8> for Player {
    fn from(value: u8) -> Self {
        match value {
            0 => Player::X,
            1 => Player::O,
            _ => panic!("Invalid player value"),
        }
    }
}

impl From<Player> for u8 {
    fn from(value: Player) -> Self {
        match value {
            Player::X => 0,
            Player::O => 1,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Player::X => "X",
            Player::O => "O",
        })
    }
}

fn main() {
    let c = Colors::Red;
    match c
    {
        Colors::Red => println!("Red"),
        Colors::Green => println!("Green"),
        Colors::Blue => println!("Blue"),
    }

    let room = HotelRoomStatus::Occupied(Person { name: "John Doe".to_string(), age: 30 });
    let room = HotelRoomStatus::Maintenance("Leak in the roof".to_string());

    match room {
        HotelRoomStatus::Vacant => println!("Room is vacant"),
        HotelRoomStatus::Occupied(person) => println!("Room is occupied by {}", person.name),
        HotelRoomStatus::Maintenance(maintenance_info) => println!("Room is in maintenance: {}", maintenance_info),
    }

    let room = HotelRoomStatus::Vacant;
    if room == HotelRoomStatus::Vacant {
        println!("Room is vacant");
    }

    if let HotelRoomStatus::Occupied(person) = room {
        println!("Room is occupied by {}", person.name);
    }

    let numbers = [1, 2, 3];
    let i_am_sure = numbers.iter().next().unwrap();
    let first = numbers.iter().next();

    match first {
        Some(n) => println!("First number: {}", n),
        None => println!("No number found"),
    }

    if let Some(n) = first {
        println!("First number: {}", n);
    }

    let board: [[Option<Player>;3];3] = [[None;3];3];

    let player_ix: u8 = Player::X.into();
    let player_ix: u8 = From::from(Player::X);
    let player: Player = 1.into();
    let player: Player = From::from(1);

    let result = div(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => match error {
            DivErrors::DivisionByZero => println!("Division by zero"),
            DivErrors::SomeOtherError | DivErrors::SomeEvenMoreStrangeError => println!("Some other error"),
        },
    }

}

#[derive(Debug)]
enum DivErrors {
    DivisionByZero,
    SomeOtherError,
    SomeEvenMoreStrangeError,
}

fn div(a: i32, b: i32) -> Result<i32, DivErrors> {
    if b == 0 {
        return Err(DivErrors::DivisionByZero);
    }
    Ok(a / b)
}