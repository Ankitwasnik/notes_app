use std::io;

use self::models::*;
use diesel::*;
use notes_app::*;
use schema::notes;

pub mod models;
pub mod schema;

fn main() {
    use crate::notes::dsl::notes;

    let connection = &mut establish_db_connection();
    loop {
        println!("");
        println!("1: Save a note");
        println!("2: Get all notes");
        println!("Any other key: Quit");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Could not get input");

        match option.trim() {
            "1" => {
                println!("Option 1 selected");
                println!("Enter your note");
                let mut note = String::new();
                io::stdin()
                    .read_line(&mut note)
                    .expect("Could not get input");

                create_note(connection, &note.trim());
            }
            "2" => {
                let all_notes: Vec<Note> = notes
                    .load(connection)
                    .expect("Could not get notes from table");

                for note in all_notes {
                    println!("Note:{note}");
                }
            }
            _ => {
                break;
            }
        }
    }
}
