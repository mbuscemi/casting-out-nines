use std::{collections::HashMap, convert::TryInto};

pub struct Digits {
    digits: Vec<i8>,
    frequency: HashMap<i8, u64>,
}

impl Digits {
    pub fn new(number: i64) -> Self {
        let digits = to_digits(number);
        let frequency = to_frequency(&digits);

        Digits { 
            digits,
            frequency,
        }
    }

    pub fn get(self) -> Vec<i8> {
        self.digits
    }

    pub fn get_frequency(self) -> HashMap<i8, u64> {
        self.frequency
    }
}

fn to_digits(number: i64) -> Vec<i8> {
    let mut n: i64 = number;
    let mut digits: Vec<i8> = Vec::new();

    if n == 0 {
        digits.push(0);
    }

    while n > 0 {
        let ones_place = (n % 10).try_into().unwrap();
        digits.push(ones_place);
        n /= 10;
    }

    digits.reverse();
    digits
}

fn to_frequency(digits: &Vec<i8>) -> HashMap<i8, u64> {
    let mut map  = HashMap::new();

    for i in 0i8..=9i8 {
        map.insert(i, 0);
    }

    for n in digits {
        let entry = map.entry(*n).or_insert(0);
        *entry += 1;
    }

    map
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;
    
    #[test]
    fn zero__yields_single_zero() {
        let zero = Digits::new(0);
        assert_eq!(zero.get(), vec![0]);
    }

    #[test]
    fn single_digit_numbers__yield_single_value_vector() {
        let one = Digits::new(1);
        let two = Digits::new(2);
        let three = Digits::new(3);
        let six = Digits::new(6);
        let nine = Digits::new(9);

        assert_eq!(one.get(), vec![1]);
        assert_eq!(two.get(), vec![2]);
        assert_eq!(three.get(), vec![3]);
        assert_eq!(six.get(), vec![6]);
        assert_eq!(nine.get(), vec![9]);
    }

    #[test]
    fn double_digit_numbers__yield_correct_double_value_vector() {
        let fifteen = Digits::new(15);
        let thirty_three = Digits::new(33);
        let forty_eight = Digits::new(48);
        let ninety_seven = Digits::new(97);

        assert_eq!(fifteen.get(), vec![1, 5]);
        assert_eq!(thirty_three.get(), vec![3, 3]);
        assert_eq!(forty_eight.get(), vec![4, 8]);
        assert_eq!(ninety_seven.get(), vec![9, 7]);
    }

    #[test]
    fn triple_digit_numbers__yield_correct_triple_value_vector() {
        let x = Digits::new(549);
        let y = Digits::new(761);

        assert_eq!(x.get(), vec![5, 4, 9]);
        assert_eq!(y.get(), vec![7, 6, 1]);
    }

    #[test]
    fn really_large_number__yields_correct_value_vector() {
        let x = Digits::new(5462895035);

        assert_eq!(x.get(), vec![5, 4, 6, 2, 8, 9, 5, 0, 3, 5]);
    }

    #[test]
    fn really_large_number__yields_correct_frequency_map() {
        let x = Digits::new(5428950354);
        let frequency = x.get_frequency();

        assert_eq!(*frequency.get(&0).unwrap(), 1);
        assert_eq!(*frequency.get(&1).unwrap(), 0);
        assert_eq!(*frequency.get(&2).unwrap(), 1);
        assert_eq!(*frequency.get(&3).unwrap(), 1);
        assert_eq!(*frequency.get(&4).unwrap(), 2);
        assert_eq!(*frequency.get(&5).unwrap(), 3);
        assert_eq!(*frequency.get(&6).unwrap(), 0);
        assert_eq!(*frequency.get(&7).unwrap(), 0);
        assert_eq!(*frequency.get(&8).unwrap(), 1);
        assert_eq!(*frequency.get(&9).unwrap(), 1);
    }
}