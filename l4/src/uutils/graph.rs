#![allow(dead_code)]

pub trait Calculated {
    fn area(&self) -> f32;
}

#[derive(Debug)]
pub struct Triangle {
    length: f32,
    height: f32,
}

impl Calculated for Triangle {
    fn area(&self) -> f32 {
        (self.height * self.length) / 2.0
    }
}

impl Triangle {
    pub fn new(length: f32, height: f32) -> Self {
        Self { length, height }
    }
}

#[derive(Debug)]
pub struct Circular {
    radius: f32,
}

impl Calculated for Circular {
    fn area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
    }
}

impl Circular {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

#[derive(Debug)]
pub struct Square {
    side_length: f32,
}

impl Calculated for Square {
    fn area(&self) -> f32 {
        self.side_length * self.side_length
    }
}

impl Square {
    pub fn new(side_length: f32) -> Self {
        Self { side_length }
    }
}

pub fn print_area<T>(obj: &T)
where
    T: Calculated,
{
    println!("{} area:{}", std::any::type_name::<T>(), obj.area());
}
