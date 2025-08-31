use rand::Rng;

pub struct Dice {
    sides: u8,
}

impl Dice {
    pub fn new(sides: u8) -> Self {
        Dice { sides }
    }
    
    pub fn roll(&self) -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=self.sides)
    }
}

impl Default for Dice {
    fn default() -> Self {
        Self::new(6) // Standard 6-sided dice
    }
}