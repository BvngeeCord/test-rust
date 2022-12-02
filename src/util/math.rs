pub struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Self {
        Self { numerator, denominator, }
    }
    
    pub fn as_float(&self) -> f32 {
        self.numerator as f32 / self.denominator as f32
    }
}


pub enum Measurement {
    Fraction(Fraction),
    Fixed(u32),
}
