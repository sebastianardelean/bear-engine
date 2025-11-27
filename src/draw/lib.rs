use std::sync::OnceLock;
use crate::draw::Shape2D;

pub trait Shape {
    fn draw(&mut self);

 
}