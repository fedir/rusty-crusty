use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("Starting async tasks...");

    let task1 = tokio::spawn(async {
        do_work("Task 1", 2).await;
    });

    let task2 = tokio::spawn(async {
        do_work("Task 2", 1).await;
    });

    let _ = tokio::join!(task1, task2);
    
    println!("All tasks completed.");
}

async fn do_work(name: &str, seconds: u64) {
    println!("{} started.", name);
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    println!("{} finished.", name);
}
