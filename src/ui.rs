pub fn header() {
    println!("\n\t\t\t-----------------------------------------------------");
    println!("\t\t\t|                                                   |");
    println!("\t\t\t|                 UNIT CONVERTER                    |");
    println!("\t\t\t|                                                   |");
    println!("\t\t\t-----------------------------------------------------");
}

pub fn category_prompt() {
    println!("\n\n\t\t\t\tSelect Category: ");
    println!("\t\t\t\tPress 1 for Temperature");
    println!("\t\t\t\tPress 2 for Weight");
    println!("\t\t\t\tPress 3 for Length");
}

pub fn temperature_units_prompt() {
    println!("\n\t\t\t\tPress 1 for Celsius: ");
    println!("\t\t\t\tPress 2 for Farenheit: ");
    println!("\t\t\t\tPress 3 for Kelvin: ");
}

pub fn length_units_prompt() {
    println!("\n\t\t\t\tPress 1 for Meters: ");
    println!("\t\t\t\tPress 2 for Miles: ");
    println!("\t\t\t\tPress 3 for Inches: ");
    println!("\t\t\t\tPress 4 for Feet: ");
}

pub fn weight_units_prompt() {
    println!("\n\t\t\t\tPress 1 for Kilograms: ");
    println!("\t\t\t\tPress 2 for Pounds: ");
}