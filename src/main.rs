use chrono::{Duration, Local};
use clap::Parser;
use diesel::prelude::*;
use dun::models::{NewTask, Task};
use dun::schema::tasks;
use dun::schema::tasks::created_at;
use dun::schema::tasks::message;
use dun::database::database_connection::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    did: Option<String>,

    #[arg(short, long)]
    yesterday: bool,
}

fn main() {
    let mut db = establish_connection();
    let args = Args::parse();

    if let Some(did) = args.did {
        let new_task = NewTask { message: &did };

        diesel::insert_into(tasks::table)
            .values(&new_task)
            .returning(Task::as_returning())
            .get_result::<Task>(&mut db)
            .expect("Error saving new post");

        println!("It appears that you did {}", did);
    } else if args.yesterday {
        let today = Local::today().naive_local().and_hms(0, 0, 0);
        let yesterday = today - Duration::days(1);
        let tomorrow = today + Duration::days(1);

        let task_messages = tasks::table
            .select(message)
            .filter(created_at.ge(yesterday).and(created_at.lt(tomorrow)))
            .load::<String>(&mut db)
            .expect("Error");

        println!("{:?}", task_messages);
        // Print out yesterdays events
    } else {
        println!("Gotta put something");
    }
}
