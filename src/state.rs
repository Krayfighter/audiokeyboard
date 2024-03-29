
use std::sync::{
    Arc,
    Mutex,
};

use std::collections::HashMap;

#[derive(Clone)]
pub struct KeyboardState (
    Arc<Mutex<HashMap<char, f32>>>
);

impl KeyboardState {
    pub fn new() -> Self {
        return Self ( Arc::new(Mutex::new(HashMap::default())) );
    }

    pub fn cloned(&self) -> HashMap<char, f32> {
        let inner = self.0.lock().unwrap();
        return inner.clone();
    }

    pub fn get(&self, idx: char) -> Option<f32> {
        let inner = self.0.lock().unwrap();
        return inner.get(&idx).copied();
    }

    pub fn set(&mut self, key: char, value: f32) {
        let mut inner = self.0.lock().unwrap();
        inner.insert(key, value);
    }

    pub fn remove(&mut self, key: char) {
        let mut inner = self.0.lock().unwrap();
        let _ = inner.remove(&key);
    }
}

#[derive(Clone)]
pub struct SyncFunctionPtr( Arc<Mutex<Option<fn(f32, f32, f32) -> f32>>> );

impl Default for SyncFunctionPtr {
    fn default() -> Self {
        return Self ( Arc::new(Mutex::new( None )) );
    }
}

impl SyncFunctionPtr {
    pub fn get(&self) -> Option<fn(f32, f32, f32) -> f32> {
        let inner = self.0.lock().unwrap();
        return inner.clone();
    }

    pub fn set(&self, function: fn(f32, f32, f32) -> f32) {
        let mut inner = self.0.lock().unwrap();
        *inner = Some(function);
    }
}



