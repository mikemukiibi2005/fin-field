use crate::{field_element::FieldElement, point::Point};

pub fn ex_1_5() {
    for k in [1,2,7,13,18] {
        println!("The elements of the set when k = {} are: ", k);

        for num in 0..=18 {
            let fe = FieldElement::new(19, num);
            println!("{} * {} is {}", k, num, fe*FieldElement::new(19, k));
        }
    }
}

pub fn ex_1_4() {
    let _95 = FieldElement::new(97, 95);
    let _45 = FieldElement::new(97, 45);
    let _31 = FieldElement::new(97, 31);
    let product = _95*_45*_31;
    println!("{}", product);

    let _17 = FieldElement::new(97, 17);
    let _13 = FieldElement::new(97, 13);
    let _19 = FieldElement::new(97, 19);
    let product = _17*_13*_19;
    println!("{}", product);

    let _12 = FieldElement::new(97, 12);
    let _77 = FieldElement::new(97, 77);
    let _12_pow_7 = _12.pow(7);
    let _77_pow_49 = _77.pow(49);
    let product = _12_pow_7*_77_pow_49;
    println!("{}", product);
}

pub fn ex_1_2() {
    let _44 = FieldElement::new(57, 44);
    let _33 = FieldElement::new(57, 33);
    let sum = _44 + _33;
    println!("{}", sum);

    let _9 = FieldElement::new(57, 9);
    let _29 = FieldElement::new(57, 29);
    let diff = _9 -_29;
    println!("{}", diff);

    let _17 = FieldElement::new(57, 17);
    let _42 = FieldElement::new(57, 42);
    let _49 = FieldElement::new(57, 49);
    let sum = _17 + _42 + _49;
    println!("{}", sum);

    let _52 = FieldElement::new(57, 52);
    let _30 = FieldElement::new(57, 30);
    let _38 = FieldElement::new(57, 38);
    let sum = _52 - _30 - _38;
    println!("{}", sum);
}

// Check if point is on the curve for: a = 5, b =7
pub fn ex_2_1() {
    for p in [(2,4),(-1,-1),(18,77),(5,7)] {
        let a = 5;
        let b = 7;
        let x: i32 = p.0;
        let y: i32 = p.1;
        let off_curve = y.pow(2) != (x.pow(3)+a*x+b);
        if off_curve {
            println!("{:?} is not on the curve.", p);
        } else {
            println!("{:?} is on the curve.", p);
        }
    }
}

// Evaluate (2,5) + (-1, -1) for a: 5, b: 7
pub fn ex_2_4() {
    let p1 = (2,5);
    let p2 = (-1,-1);

    // Slope, s:(y2-y1)/(x2-x1)
    let  s: i32 = (p2.1-p1.1)/(p2.0-p1.0);
    let x3 = s.pow(2) - (p1.0+p2.0);
    let y3 = s*(p1.0-x3)-p1.1;
    let p3 = (x3,y3);
    println!("{:?} + {:?} = {:?}", p1, p2, p3);
}

// Prove that points lie on the curve, a: 0, b: 7, P: 223
pub fn ex_3_1() {
    for p in [(192,105),(17,56),(200,119), (1,193),(42,99)] {
        let b = 7;
        let order = 223;

        let x = FieldElement::new(order, p.0);
        let y = FieldElement::new(order, p.1);
        let b = FieldElement::new(order, b);

        let off_curve = y.pow(2) != x.pow(3) + b;

        if off_curve {
            println!("{:?} is off the curve.", p);
        } else {
            println!("{:?} is on the curve.", p);
        }
    }
}

// Point addition
pub fn ex_3_2() {
    let pairs = [
        ((170, 142),(60, 139)),
        ((47, 71),(17,56)),
        ((143, 98),(76,66)),
    ];

    let a = FieldElement::new(223, 0);
    let b = FieldElement::new(223, 7);

    for p in pairs {
        let point_1 = Point::new(
            FieldElement::new(223, p.0.0), 
            FieldElement::new(223, p.0.1), 
            a.clone(),
            b.clone(),
        );

        let point_2 = Point::new(
            FieldElement::new(223, p.1.0), 
            FieldElement::new(223, p.1.1), 
            a.clone(),
            b.clone(),
        );

        let sum = point_1 + point_2;

        println!("{:?} + {:?} = {}", p.0, p.1, sum);
    }
}