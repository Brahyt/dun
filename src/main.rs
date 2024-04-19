use clap::Parser;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use dun::schema::tasks;
use crate::tasks::message;
use dun::models::{Task, NewTask};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    did: Option<String>,

    #[arg(short, long)]
    yesterday: bool
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DUN_DATABASE_URL").expect("DUN_DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let mut db = establish_connection();
    let args = Args::parse();

    if let Some(did) = args.did {
        let new_task = NewTask {message: &did};

        diesel::insert_into(tasks::table)
            .values(&new_task)
            .returning(Task::as_returning())
            .get_result::<Task>(&mut db)
            .expect("Error saving new post");

        println!("It appears that you did {}", did);
    } else if args.yesterday {
        println!("Yesterdays events");
        // Print out yesterdays events
    } else {
        println!("Gotta put something");
    }
}
