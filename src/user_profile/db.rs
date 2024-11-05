// src/user-profile/db.rs

use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use crate::user_profile::profile::Profile;

use serde::Serialize;

#[derive(Serialize)]
pub struct ProfileData {
    pub profiles: Vec<Profile>,
}


impl ProfileData {
    // Load profiles from file with JSON deserialization
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let mut file = File::open(path).unwrap_or_else(|_| File::create(path).unwrap());
        let mut content = String::new();
        file.read_to_string(&mut content).map_err(|_| "Failed to read file")?;

        let profiles: Vec<Profile> = serde_json::from_str(&content).unwrap_or(vec![]);
        Ok(ProfileData { profiles })
    }

    // Save profiles to file with JSON serialization
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let json_data = serde_json::to_string_pretty(&self).map_err(|_| "Failed to serialize data")?;
        let mut file = OpenOptions::new().write(true).truncate(true).open(path)
            .map_err(|_| "Unable to open file for writing")?;
        file.write_all(json_data.as_bytes()).map_err(|_| "Failed to write data to file")?;
        Ok(())
    }

    // Add a new profile and encrypt wallet address before saving
    pub fn add_profile(&mut self, mut profile: Profile) -> Result<(), String> {
        profile.encrypt_wallet_address()?;
        self.profiles.push(profile);
        Ok(())
    }
}

