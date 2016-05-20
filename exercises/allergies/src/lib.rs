
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum  Allergen  {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    fn from_value(value: u32) -> Allergen {
        match value {
            1 => Allergen::Eggs,
            2 => Allergen::Peanuts,
            4 => Allergen::Shellfish,
            8 => Allergen::Strawberries,
            16 => Allergen::Tomatoes,
            32 => Allergen::Chocolate,
            64 => Allergen::Pollen,
            128 => Allergen::Cats,
            _ => panic!("unknown value {}", value)
        }
    }
}

pub struct Allergies {
    score: u32
}


impl Allergies {
    pub fn new(score: u32) -> Allergies {
        let allergies = Allergies { score: score };
        return allergies;
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let allergen_value = *allergen as u32;
        return self.is_allergic_to_value(allergen_value);
    }

    fn is_allergic_to_value(&self, allergen_value: u32) -> bool {
        return self.score & allergen_value == allergen_value;
    }
    
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergies : Vec<Allergen> = vec![];
        let mut allergen_value = 1;
        for _ in 0..8 {
            if self.is_allergic_to_value(allergen_value) {
                allergies.push(Allergen::from_value(allergen_value));
            }
            allergen_value *= 2;
        }
        return allergies;
    }
}
