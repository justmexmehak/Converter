use crate::traits::{CheckValidRange, Convert};

pub struct  Temperature {
    pub input_value: f64,
    input_unit: TemperatureUnit,
    pub output_unit: TemperatureUnit,
}

impl Temperature {

    pub fn new(value: f64) -> Result<Self, ()> {
        if !Self::is_valid(value) {
            return Err(())
        }
        Ok(Self {
            input_value: value,
            input_unit: TemperatureUnit::UNDECIDED,
            output_unit: TemperatureUnit::UNDECIDED
        })
    }

    pub fn set_input_unit(& mut self, unit: TemperatureUnit) {
        self.input_unit = unit;
    }

    pub fn set_output_unit(& mut self, unit: TemperatureUnit) {
        self.output_unit = unit;
    }

    fn cel_to_far(magnitude: f64) -> f64 {
        magnitude * 9.0 / 5.0 + 32.0
    }

    fn far_to_cel(magnitude: f64) -> f64 {
        (magnitude - 32.0) * 5.0 / 9.0
    }

    fn cel_to_kel(magnitude: f64) -> f64 {
        magnitude - 273.15
    }

    fn kel_to_cel(magnitude: f64) -> f64 {
        magnitude + 273.15
    }
}

impl CheckValidRange for Temperature {
    fn is_valid(value: f64) -> bool {
        if value >= f64::MIN && value <= f64::MAX { return true }
        false
    }
}

impl Convert for Temperature {
    fn convert(&self) -> f64 {
        let si_value: f64;

        // convert to SI
        si_value = match self.input_unit {
            TemperatureUnit::CELSIUS => self.input_value,
            TemperatureUnit::FARENHEIT => Self::far_to_cel(self.input_value),
            TemperatureUnit::KELVIN => Self::kel_to_cel(self.input_value),
            TemperatureUnit::UNDECIDED => panic!("Cannot convert undecided!")
        };

        // Convert from SI to output unit
        match self.output_unit {
            TemperatureUnit::CELSIUS => si_value,
            TemperatureUnit::FARENHEIT => Self::cel_to_far(si_value),
            TemperatureUnit::KELVIN => Self::cel_to_kel(si_value),
            TemperatureUnit::UNDECIDED => panic!("Cannot convert undecided!")
        }
    }
}

#[derive(Debug)]
pub enum TemperatureUnit {
    CELSIUS, 
    KELVIN,
    FARENHEIT,
    UNDECIDED,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // negative value for weight
    fn test_negative_temp() {
        let inp = -25.4;
        match Temperature::new(inp) {
            Ok(temp) => assert_eq!(temp.input_value, inp),
            Err(()) => panic!("Temperature can be negative"),
        }
    }

    // have the input and output unit be same
    #[test]
    fn same_units() {
        let mut x = Temperature::new(37.2).unwrap();
        x.set_input_unit(TemperatureUnit::CELSIUS);
        x.set_output_unit(TemperatureUnit::CELSIUS);
        assert_eq!(x.convert(), 37.2);
    }
}


