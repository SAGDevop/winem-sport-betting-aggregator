#[macro_use]
extern crate rocket;

use cron::Schedule;
use rocket::tokio::task;
use rocket::tokio::time::{sleep, Duration};
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Winem!"
}

async fn run_task() {
    loop {
        println!("Task executed at {:?}", SystemTime::now());
        sleep(Duration::from_secs(60)).await;
    }
}

#[launch]
fn rocket() -> _ {
    let schedule = Schedule::from_str("1/1 * * * * *").unwrap();
    let shared_schedule = Arc::new(schedule);

    task::spawn(async move {
        let mut intervals = shared_schedule.upcoming(SystemTime::now());
        while let Some(next) = intervals.next() {
            let now = SystemTime::now();
            let duration = next.duration_since(now).unwrap();
            sleep(duration).await;
            println!("Task executed at {:?}", next);
        }
    });

    rocket::build().mount("/", routes![index])
}
