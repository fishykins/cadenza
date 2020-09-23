use crate::maths::Frequency;

pub trait Temperament {
    fn ratio(&self, tone: i16) -> f32;
    fn frequency(&self, tone: i16) -> Frequency;
}