use diesel::{Connection, PgConnection, RunQueryDsl};
use dotenvy::dotenv;
use models::{NewNote, Note};
use schema::notes;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_db_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Please provide database URL");
    let conn_res = PgConnection::establish(&database_url);
    match conn_res {
        Ok(conn) => {
            println!("Connection to database:{} is successful", database_url);
            return conn;
        }
        Err(e) => {
            println!("Error:{}", e);
            panic!("Could not connect to database:{}", database_url);
        }
    }
}

pub fn create_note(connection: &mut PgConnection, text: &str) {
    let note = NewNote { text };
    diesel::insert_into(notes::table)
        .values(note)
        .get_result::<Note>(connection)
        .expect("Error saving note");
}
