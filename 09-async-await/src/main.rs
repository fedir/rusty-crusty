use std::time::Duration;

/// The entry point of an async Rust program using the Tokio runtime.
/// The #[tokio::main] macro sets up the executor that runs async tasks.
#[tokio::main]
async fn main() {
    println!("Starting async tasks...");

    // tokio::spawn schedules a task to run concurrently on the executor.
    let task1 = tokio::spawn(async {
        do_work("Task 1", 2).await;
    });

    let task2 = tokio::spawn(async {
        do_work("Task 2", 1).await;
    });

    // tokio::join! waits for multiple futures to complete at once.
    let _ = tokio::join!(task1, task2);

    println!("All tasks completed.");
}

/// A simulated asynchronous workload.
/// The 'async' keyword makes this function return a Future.
async fn do_work(name: &str, seconds: u64) {
    println!("{} started.", name);
    // tokio::time::sleep is the async version of thread::sleep.
    // It avoids blocking the entire operating system thread.
    tokio::time::sleep(Duration::from_secs(seconds)).await;
    println!("{} finished.", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_work() {
        // Just verify it runs without panicking within a reasonable time
        do_work("Test", 0).await;
    }
}
