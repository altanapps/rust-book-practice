
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // This is a thread
    // let handle = thread::spawn(|| {
    //     for i in 1..10{
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // // Wait for the spawned thread to finish
    // handle.join().unwrap();

    // // This is the main thread
    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

//     let v = vec![1, 2, 3];
    
//     // With move, the thread takes the ownership of v
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v)
//     });

//     handle.join().unwrap();
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // Claims the ownership of tx and sends the value
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];
        
        // Send the value and sleep for a second
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    // Print that you have received it
    for received in rx {
        println!("Got: {}", received)
    }
}