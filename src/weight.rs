use crate::traits::{CheckValidRange, Convert};

pub struct Weight {
    pub input_value: f64,
    input_unit: WeightUnit,
    pub output_unit: WeightUnit,
}

impl Weight {

    const POUNDS_TO_KILOGRAMS: f64 = 0.453592;
    const KILOGRAMS_TO_POUNDS: f64 = 2.20462;

    pub fn new(value: f64) -> Result<Self, ()> {
        if !Self::is_valid(value) {
            return Err(())
        }
        Ok(Self {
            input_value: value,
            input_unit: WeightUnit::UNDECIDED,
            output_unit: WeightUnit::UNDECIDED
        })
    }

    pub fn set_input_unit(& mut self, unit: WeightUnit) {
        self.input_unit = unit;
    }

    pub fn set_output_unit(& mut self, unit: WeightUnit) {
        self.output_unit = unit;
    }
}

impl CheckValidRange for Weight {}

impl Convert for Weight {
    fn convert(&self) -> f64 {
        let si_value: f64;

        // convert to SI
        si_value = match self.input_unit {
            WeightUnit::KILOGRAMS => self.input_value,
            WeightUnit::POUNDS => self.input_value * Self::POUNDS_TO_KILOGRAMS,
            WeightUnit::UNDECIDED => panic!("Cannot convert undecided!")
        };

        // Convert from SI to output unit
        match self.output_unit {
            WeightUnit::KILOGRAMS => si_value,
            WeightUnit::POUNDS => si_value * Self::KILOGRAMS_TO_POUNDS,
            WeightUnit::UNDECIDED => panic!("Cannot convert undecided!")
        }
    }
}

#[derive(Debug)]
pub enum WeightUnit {
    KILOGRAMS,
    POUNDS,
    UNDECIDED,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    #[test]
    // negative value for weight
    fn test_negative_weight() {
        let inp = -25.4;
        match Weight::new(inp) {
            Ok(weight) => assert_eq!(weight.input_value, inp),
            Err(()) => panic!("Weight cannot be negative"),
        }
    }
}
