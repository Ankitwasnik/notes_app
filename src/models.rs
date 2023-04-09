use std::fmt;

use diesel::prelude::*;

use crate::schema::notes;

#[derive(Queryable)]
pub struct Note {
    pub id: i32,
    pub text: String,
    pub published: bool,
}

impl std::fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Id:{}, Text:{}, Published:{})",
            self.id, self.text, self.published
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = notes)]
pub struct NewNote<'a> {
    pub text: &'a str,
}
