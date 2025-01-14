#![allow(dead_code, unused_variables)]

use std::fmt::Display;

use rand::Rng;

trait Billable {
    fn total(&self) -> f32;
}

#[derive(Debug, PartialEq, Clone)]
struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

impl Billable for ConsultingWork {
    fn total(&self) -> f32 {
        self.hours * self.rate
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Material {
    what: String,
    costs_per_item: f32,
    quantity: f32,
}

impl Billable for Material {
    fn total(&self) -> f32 {
        self.costs_per_item * self.quantity
    }
}

fn get_something_billable() -> impl Billable {
    Material {
        what: "Book about Rust".to_string(),
        costs_per_item: 100.0,
        quantity: 1.0,
    }
}

fn get_something_else_billable() -> impl Billable {
    // ConsultingWork {
    //     what: "Rust Training".to_string(),
    //     hours: 10.0,
    //     rate: 100.0,
    // }
    
    // vec![1.0, 2.0, 3.0]

    [1.0, 2.0, 3.0]
}

fn get_something_random() -> Box<dyn Billable> {
    if rand::thread_rng().gen_range(0..=1) == 0 {
        Box::new(42.0)
    } else {
        Box::new(vec![1.0, 2.0, 3.0])
    }
}

fn print_billable(item: &impl Billable) {
    println!("Total: {:?}", item.total());
}

fn print_dyn_billable(item: &Box<dyn Billable>) {
    println!("Total: {:?}", item.total());
}

impl Billable for f32 {
    fn total(&self) -> f32 {
        *self
    }
}

impl Billable for Vec<f32> {
    fn total(&self) -> f32 {
        self.iter().sum()
    }
}

impl<const N: usize> Billable for [f32; N] {
    fn total(&self) -> f32 {
        self.iter().sum()
    }
}

trait Pointworthy {
    fn point(&self) -> i32;
}

impl Display for ConsultingWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "You pay {} for {} hours of {}", self.total(), self.hours, self.what)
    }
}

//impl<T: Billable> Pointworthy for T {
impl<T> Pointworthy for T
where
    T: Billable,
{
    fn point(&self) -> i32 {
        self.total() as i32 / 100
    }
}

fn print_points(item: &impl Pointworthy) {
    println!("Points: {:?}", item.point());
}

fn main() {
    let cw = get_something_billable();
    print_billable(&cw);
    print_points(&cw);

    let i1 = get_something_billable();
    let i2 = get_something_billable();
    let i3 = get_something_billable();
    let borrowed_items = vec![&i1, &i2, &i3];
    for item in borrowed_items {
        print_billable(item);
        print_points(item);
    }

    let cw = get_something_else_billable();
    print_billable(&cw);
    print_points(&cw);

    let cw = ConsultingWork {
        what: "Rust Training".to_string(),
        hours: 10.0,
        rate: 100.0,
    };
    println!("{}", cw);

    let arr = [1.0, 2.0, 3.0];
    print_billable(&arr);

    let arr = [1.0, 2.0, 3.0, 4.0];
    print_billable(&arr);

    let cw = [
        get_something_random(),
        get_something_random(),
        get_something_random(),
    ];
    for item in cw {
        print_dyn_billable(&item);
    }
}
