use chrono::{Duration, Local};
use clap::Parser;
use diesel::prelude::*;
use dun::models::{NewTask, Task};
use dun::schema::tasks;
use dun::schema::tasks::created_at;
use dun::schema::tasks::message;
use dun::database::database_connection::*;
use std::process::{Command, Stdio};
use std::io::Write;

fn format_for_claude(tasks: &[String]) {
    let prompt = format!(
        "Please format these tasks I completed yesterday into a nice summary for my daily standup. Here are the tasks: {:?}",
        tasks
    );
    
    let mut child = Command::new("claude")
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start claude command");
    
    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(prompt.as_bytes()).expect("Failed to write to claude stdin");
    }
    
    child.wait().expect("Failed to wait for claude command");
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    did: Option<String>,

    #[arg(short, long)]
    yesterday: bool,

    #[arg(short, long)]
    claude: bool,

    #[arg(short, long)]
    today: bool,
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

        println!("You did: {}", did);
    } else if args.today {
        let today = Local::now().naive_local().date();
        let today_start = today.and_hms_opt(0, 0, 0).unwrap();
        let tomorrow_start = (today + Duration::days(1)).and_hms_opt(0, 0, 0).unwrap();

        let task_messages = tasks::table
            .select(message)
            .filter(created_at.ge(today_start).and(created_at.lt(tomorrow_start)))
            .load::<String>(&mut db)
            .expect("Error");

        if args.claude {
            format_for_claude(&task_messages);
        } else {
            println!("{:?}", task_messages);
        }
    } else if args.yesterday {
        let today = Local::now().naive_local().date();
        let yesterday_start = (today - Duration::days(1)).and_hms_opt(0, 0, 0).unwrap();
        let today_start = today.and_hms_opt(0, 0, 0).unwrap();

        let task_messages = tasks::table
            .select(message)
            .filter(created_at.ge(yesterday_start).and(created_at.lt(today_start)))
            .load::<String>(&mut db)
            .expect("Error");

        if args.claude {
            format_for_claude(&task_messages);
        } else {
            println!("{:?}", task_messages);
        }
        // Print out yesterdays events
    } else {
        println!("Gotta put something");
    }
}
