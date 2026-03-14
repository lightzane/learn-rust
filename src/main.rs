use std::thread;
use std::time::Duration;

use std::sync::mpsc;

use std::sync::{Arc, Mutex};

fn main() {
    // * ---------------------------------------------------------------
    // * Creating a new thread with spawn
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    /*
        [NOTE]

        When the main thread of a Rust program completes,
        all spawned threads are shut down, whether or not they have finished running
    */

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread with handle");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for the thread to finish before the main thread exits
    handle.join().unwrap();

    // main thread also waits above line before proceeding to the next lines
    for i in 1..5 {
        println!("hi number {i} from the main thread after handle!");
        thread::sleep(Duration::from_millis(1));
    }

    // * ---------------------------------------------------------------
    // * Using `move` closures
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // ⚠️ This closure will throw an error if the `move` is not present
        // By adding `move` we force the closure to take ownership of the values
        println!("here's a vector: {v:?}");
    });

    handle.join().unwrap();

    // * ---------------------------------------------------------------
    // * Transfer data between threads with message passing

    // > Slogan from the [Go language documentation](https://golang.org/doc/effective_go.html#concurrency)
    // 💡 “Do not communicate by sharing memory; instead, share memory by communicating.”

    // mpsc = multi-producer single-consumer
    // tx = transmitter (sender)
    // rx = receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    // * ---------------------------------------------------------------
    // * Transferring ownership through the channels
    let (tx, rx) = mpsc::channel();
    // tx: Sender<String>
    // rx: Sender<String>
    // String is inferred via tx.send()

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {val}"); // ! ❌ [error] borrow of moved value `val`
        // The `send()` takes ownership of its parameter !
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    // * ---------------------------------------------------------------
    // * Sending multiple values
    let (tx, rx) = mpsc::channel::<String>(); // explicitly give type annotations

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Treating the `rx` as an iterator
    for received in rx {
        println!("Got: {received}"); // will print all `val`!
        // Hence, we can tell that the main thread
        // is waiting to receive values from the spawned thread
    }

    // * ---------------------------------------------------------------
    // * Creating multiple producers
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    // * ---------------------------------------------------------------
    // * Shared-state concurrency (part 1)

    // Use `Mutext<T>` - mutual exclusion
    // allows one thread to access some data at any given time.

    // First example will show in single-thread
    let m = Mutex::new(5);

    // internal scope (just in case you didn't notice)
    {
        let mut num = m.lock().unwrap();
        *num = 6;

        // .lock() returns a `MutexGuard` that implements `Deref` and `Drop`
        // So when it goes out of scope, it will automatically unlock
        // or release the lock
    }

    // The data is unlocked
    println!("m = {m:?}");

    // * ---------------------------------------------------------------
    // * Shared-state concurrency (part 2)

    // Since threads should take ownership
    // for multi-threads we must enable multi-ownership
    // You may think about `Rc<T>` but remember that it's only for single-thread!
    // For multi-thread we use the `Arc<T>` instead!

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
