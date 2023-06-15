#![allow(dead_code)]

use std::ops::{Add, Sub};

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.x + other.y,
        }
    }
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.x - other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 2.0, y: 3.0 };
    let p2 = Point { x: 1.0, y: 2.0 };

    let sum = &p1 + &p2;
    let diff = &p1 - &p2;

    println!("Sum: {:?}", sum);
    println!("Difference: {:?}", diff);
}