//! Storage implementation.
//!
//! Defining generic abstractions for the application storage
//! - RegistrationStore: Stores user registrations
//! - AuthenticationStore: Stores authentication session data
//!

use std::{sync::{Arc, Mutex}, collections::HashMap};

#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct RegistrationSecret {
    pub y1: i64,
    pub y2: i64,
}

#[derive(Clone,Debug,Default,PartialEq,Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registration_store_insert() {
        // Setup
        let store = RegistrationStore::new();
        let secret = RegistrationSecret { y1: 1, y2: 2 };
        // Test
        store.insert("user".to_string(), secret.clone());

        // Assert
        assert_eq!(secret.clone(), store.get("user".to_string()).unwrap());
        
    }

    #[test]
    fn test_authentication_store_insert() {
        // Setup
        let store = AuthenticationStore::new();
        let authentication = Authentication { r1: 1, r2: 2, c:3 };
        // Test
        store.insert("auth_id".to_string(), authentication.clone());

        // Assert
        assert_eq!(authentication.clone(), store.get("auth_id".to_string()).unwrap());
    }

    
}

