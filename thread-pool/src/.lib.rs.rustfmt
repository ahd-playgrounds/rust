use std::{
    sync::{mpsc, Arc, Mutex},
    thread::JoinHandle,
};

pub struct ThreadPool {
    _handles: Vec<JoinHandle<()>>,
    sender: mpsc::Sender<Box<dyn Fn() + Send>>,
}

impl ThreadPool {
    pub fn new(count: u8) -> Self {
        let (sender, reciever) = mpsc::channel::<Box<dyn Fn() + Send>>();
        let reciever = Arc::new(Mutex::new(reciever));
        let mut handles = vec![];
        for i in 0..=count {
            let reciever = reciever.clone();
            let handle = std::thread::spawn(move || loop {
                println!("grabbing lock {i}");
                let work = reciever.lock().unwrap().recv().unwrap();
                println!("freeing lock {i}");
                work();
            });
            handles.push(handle);
        }
        Self {
            _handles: handles,
            sender,
        }
    }

    pub fn execute<F: Fn() + Send + 'static>(&self, fun: F) {
        self.sender.send(Box::new(fun)).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU8, Ordering};

    use super::*;

    #[test]
    fn it_works() {
        let pool = ThreadPool::new(4);
        let z = Arc::new(AtomicU8::new(5));
        let x = z.clone();
        let b = z.clone();

        pool.execute(move || {
            x.fetch_add(1, Ordering::SeqCst);
            println!("doing work");
            std::thread::sleep(std::time::Duration::from_secs(1));
            println!("doing last bit of work");
        });

        pool.execute(move || {
            b.fetch_add(1, Ordering::SeqCst);
            println!("doing more work")
        });
        std::thread::sleep(std::time::Duration::from_secs(2));
        println!("{}", z.load(Ordering::SeqCst));
        assert!(false);
    }
}
