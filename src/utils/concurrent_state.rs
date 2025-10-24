use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct ConcurrentState<T> {
    pub state: Arc<Mutex<T>>
}

impl<T> ConcurrentState<T> {
    pub fn new(state: T) -> Self {
        ConcurrentState{state: Arc::new(Mutex::new(state))}
    }
}