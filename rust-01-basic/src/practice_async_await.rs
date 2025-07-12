// Problem 1: Fix the code to make it compile. You may only add code, not remove it.
// Solution:

use std::time::Duration;
use tokio::time::sleep;

struct Project {
    id: u32,
    name: String,
    duration_days: u32,
}

impl Project {
    fn new(id: u32, name: &str, duration_days: u32) -> Self {
        Self {
            id,
            name: name.to_string(),
            duration_days,
        }
    }
}

#[tokio::main]
pub(crate) async fn solution_p1() {
    let (id1, id2) = (1, 2);
    let project1 = read_details_from_db(id1).await.unwrap();
    let project2 = read_details_from_db(id2).await.unwrap();
    if project1.duration_days > project2.duration_days {
        println!(
            "{} takes {} days more than {}",
            project1.name,
            project1.duration_days - project2.duration_days,
            project2.name
        );
    } else if project2.duration_days > project1.duration_days {
        println!(
            "{} takes {} days more than {}",
            project2.name,
            project2.duration_days - project1.duration_days,
            project1.name
        );
    } else {
        println!("Both {} and {} take the same number of days", project1.name, project2.name);
    }
}

async fn read_details_from_db(id: u32) -> Result<Project, String> {
    // dummy read from database
    sleep(Duration::from_millis(1000)).await;
    let database = [
        Project::new(1, "Project Alpha", 30),
        Project::new(2, "Project Beta", 45),
        Project::new(3, "Project Gamma", 30),
    ];
    for project in database {
        if id == project.id {
            return Ok(project);
        }
    }
    Err("Project record not present".into())
}



// Problem 2: Fix the code so that the two functions execute concurrently.
// Consider calling the functions inside spawned tasks.
// Solution:


async fn fn1() {
    println!("Task 1 started!");
    sleep(Duration::from_secs(3)).await;
    println!("Task 1 completed!");
}

async fn fn2() {
    println!("Task 2 started!");
    sleep(Duration::from_secs(2)).await;
    println!("Task 2 completed!");
}

#[tokio::main]
pub(crate) async fn solution_p2() {
    let mut handles = vec![];
    let handle_1 = tokio::spawn(async move {
        fn1().await;
    });
    handles.push(handle_1);

    let handle_2 = tokio::spawn(async move {
        fn2().await;
    });
    handles.push(handle_2);

    for handle in handles {
        handle.await.unwrap();
    }
}
