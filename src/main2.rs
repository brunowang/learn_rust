use api::Prods;
use models::book::*;

mod api;
mod models;

fn main() {
    let book: Book = new_book(101, 25.0);
    println!("{:?}, {}", book, book.get_discount_price(0.8))
}
