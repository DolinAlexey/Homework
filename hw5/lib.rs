// Перенесены все общие операции в трейт.
pub trait NumericSet {
    fn default_values() -> Self;
    fn get_item(&self, item: Item) -> f64;
    fn set_item(&mut self, item: Item, value: f64);
    // Методы sum и is_default реализованы только один раз - в самом трейте.
    fn is_default(&self) -> bool {
        self.get_item(Item::First) == 0.0
            && self.get_item(Item::Second) == 0.0
            && self.get_item(Item::Third) == 0.0
    }
    fn sum(&self) -> f64 {
        self.get_item(Item::First) + self.get_item(Item::Second) + self.get_item(Item::Third)
    }
}

pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}
// Реализован трейт на обоих типах
pub struct Tuple(u32, f32, f64);

impl NumericSet for Tuple {
    fn default_values() -> Self {
        Tuple(0, 0.0, 0.0)
    }

    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as f64,
            Item::Second => self.1 as f64,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as u32,
            Item::Second => self.1 = value as f32,
            Item::Third => self.2 = value,
        };
    }
}

pub struct Array([f64; 3]);

impl NumericSet for Array {
    fn default_values() -> Self {
        Array([0.0; 3])
    }

    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value;
    }
}
//  обобщённая тестовая логика, которая будет работать для обоих типов
#[cfg(test)]
mod tests {
    use super::*;

    fn test_numeric_set<N: NumericSet>() {
        let default_values = N::default_values();
        assert!(default_values.is_default());

        let mut instance = N::default_values();
        instance.set_item(Item::First, 10.0);
        instance.set_item(Item::Second, 20.0);
        instance.set_item(Item::Third, 30.0);

        assert_eq!(instance.sum(), 60.0);
        assert!(!instance.is_default());
    }

    #[test]
    fn test_tuple() {
        test_numeric_set::<Tuple>();
    }

    #[test]
    fn test_array() {
        test_numeric_set::<Array>();
    }
}

