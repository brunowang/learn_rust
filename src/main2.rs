use crate::api::Stock;
use api::Prods;
use models::Book;
use models::Phone;

mod api;
mod models;

fn show_prods_price_sum(b: &[&Book], p: &[&Phone]) {
    let mut sum: f32 = 0.0;
    for prod in b.iter() {
        sum += prod.get_discount_price(0.8);
    }
    for prod in p.iter() {
        sum += prod.get_discount_price(0.95);
    }
    println!("The product's price sum is: {}", sum)
}

fn show_detail<T>(p: &T)
where
    T: Prods + Stock,
{
    println!("book's price: {:?}", p.get_discount_price(1.0));
    println!("book's stock: {:?}", p.get_stock());
}

fn main() {
    let book: Book = Prods::new(101, 25.0);
    let book2: Book = Prods::new(102, 30.0);
    let phone: Phone = Prods::new(201, 2799.0);
    let phone2: Phone = Prods::new(202, 1399.0);
    show_prods_price_sum(&[&book, &book2], &[&phone, &phone2]);
    show_detail(&book);
    println!("{:?}, {:?}", book, book2);
    println!("{:?}, {:?}", phone, phone2);
    println!("total origin price: {}", book + book2);
}
