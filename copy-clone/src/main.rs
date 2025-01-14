#[derive(Copy, Clone, Debug)]
struct Vector2d {
    x: f32,
    y: f32,
}

impl Vector2d {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let v = Vector2d::new(1.0, 2.0);
    let v2 = v;
    print_vector(v2);
}

fn print_vector(v: Vector2d) {
    println!("Vector: {v:?}");
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
