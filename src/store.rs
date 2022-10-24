//! Storage implementation.
//!
//! Defining generic abstractions for the application storage
//! - RegistrationStore: Stores user registrations
//! - AuthenticationStore: Stores authentication session data
//!

use std::{sync::{Arc, Mutex}, collections::HashMap};

#[derive(Clone,Debug,Default)]
pub struct RegistrationSecret {
    pub y1: i64,
    pub y2: i64,
}

#[derive(Clone,Debug,Default)]
pub struct Authentication {
    pub r1: i64,
    pub r2: i64,
    pub c: i64,
}

#[derive(Clone,Debug,Default)]
pub struct RegistrationStore {
    registrations: Arc<Mutex<HashMap<String, RegistrationSecret>>>,
}

#[derive(Clone,Debug,Default)]
pub struct AuthenticationStore {
    authentications: Arc<Mutex<HashMap<String, Authentication>>>,
}

pub trait Store<T> {
    fn new() -> Self;
    fn insert(&self, key: String, value: T);
    fn get(&self, key: String) -> Option<T>;
}

impl Store<RegistrationSecret> for RegistrationStore {
    fn new() -> Self {
        Self {
            registrations: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn insert(&self, key: String, value: RegistrationSecret) {
        let mut registrations = self.registrations.lock().unwrap();
        registrations.insert(key, value);
    }

    fn get(&self, key: String) -> Option<RegistrationSecret> {
        let registrations = self.registrations.lock().unwrap();
        match registrations.get(&key) {
            Some(registration) => Some(registration.clone()),
            None => None,
        }
    }
}

impl Store<Authentication> for AuthenticationStore {
    fn new() -> Self {
        Self {
            authentications: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn insert(&self, key: String, value: Authentication) {
        let mut authentications = self.authentications.lock().unwrap();
        authentications.insert(key, value);
    }

    fn get(&self, key: String) -> Option<Authentication> {
        let authentications = self.authentications.lock().unwrap();
        match authentications.get(&key) {
            Some(authentication) => Some(authentication.clone()),
            None => None,
        }
    }
}
