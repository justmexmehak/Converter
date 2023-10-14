use std::io;
use crate::category::Category;
use crate::temperature::{Temperature, TemperatureUnit};
use crate::length::{Length, LengthUnit};
use crate::weight::{Weight, WeightUnit};

pub fn get_category_input() -> Category {
    match choice_input(1, 3) {
        1 => {
            Category::TEMPERATURE(get_temperature_input())
        },
        2 => {
            Category::WEIGHT(get_weight_input())
        },
        3 => {
            Category::LENGTH(get_length_input())
        },
        _ => panic!("Category does not exist"),
    }
}

fn get_temperature_input() -> Temperature {
    println!("\n\n\t\t\t\tTemperature: ");

    let mut inp_str = String::new();
    let mut inp: f64;

    loop {
        inp_str.clear();
        io::stdin().read_line(& mut inp_str).expect("Failed to read input");
        inp = inp_str.trim().parse().expect("Failed to convert to float");

        match Temperature::new(inp) {
            Ok(temperature) => return temperature,
            Err(()) => continue,
        }
    }
}

fn get_length_input() -> Length {
    println!("\n\t\t\t\tLength: ");

    let mut inp_str = String::new();
    let mut inp: f64;

    loop {
        inp_str.clear();
        io::stdin().read_line(& mut inp_str).expect("Failed to read input");
        inp = inp_str.trim().parse().expect("Failed to convert to float");

        match Length::new(inp) {
            Ok(length) => return length,
            Err(()) => continue,
        }
    }
}

fn get_weight_input() -> Weight {
    println!("\n\t\t\t\tWeight: ");

    let mut inp_str = String::new();
    let mut inp: f64;

    loop {
        inp_str.clear();
        io::stdin().read_line(& mut inp_str).expect("Failed to read input");
        inp = inp_str.trim().parse().expect("Failed to convert to float");

        match Weight::new(inp) {
            Ok(weight) => return weight,
            Err(()) => continue,
        }
    }
}

pub fn get_length_unit() -> LengthUnit {
    match choice_input(1, 4) {
        1 => LengthUnit::METRES,
        2 => LengthUnit::MILES,
        3 => LengthUnit::INCHES,
        4 => LengthUnit::FEET,
        _ => panic!("Unit does not exist"),
    }

}

pub fn get_weight_unit() -> WeightUnit {
    match choice_input(1, 2) {
        1 => WeightUnit::KILOGRAMS,
        2 => WeightUnit::POUNDS,
        _ => panic!("Unit does not exist"),
    }

}

pub fn get_temperature_unit() -> TemperatureUnit {
    match choice_input(1, 3) {
        1 => TemperatureUnit::CELSIUS,
        2 => TemperatureUnit::FARENHEIT,
        3 => TemperatureUnit::KELVIN,
        _ => panic!("Unit does not exist"),
    }
}

fn choice_input(start: u32, end: u32) -> u32 {
    let mut cat_str =  String::new();
    let mut cat: u32;

    loop {
        cat_str.clear();
        io::stdin().read_line(&mut cat_str).expect("Failed to read input");
        cat = cat_str.trim().parse().expect("Failed to convert to integer");

        match UserInput::new(cat, start, end) {
            Ok(user_input) => return user_input.value(),
            Err(()) => continue
        }
    }
}

struct UserInput {
    value: u32
}

impl UserInput {
    fn new(value: u32, start: u32, end: u32) -> Result<Self, ()> {
        if value < start || value > end {
            println!("Out of range. Category doesn't exist");
            return Err(())
        }
        Ok(UserInput { value })
    }

    fn value(&self) -> u32{
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_user_input() {
        match UserInput::new(2, 1, 3) {
            Ok(user_input) => assert_eq!(user_input.value, 2),
            Err(()) => panic!("Value out of bounds")
        }
    }

    #[should_panic]
    #[test]
    fn test_user_input_fail() {
        match UserInput::new(5, 1, 3) {
            Ok(user_input) => assert_eq!(user_input.value, 5),
            Err(()) => panic!("Value out of bounds")
        }
    }
}