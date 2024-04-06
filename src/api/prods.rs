use crate::models::book::Book;

pub trait Prods {
    fn get_discount_price(&self, discount: f32) -> f32;
}

impl Prods for Book {
    fn get_discount_price(&self, discount: f32) -> f32 {
        self.price * discount
    }
}
