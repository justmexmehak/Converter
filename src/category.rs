use crate::length::Length;
use crate::weight::Weight;
use crate::temperature::Temperature;

pub enum Category {
    TEMPERATURE(Temperature),
    WEIGHT(Weight),
    LENGTH(Length),
}