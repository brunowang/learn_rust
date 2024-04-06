use crate::models::book::Book;

pub trait Prods {
    fn new(id: i32, price: f32) -> Self;
    fn get_discount_price(&self, discount: f32) -> f32;
}

impl Prods for Book {
    fn new(id: i32, price: f32) -> Self {
        Book { id, price }
    }
    fn get_discount_price(&self, discount: f32) -> f32 {
        self.price * discount
    }
}
