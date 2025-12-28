pub struct Allergies {
    score: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Allergen {
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
    pub fn value(&self) -> u8 {
        *self as u8
    }

    pub fn all() -> [Allergen; 8] {
        [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ]
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score: Self::convert_to_u8(&score),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & allergen.value() != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let score = self.score;
        let mut list: Vec<Allergen> = Vec::new();
        for aller in Allergen::all().iter() {
            if aller.value() & score != 0 {
                list.push(*aller);
            }
        }

        list
    }

    fn convert_to_u8(number: &u32) -> u8 {
        u8::from_str_radix(&format!("{:08b}", *number as u8), 2).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn not_allergic_to_anything_eggs() {
        let allergies = Allergies::new(0);

        assert!(!allergies.is_allergic_to(&Allergen::Eggs))
    }

    #[test]
    fn allergic_only_to_eggs_eggs() {
        let allergies = Allergies::new(1);

        assert!(allergies.is_allergic_to(&Allergen::Eggs))
    }

    #[test]
    fn allergic_to_eggs_and_something_else_eggs() {
        let allergies = Allergies::new(3);

        assert!(allergies.is_allergic_to(&Allergen::Eggs))
    }

    #[test]
    fn allergic_to_something_but_not_eggs_eggs() {
        let allergies = Allergies::new(2);

        assert!(!allergies.is_allergic_to(&Allergen::Eggs))
    }

    #[test]
    fn allergic_to_everything_eggs() {
        let allergies = Allergies::new(255);

        assert!(allergies.is_allergic_to(&Allergen::Eggs))
    }

    #[test]
    fn not_allergic_to_anything_peanuts() {
        let allergies = Allergies::new(0);
        assert!(!allergies.is_allergic_to(&Allergen::Peanuts))
    }
}
