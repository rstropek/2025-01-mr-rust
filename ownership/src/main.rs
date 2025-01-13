#![allow(dead_code, unused_variables)]

struct Person {
    first_name: String,
    last_name: String,
}

fn main() {
    let p = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
    };
    println!("{} {}", p.first_name, p.last_name);
    let p2 = p;
    println!("{} {}", p2.first_name, p2.last_name);
    print_person(&p2);
    let mut p3 = p2;

    let rob_1 = &p3;
    let rob_2 = &p3;
    println!("{} {}", rob_1.first_name, rob_1.last_name);
    println!("{} {}", rob_2.first_name, rob_2.last_name);
    
    let mub = &mut p3;
    change_person(mub);
    print_person(&p3);
    
    let rob_1 = &p3;
    let rob_2 = &p3;
    println!("{} {}", rob_1.first_name, rob_1.last_name);
    println!("{} {}", rob_2.first_name, rob_2.last_name);

    print_person(&create_person());
}

fn print_person(p: &Person) {
    println!("{} {}", p.first_name, p.last_name);
}

fn change_person(p: &mut Person) {
    p.first_name = "Jane".to_string();
}

fn create_person() -> Person {
    Person {
        first_name: "John".to_string(),
        last_name: "Wick".to_string(),
    }
}