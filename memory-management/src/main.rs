#![allow(unused_variables, dead_code)]

use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
struct MyPreciousRing {
    engraving: String,
}

impl Default for MyPreciousRing {
    fn default() -> Self {
        Self {
            engraving: "One ring to rule them all".to_string(),
        }
    }
}

impl Drop for MyPreciousRing {
    fn drop(&mut self) {
        println!("In Drop::drop: {}", self.engraving);
    }
}

fn main() {
    {
        println!("Checkpoint 1");
        let mut saurons_ring = MyPreciousRing::default();
        saurons_ring.engraving.push('1');
        println!("{:?}", saurons_ring);

        println!("Checkpoint 2");
        let mut gollums_ring = saurons_ring;
        gollums_ring.engraving.push('2');
        println!("{:?}", gollums_ring);

        println!("Checkpoint 3");
        let mut bilbos_ring = gollums_ring;
        bilbos_ring.engraving.push('3');
        println!("{:?}", bilbos_ring);

        println!("Checkpoint 4");
    }

    {
        let saurons_ring = Rc::new(MyPreciousRing::default());
        println!("saurons_ring: {:?}", saurons_ring);
        println!("Refcounter: {}", Rc::strong_count(&saurons_ring));

        let gollums_ring = Clone::clone(&saurons_ring);
        println!("gollums_ring: {:?}", gollums_ring);
        println!("Refcounter: {}", Rc::strong_count(&gollums_ring));

        println!("Pointer to saurons_ring: {:p}", saurons_ring);
        println!("Pointer to gollums_ring: {:p}", gollums_ring);

        drop(gollums_ring);
        println!("Refcounter: {}", Rc::strong_count(&saurons_ring));
    }

    {
        let saurons_ring = Rc::new(RefCell::new(MyPreciousRing::default()));
        let gollums_ring = saurons_ring.clone();
        let bilbos_ring = Clone::clone(&gollums_ring);

        {
            let mut ring = bilbos_ring.borrow_mut();
            ring.engraving.push('4');
            println!("ring: {:?}", ring);
        }

        let mut ring2 = gollums_ring.borrow_mut();
        ring2.engraving.push('5');
        println!("ring2: {:?}", ring2);
        drop(ring2);

        let ring1 = bilbos_ring.borrow();
        let ring2 = saurons_ring.borrow();
        println!("ring1: {:?}", ring1);
        println!("ring2: {:?}", ring2);

    }
}
