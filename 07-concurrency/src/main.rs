use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // --- 1. Simple Thread Spawning ---
    // thread::spawn takes a closure and runs it in a separate operating system thread.
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The main thread continues to run concurrently with the spawned thread.
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // handle.join() blocks the current thread until the spawned thread finishes.
    handle.join().unwrap();

    // --- 2. Message Passing using Channels ---
    // channel() returns a transmitter (tx) and a receiver (rx).
    let (tx, rx) = mpsc::channel();

    // Spawn another thread to send messages back to the main thread.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // .send() transfers ownership of the value to the receiver.
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    println!("\nReceived:");
    // The receiver (rx) can be treated as an iterator that waits for messages.
    for received in rx {
        println!("Got: {}", received);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_spawn() {
        let handle = thread::spawn(|| 42);
        assert_eq!(handle.join().unwrap(), 42);
    }
}
