extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash};

pub fn hash_password(password: &str) -> Option<String> {
    if let Ok(hashed) = hash(password, DEFAULT_COST) { 
        return Some(hashed);
    } else { 
        return None;
    }
}