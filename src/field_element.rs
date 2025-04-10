use std::{fmt::Display, ops::{Add, Div, Mul, Sub}};

#[derive(Clone)]
pub struct FieldElement {
    pub num: i128,
    pub prime: i128,
}

impl FieldElement {
    pub fn new(prime:i128, num:i128) -> Self {
        Self {
            num,
            prime,
        }
    }

    pub fn pow(self, power:i128) -> Self {
        let num = mod_exp::mod_exp(self.num, power, self.prime);
        Self{
            prime: self.prime,
            num,
        }   
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        print!("FieldElement_{}({})", self.prime, self.num);
        Ok(())
    }
}

impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }

    fn ne(&self, other: &Self) -> bool {
        self.num != other.num || self.prime == other.prime
    }
}

impl Add for FieldElement {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Must be in the same order");
        }
        Self {
            num: (self.num + rhs.num) % self.prime,
            prime: self.prime,
        }
    }
}

impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Must be in the same order");
        }
        
        let mut diff = self.num - rhs.num;

        if diff.is_negative() {
            diff = self.prime+diff;

            return Self {
                num: diff%self.prime,
                prime: self.prime,
            }
        }

        Self {
            num: diff%self.prime,
            prime: self.prime,
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            num: (self.num * rhs.num) % self.prime,
            prime: self.prime,
        }
    }
}

impl Div for FieldElement {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let pow = self.prime.clone() - 2;
        let _rhs_pow = mod_exp::mod_exp(rhs.num, pow, rhs.prime);

        Self {
            prime: self.prime,
            num: (self.num * _rhs_pow) % self.prime.clone(),
        }
    }
}
