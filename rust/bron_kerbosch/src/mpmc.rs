//! Rudimentary multiple-producer-multiple-consumer channel

use std::sync::{mpsc, Arc, Mutex};

#[derive(Debug)]
pub struct Receiver<T>(Arc<Mutex<mpsc::Receiver<T>>>);

pub fn channel<T>() -> (mpsc::Sender<T>, Receiver<T>) {
    let (tx, rx) = mpsc::channel();
    (tx, Receiver(Arc::new(Mutex::new(rx))))
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Receiver(self.0.clone())
    }
}
impl<T> Receiver<T> {
    pub fn recv(&self) -> Result<T, mpsc::RecvError> {
        let guard = self.0.lock().unwrap();
        guard.recv()
    }
}
