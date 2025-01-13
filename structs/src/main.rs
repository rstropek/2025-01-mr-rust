mod clock {
    pub struct Clock {
        hours: u8,
        minutes: u8,
    }

    impl Clock {
        pub fn new(hours: u8, minutes: u8) -> Self {
            Self { hours, minutes }
        }

        pub fn print(&self) -> &Self {
            println!("{}:{}", self.hours, self.minutes);
            self
        }

        pub fn add(&self, minutes: u8) -> Self {
            Clock::new(self.hours, self.minutes + minutes)
        }

        pub fn add_2(&mut self, minutes: u8) -> &mut Self {
            self.minutes += minutes;
            self
        }
    }
}

fn main() {
    let c = clock::Clock::new(10, 30);
    c.print();
    let mut c = c.add(10);
    c.add_2(10).add_2(10);
    // c.add_3(10);
    c.print();
}
