#![allow(dead_code, unused_variables)]

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Order<'a> {
    person: &'a Person,
    product: String,
    quantity: u8,
}

#[derive(Debug, PartialEq)]
struct Point2d {
    x: f32,
    y: f32,
}

#[derive(Debug, PartialEq)]
struct Line {
    start: Point2d,
    end: Point2d,
}

impl Line {
    fn length(&self) -> f32 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn get_longer_line<'a>(l1: &'a Line, l2: &'a Line) -> &'a Line {
    if l1.length() > l2.length() {
        l1
    } else {
        l2
    }
}

fn main() {
    {
        let p = Person {
            name: "John".to_string(),
            age: 25,
        };

        let o = Order {
            person: &p,
            product: "Laptop".to_string(),
            quantity: 1,
        };

        let o2 = Order {
            person: &p,
            product: "Mouse".to_string(),
            quantity: 1,
        };
        
        println!("{o:?}");
        println!("{o2:?}");
    }

    {
        let mut people = vec![
            Person {
                name: "John".to_string(),
                age: 25,
            },
            Person {
                name: "Jane".to_string(),
                age: 30,
            },
        ];

        //let john = people.remove(0);

        let orders = vec![
            Order {
                person: &people[0],
                product: "Laptop".to_string(),
                quantity: 1,
            },
            Order {
                person: &people[1],
                product: "Mouse".to_string(),
                quantity: 1,
            },
        ];
        
        for o in orders {
            println!("{o:?}");
        }
    }

    {
        let l1 = Line {
            start: Point2d { x: 0.0, y: 0.0 },
            end: Point2d { x: 1.0, y: 1.0 },
        };

        let l2 = Line {
            start: Point2d { x: 0.0, y: 0.0 },
            end: Point2d { x: 2.0, y: 2.0 },
        };

        let longer = get_longer_line(&l1, &l2);
        //println!("{:p}, {:p}, {:p}", &l1, &l2, longer);
        println!("{longer:?}");
    }
}
