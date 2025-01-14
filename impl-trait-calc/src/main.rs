mod calc {
    struct CalculatorImpl {
        value: i32,
    }

    impl CalculatorImpl {
        fn new(initial_value: i32) -> Self {
            Self { value: initial_value }
        }
    }

    pub trait Calculator {
        fn calculate_result(&self) -> i32;
    }

    impl Calculator for CalculatorImpl {
        fn calculate_result(&self) -> i32 {
            self.value * 10
        }
    }

    pub fn calculator_factory(initial_value: i32) -> impl Calculator {
        CalculatorImpl::new(initial_value)
    }
}

use calc::Calculator;

fn main() {
    let calculator = calc::calculator_factory(1);
    println!("{}", calculator.calculate_result());
}
