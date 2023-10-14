mod temperature;
mod weight;
mod traits;
mod length;
mod user_input;
mod category;
mod ui;

use clearscreen;
use ui::*;
use user_input::*;
use category::Category;

use crate::traits::Convert;

fn main() {

    clearscreen::clear().unwrap();
    header();

    category_prompt();

    let category = get_category_input();

    clearscreen::clear().unwrap();
    header();

    match category {

        Category::LENGTH(mut length) => {
            println!("\n\t\t\t\tLength: {}", length.input_value);
            length_units_prompt();
            println!("\n\t\t\t\tInput Unit: ");
            length.set_input_unit(get_length_unit());
            println!("\n\t\t\t\tOutput Unit: ");
            length.set_output_unit(get_length_unit());
            println!("\n\t\t\t\tResult: {} {:?}", length.convert(), length.output_unit);
        },

        Category::TEMPERATURE(mut temperature) => {
            println!("\n\t\t\t\tTemperature: {}", temperature.input_value);
            temperature_units_prompt();
            println!("\n\t\t\t\tInput Unit: ");
            temperature.set_input_unit(get_temperature_unit());
            println!("\n\t\t\t\tOutput Unit: ");
            temperature.set_output_unit(get_temperature_unit());
            println!("\n\t\t\t\tResult: {} {:?}", temperature.convert(), temperature.output_unit);
        },

        Category::WEIGHT(mut weight) => {
            println!("\n\t\t\t\tWeight: {}", weight.input_value);
            weight_units_prompt();
            println!("\n\t\t\t\tInput Unit: ");
            weight.set_input_unit(get_weight_unit());
            println!("\n\t\t\t\tOutput Unit: ");
            weight.set_output_unit(get_weight_unit());
            println!("\n\t\t\t\tResult: {} {:?}", weight.convert(), weight.output_unit);
        }

    }


}



