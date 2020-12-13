use std::sync::Mutex;

pub fn run() {
    let m = Mutex::new(5);

    {
        // "Mutex<T>" is a smart pointer. More accurately, the call to lock returns a
        // smart pointer called "MutexGuard", wrapped in a "LockResult".
        //
        // The "MutexGuard" smart pointer:
        // + Implements "Deref" to point at our inner data.
        // + Implements "Drop" to automatically release the lock when it goes out-of-scope.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
