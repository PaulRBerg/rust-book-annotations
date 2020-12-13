use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // The main thread will wait for the spawned thread to finish and then run
    // its "for" loop, so the output won’t be interleaved anymore.
    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Without this, the spawned thread is stopped prematurely most of the time
    // due to the main thread ending. In fact, the spawned thread wouldn't be
    // guaranteed to get run at all (because of how the OS schedules threads).
    //
    // The two threads continue alternating, but the main thread waits because
    // of the call to "handle.join()" and does not end until the spawned thread
    // is finished.
    // handle.join().unwrap();
}
