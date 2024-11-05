// Imports for AES-256-CBC encryption/decryption
use aes::Aes256;
use cbc::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use hex_literal::hex;
use serde::{Serialize, Deserialize};
use regex::Regex;
use std::str;

// Define type aliases for AES-256 CBC cipher
type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

// Encryption key and initialization vector constants
const ENCRYPTION_KEY: [u8; 32] = hex!("00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff");
const IV: [u8; 16] = hex!("aabbccddeeff00112233445566778899");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub alias: String,
    pub skills: Vec<String>,
    pub bio: String,
    pub wallet_address: String, // Encrypted wallet address
    pub location: Option<String>,
}

impl Profile {
    // Constructor with validation
    pub fn new(alias: String, skills: Vec<String>, bio: String, wallet_address: String, location: Option<String>) -> Result<Self, String> {
        let profile = Profile {
            alias,
            skills,
            bio,
            wallet_address,
            location,
        };
        profile.validate()?; // Ensure validation on creation
        Ok(profile)
    }

    // Encrypt wallet address
    pub fn encrypt_wallet_address(&mut self) -> Result<(), String> {
        let cipher = Aes256CbcEnc::new_from_slices(&ENCRYPTION_KEY, &IV).map_err(|_| "Invalid key/IV")?;
        let mut buffer = self.wallet_address.as_bytes().to_vec();
        // Ensure the buffer has enough space for padding
        buffer.resize(buffer.len() + 16 - (buffer.len() % 16), 0);
        let encrypted_data = cipher.encrypt_padded_mut::<Pkcs7>(&mut buffer, self.wallet_address.len())
            .map_err(|_| "Encryption failed")?;
        self.wallet_address = hex::encode(encrypted_data);
        Ok(())
    }

    // Decrypt wallet address
    pub fn decrypt_wallet_address(&self) -> Result<String, String> {
        let cipher = Aes256CbcDec::new_from_slices(&ENCRYPTION_KEY, &IV).map_err(|_| "Invalid key/IV")?;
        let mut encrypted_bytes = hex::decode(&self.wallet_address).map_err(|_| "Invalid hex in wallet address")?;
        let decrypted_data = cipher.decrypt_padded_mut::<Pkcs7>(&mut encrypted_bytes)
            .map_err(|_| "Decryption failed")?;
        Ok(String::from_utf8(decrypted_data.to_vec()).map_err(|_| "Invalid UTF-8 in decrypted data")?)
    }

    // Validation methods
    fn validate(&self) -> Result<(), String> {
        self.validate_alias()?;
        self.validate_skills()?;
        self.validate_bio()?;
        self.validate_wallet_address()?;
        self.validate_location()?;
        Ok(())
    }

    fn validate_alias(&self) -> Result<(), String> {
        if self.alias.trim().is_empty() || self.alias.len() > 20 {
            Err("Alias must be between 1 and 20 characters.".into())
        } else {
            Ok(())
        }
    }

    fn validate_skills(&self) -> Result<(), String> {
        if self.skills.is_empty() {
            Err("Skills list cannot be empty.".into())
        } else {
            Ok(())
        }
    }

    fn validate_bio(&self) -> Result<(), String> {
        if self.bio.len() > 200 {
            Err("Bio cannot exceed 200 characters.".into())
        } else {
            Ok(())
        }
    }

    fn validate_wallet_address(&self) -> Result<(), String> {
        let eth_wallet_regex = Regex::new(r"^0x[a-fA-F0-9]{40}$").unwrap();
        if !eth_wallet_regex.is_match(&self.wallet_address) {
            Err("Invalid Ethereum wallet address format.".into())
        } else {
            Ok(())
        }
    }

    fn validate_location(&self) -> Result<(), String> {
        if let Some(location) = &self.location {
            if location.len() > 50 {
                return Err("Location cannot exceed 50 characters.".into());
            }
        }
        Ok(())
    }

    // Display method with decrypted wallet address
    pub fn display(&self) -> String {
        let wallet_display = self.decrypt_wallet_address().unwrap_or_else(|_| "Error decrypting".to_string());
        format!(
            "Alias: {}\nSkills: {:?}\nBio: {}\nWallet Address: {}\nLocation: {:?}",
            self.alias,
            self.skills,
            self.bio,
            wallet_display,
            self.location
        )
    }
}
