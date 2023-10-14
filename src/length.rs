use crate::traits::{Convert, CheckValidRange};
extern crate approx;
pub struct Length {
    pub input_value: f64,
    input_unit: LengthUnit,
    pub output_unit: LengthUnit,
}

impl Length {

    // from unit to SI
    const MILES_TO_METRES: f64 = 1609.34;
    const INCHES_TO_METRES: f64 = 0.0254;
    const FEET_TO_METRES: f64 = 0.3048;

    // from SI to unit
    const METRES_TO_MILES: f64 = 1.0 / 1609.34;
    const METRES_TO_INCHES: f64 = 39.37;
    const METRES_TO_FEET: f64 = 3.281;

    pub fn new(value: f64) -> Result<Self, ()> {
        if !Self::is_valid(value) {
            return Err(())
        }
        Ok(Self {
            input_value: value,
            input_unit: LengthUnit::UNDECIDED,
            output_unit: LengthUnit::UNDECIDED
        })
    }

    pub fn set_input_unit(& mut self, unit: LengthUnit) {
        self.input_unit = unit;
    }

    pub fn set_output_unit(& mut self, unit: LengthUnit) {
        self.output_unit = unit;
    }
}

impl Convert for Length {
    fn convert(&self) -> f64 {
        let si_value: f64;

        // convert to SI
        si_value = match self.input_unit {
            LengthUnit::METRES => self.input_value,
            LengthUnit::MILES => &self.input_value * Self::MILES_TO_METRES,
            LengthUnit::INCHES => &self.input_value * Self::INCHES_TO_METRES,
            LengthUnit::FEET => &self.input_value * Self::FEET_TO_METRES,
            LengthUnit::UNDECIDED => panic!("Cannot convert undecided!")
        };

        // Convert from SI to output unit
        match self.output_unit {
            LengthUnit::METRES => si_value,
            LengthUnit::MILES => si_value * Self::METRES_TO_MILES,
            LengthUnit::INCHES => si_value * Self::METRES_TO_INCHES,
            LengthUnit::FEET => si_value * Self::METRES_TO_FEET,
            LengthUnit::UNDECIDED => panic!("Cannot convert undecided!")
        }
    }
}

impl CheckValidRange for Length {}

#[derive(Debug)]
pub enum LengthUnit {
    METRES,
    MILES,
    INCHES,
    FEET,
    UNDECIDED,
}

#[cfg(test)]
mod tests {
    use super::*;

    // si to another unit
    #[test]
    fn metres_to_miles() {
        let mut x = Length::new(37.2).unwrap();
        x.set_input_unit(LengthUnit::METRES);
        x.set_output_unit(LengthUnit::MILES);
        //assert_relative_eq!(x.convert(), 37.2 * 1 / 1609.34, 0.01);
        assert_eq!(x.convert() as i64, (37.2 * 1.0 / 1609.34) as i64);
    }

    // another unit to si 
    #[test]
    fn miles_to_metres() {
        let mut x = Length::new(37.2).unwrap();
        x.set_input_unit(LengthUnit::MILES);
        x.set_output_unit(LengthUnit::METRES);
        //assert_approx_eq!(x.convert(), 37.2 * 1609.34, 0.01);
        assert_eq!(x.convert() as i64, (37.2 * 1609.34) as i64);
    }

    #[test]
    // multiple conversions
    fn inches_to_feet() {
        let mut x = Length::new(64.0).unwrap();
        x.set_input_unit(LengthUnit::INCHES);
        x.set_output_unit(LengthUnit::FEET);

        assert_eq!(x.convert() as i64, (x.input_value / 12.0) as i64);
    }

    // convert without specifying units
    #[should_panic]
    #[test]
    fn convert_undecided() {
        let x = Length::new(37.2).unwrap();
        x.convert();
    }

    #[should_panic]
    #[test]
    // negative value for length
    fn test_negative_length() {
        let inp = -25.4;
        match Length::new(inp) {
            Ok(length) => assert_eq!(length.input_value, inp),
            Err(()) => panic!("Length cannot be negative"),
        }
    }
}