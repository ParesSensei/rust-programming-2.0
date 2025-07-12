// -------------------------------------------
// 			Async Await
// -------------------------------------------

async fn printings() {
    println!("I am async function");
}

#[tokio::test]
async fn main() {
    let x = printings();

    println!("The future has not been polled yet");
    // drop(x);
    x.await;
}


// -------------------------------------------
// 			Tokio Tasks
// -------------------------------------------
async fn printing(i: i32) {
    sleep(Duration::from_secs(1)).await;
    println!("Task {i}");
}
use std::time::Duration;
use tokio::time::sleep;


#[tokio::test]
//#[tokio::main(flavor = "current_thread")]
pub async fn main_2() {
    let mut handles = vec![];
    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("Task {i}, printing, first time");
            printing(i).await;
            println!("Task {i}, printing, second time");
            printing(i).await;
            println!("Task {i}, completed");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("All Tasks are now completed");
}


