///
/// Public module for the ThreadPool
/// implementation.
/// 
pub mod pool {
    use std::sync::Arc;
    use std::sync::mpsc::{self, Sender, Receiver};
    use std::sync::Mutex;
    use std::thread::{JoinHandle, self};


    ///
    /// Struct used for ThreadPool
    /// execution.
    /// 
    /// threads: list of thread join handles
    /// 
    pub struct ThreadPool {
        threads: Vec<Worker>,
        sender: Sender<Job>,
    }

    ///
    /// Implementation block for ThreadPool.
    /// 
    impl ThreadPool {

        ///
        /// Returns a new instance of a ThreadPool with
        /// a group of spawned threads.
        /// This function will panic if given a size
        /// of 0 threads.
        /// 
        /// #[test]
        /// #[should_panic]
        /// fn zero_size_test() {
        ///     let t = ThreadPool::new(0);
        /// }
        /// 
        pub fn new(size: usize) -> ThreadPool {
            assert!(size > 0);

            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));
            let mut threads = Vec::with_capacity(size);
            for id in 0..size {
                threads.push(Worker::new(id, Arc::clone(&receiver)));
            }

            ThreadPool { threads, sender }
        }

        ///
        /// Passes the closure to the pool
        /// of waiting threads to be eventually
        /// executed.
        /// 
        pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
            {
            let job = Box::new(f);

            self.sender.send(job).unwrap();
        }
    }

    ///
    /// Worker holds a unique id
    /// and a join handle for an
    /// associated thread.
    /// 
    struct Worker {
        id: usize,
        thread: JoinHandle<()>,
    }

    ///
    /// Implementation block for a
    /// worker.
    /// 
    impl Worker {

        ///
        /// Spawns a thread to wait on the receiving
        /// end of the channel and perform a job
        /// if one is available.
        /// 
        fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
            let thread = thread::spawn(move || loop {
                let job = receiver.lock().expect("Poisoned thread").recv().expect("Panicked thread");

                println!("Worker {} got a job.", id);

                job();
            });

            Worker { id, thread }
        }
    }

    ///
    /// The Job type is a pointer to
    /// any closure type that is able to
    /// be used by threads.
    /// 
    type Job = Box<dyn FnOnce() + Send + 'static>;
}