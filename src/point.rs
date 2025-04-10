use std::{fmt::Display, ops::Add};

use crate::field_element::FieldElement;

pub struct Point {
    x: FieldElement,
    y: FieldElement,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(x:FieldElement, y: FieldElement, a: FieldElement, b: FieldElement) -> Self {
        // if y.clone().pow(2) != (x.clone().pow(3) + a.clone() * x.clone() + b.clone()) {
        //     panic!("({}, {}) is not on the curve.", x.clone(), y.clone());
        // }
        Self{
            x,
            y,
            a,
            b,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        ((self.x == other.x) & (self.y == other.y)) || ((self.a == other.a) & (self.b == other.b))
    }

    fn ne(&self, other: &Self) -> bool {
        ((self.x != other.x) || (self.y != other.y)) || ((self.a != other.a) || (self.b != other.b))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f;
        print!("Point({},{})_{}_{}", self.x, self.y, self.a, self.b);
        Ok(())
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let _self_at_infinity = (self.x.num) == 0 && (self.y.num == 0);
        let _rhs_at_infinity = (rhs.x.num ==0) && (rhs.y.num == 0);

        // Points have same x, but opposite y
        let _add_inverse = (rhs.x.num == self.x.num) && (self.y.num == rhs.y.num*-1);

        // Same point
        let _tangent = self == rhs;
        let _tangent_at_infinity = _tangent && (self.y.num == 0);

        if _self_at_infinity {
            return rhs;
        } else if _rhs_at_infinity {
            return self;
        } else if _add_inverse {
            // Sum at infinity
            return Self {
                x: FieldElement::new(self.x.prime, 0),
                y: FieldElement::new(self.x.prime, 0),
                a: self.a,
                b: self.b,
            };
        } else if _tangent {
            let x_1 = self.x.clone();
            let y_1 = self.y.clone();
            let a = self.a.clone();
            let prime = self.x.prime;

            let _3 = FieldElement::new(prime, 3);
            let _2 = FieldElement::new(prime, 2);
            let s = (_3*self.x.pow(2)+a.clone())/(_2.clone()*self.y);

            let x_3 = s.clone().pow(2) - _2.clone()*x_1.clone();
            let y_3 = s.clone()*(x_1.clone()-x_3.clone())-y_1.clone();

            return Self {
                x: x_3,
                y: y_3,
                a: self.a,
                b: self.b,
            };
        } else if _tangent_at_infinity {
            // Sum at infinity
            return Self {
                x: FieldElement::new(self.x.prime, 0),
                y: FieldElement::new(self.x.prime, 0),
                a: self.a,
                b: self.b,
            }
        }
         else {
            // Normally
            let s = (rhs.y.num-self.y.num)/(rhs.x.num-self.x.num);
            let x_3 = s.pow(2)-(self.x.num+rhs.x.num);
            let y_3 = s*(self.x.num-x_3)-self.y.num;

            return Self {
                x: FieldElement::new(self.x.prime, x_3),
                y: FieldElement::new(self.y.prime, y_3),
                a: self.a,
                b: self.b,
            }
        }
    }
}