use std::sync::{Arc, RwLock};

// The Clock structure that holds the current time
pub struct Clock(Arc<RwLock<u128>>);

impl Clock {
    // Returns a new Clock object
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(0)))
    }

    // Increments the current time by 1
    pub fn tick(&mut self) {
        let mut w = self.0.write().unwrap();
        *w += 1;
    }

    // Returns the current time
    pub fn time(&self) -> u128 {
        *self.0.read().unwrap()
    }
}

// Implement the Clone trait for Clock to be able to clone the Arc pointer in a thread-safe way
impl Clone for Clock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
