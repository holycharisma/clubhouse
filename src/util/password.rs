extern crate bcrypt;

use std::str;

use bcrypt::{hash, DEFAULT_COST};

pub struct PasswordUtil {}

impl PasswordUtil {
    pub fn into_password_hash(_plaintext: &str) -> Vec<u8> {
        match hash(_plaintext, DEFAULT_COST) {
            Ok(hash) => String::into_bytes(hash),
            _ => vec![]
        }
    }

    pub fn verify_hashed_bytes(attempt: &str, hash: &[u8]) -> bool {
        match bcrypt::verify(attempt, str::from_utf8(hash).unwrap()) {
            Ok(b) => b,
            _ => false
        }
    }
}


#[cfg(test)]
mod test {

    use crate::util::emoji;

    use super::*;

    #[test]
    fn test_some_password_stuff() {

        let password_plaintext = "hunter23";
        let password_hash = PasswordUtil::into_password_hash(&password_plaintext);

        let encoded_password = emoji::encode(&password_hash);
        println!("encoded password: {}", encoded_password);

        let decoded_hash = emoji::decode(&encoded_password);

        assert!(PasswordUtil::verify_hashed_bytes(&password_plaintext, &decoded_hash));
        assert!(! PasswordUtil::verify_hashed_bytes("hunter24", &decoded_hash))
    }
}


