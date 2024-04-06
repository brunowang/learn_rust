#[derive(Debug)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub age: u8,
    pub tags: [&'static str; 3],
    tag_idx: usize,
}

pub fn new_user_info(id: i32, name: String, age: u8) -> UserInfo {
    UserInfo {
        id,
        name,
        age,
        tags: [""; 3],
        tag_idx: 0,
    }
}

impl UserInfo {
    pub fn add_user_tag(&mut self, tag: &'static str) -> &mut UserInfo {
        self.tags[self.tag_idx] = tag;
        if self.tag_idx < self.tags.len() - 1 {
            self.tag_idx += 1;
        }
        self
    }
}

#[derive(Debug)]
pub struct UserScore<T, U> {
    pub id: T,
    pub score: U,
    pub comment: &'static str,
}

pub fn new_user_score<T, U>(id: T, score: U) -> UserScore<T, U> {
    UserScore {
        id,
        score,
        comment: "",
    }
}

impl<T, U> UserScore<T, U> {
    pub fn get_score(&self) -> &U {
        &self.score
    }
    pub fn get_comment(&self) -> &'static str {
        self.comment
    }
    pub fn set_comment(&mut self, comment: &'static str) -> &mut UserScore<T, U> {
        self.comment = comment;
        self
    }
}
