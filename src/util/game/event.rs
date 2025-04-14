use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct Event<T> {
    subscribers: Mutex<Vec<Box<dyn Fn(&T) + Send + Sync>>>,
}

impl<T> Event<T> {
    pub fn new() -> Arc<Self> {
        Arc::new(Event {
            subscribers: Mutex::new(Vec::new()),
        })
    }

    pub fn subscribe<F>(self: &Arc<Self>, callback: F)
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut subs = self.subscribers.lock().unwrap();
        subs.push(Box::new(callback));
    }

    pub fn trigger(&self, data: &T) {
        let subs = self.subscribers.lock().unwrap();
        for subscriber in subs.iter() {
            subscriber(data);
        }
    }
}

pub struct EventManager {
    events: HashMap<String, Event<!>>,
}
