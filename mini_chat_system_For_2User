use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;

fn main() {
    println!("Mini Chat System (2 users)");
    // Create a channel for communication
    let (tx, rx) = mpsc::channel();
    // Thread for User1
    let tx1 = tx.clone();
    let user1 = thread::spawn(move || {
        loop {
            print!("User1: ");
            io::stdout().flush().unwrap();

            let mut msg = String::new();
            io::stdin().read_line(&mut msg).unwrap();
            let msg = msg.trim().to_string();

            if msg.to_lowercase() == "quit" {
                println!("User1 left the chat.");
                break;
            }
            tx1.send(("User1".to_string(), msg)).unwrap();
        }
    });
    // Thread for User2
    let tx2 = tx.clone();
    let user2 = thread::spawn(move || {
        loop {
            println!("User2: ");
            io::stdout().flush().unwrap();

            let mut msg = String::new();
            io::stdin().read_line(&mut msg).unwrap();
            let msg = msg.trim().to_string();

            if msg.to_lowercase() == "quit" {
                println!("User2 left the chat.");
                break;
            }
            tx2.send(("User2".to_string(), msg)).unwrap();
        }
    });
    // Main thread receives messages
    for (user, message) in rx {
        println!("{} says: {}", user, message);
    }
    // Wait for threads to finish
    user1.join().unwrap();
    user2.join().unwrap();
}
