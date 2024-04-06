#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub price: f32,
}

impl std::ops::Add for Book {
    type Output = f32;

    fn add(self, rhs: Self) -> Self::Output {
        self.price + rhs.price
    }
}
