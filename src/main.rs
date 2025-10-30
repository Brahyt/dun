use chrono::{Duration, Local, NaiveDateTime, TimeZone};
use clap::Parser;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dun::models::{NewTask, NewTaskWithDate, Task};
use dun::schema::tasks;
use dun::schema::tasks::created_at;
use dun::schema::tasks::message;
use dun::database::database_connection::*;
use std::process::{Command, Stdio};
use std::io::Write;

fn format_for_claude(tasks: &[String]) {
    let prompt = format!(
        "Please format these tasks I completed yesterday into a nice summary for my daily standup. If I surround words with **, example: *Team Sync*, I want you to use that in the formatted summary. Here are the tasks: {:?}",
        tasks
    );

    let mut child = Command::new("claude")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start claude command");

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(prompt.as_bytes()).expect("Failed to write to claude stdin");
    }

    let output = child.wait_with_output().expect("Failed to wait for claude command");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

#[derive(Debug)]
enum QueryMode {
    Add(String, Option<NaiveDateTime>),
    Today { use_claude: bool },
    Yesterday { use_claude: bool, days_back: u32 },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(short, long)]
    did: Option<String>,

    #[arg(short, long, default_missing_value = "1", num_args(0..=1))]
    yesterday: Option<u32>,

    #[arg(short, long)]
    claude: bool,

    #[arg(short, long)]
    today: bool,
}

fn determine_query_mode(args: &Args) -> Option<QueryMode> {
    if let Some(did) = &args.did {
        let custom_date = if let Some(days_back) = args.yesterday {
            let today = Local::now().naive_local().date();
            let target_date = today - Duration::days(days_back as i64);
            Some(target_date.and_hms_opt(12, 0, 0).unwrap()) // Use noon as default time
        } else {
            None
        };
        Some(QueryMode::Add(did.clone(), custom_date))
    } else if args.today {
        Some(QueryMode::Today { use_claude: args.claude })
    } else if let Some(days_back) = args.yesterday {
        Some(QueryMode::Yesterday { use_claude: args.claude, days_back })
    } else {
        None
    }
}

fn handle_query_mode(mode: QueryMode, db: &mut PgConnection) {
    match mode {
        QueryMode::Add(msg, custom_date) => {
            if let Some(date) = custom_date {
                let new_task = NewTaskWithDate {
                    message: &msg,
                    created_at: date,
                    updated_at: date,
                };

                diesel::insert_into(tasks::table)
                    .values(&new_task)
                    .returning(Task::as_returning())
                    .get_result::<Task>(db)
                    .expect("Error saving new post");

                let days_back = (Local::now().naive_local().date() - date.date()).num_days();
                println!("You did {} days ago: {}", days_back, msg);
            } else {
                let new_task = NewTask { message: &msg };

                diesel::insert_into(tasks::table)
                    .values(&new_task)
                    .returning(Task::as_returning())
                    .get_result::<Task>(db)
                    .expect("Error saving new post");

                println!("You did: {}", msg);
            }
        }
        QueryMode::Today { use_claude } => {
            let now = Local::now();
            let today = now.date_naive();
            let today_start = Local.from_local_datetime(&today.and_hms_opt(0, 0, 0).unwrap()).unwrap().naive_utc();
            let tomorrow_start = Local.from_local_datetime(&(today + Duration::days(1)).and_hms_opt(0, 0, 0).unwrap()).unwrap().naive_utc();

            let task_messages = tasks::table
                .select(message)
                .filter(created_at.ge(today_start).and(created_at.lt(tomorrow_start)))
                .load::<String>(db)
                .expect("Error");

            if use_claude {
                format_for_claude(&task_messages);
            } else {
                println!("{:?}", task_messages);
            }
        }
        QueryMode::Yesterday { use_claude, days_back } => {
            let now = Local::now();
            let today = now.date_naive();
            let target_date = today - Duration::days(days_back as i64);
            let start_time = Local.from_local_datetime(&target_date.and_hms_opt(0, 0, 0).unwrap()).unwrap().naive_utc();
            let end_time = Local.from_local_datetime(&(target_date + Duration::days(1)).and_hms_opt(0, 0, 0).unwrap()).unwrap().naive_utc();

            let task_messages = tasks::table
                .select(message)
                .filter(created_at.ge(start_time).and(created_at.lt(end_time)))
                .load::<String>(db)
                .expect("Error");

            if use_claude {
                format_for_claude(&task_messages);
            } else {
                println!("{:?}", task_messages);
            }
        }
    }
}

fn main() {
    let mut db = establish_connection();
    let args = Args::parse();

    match determine_query_mode(&args) {
        Some(mode) => handle_query_mode(mode, &mut db),
        None => println!("Gotta put something"),
    }
}
