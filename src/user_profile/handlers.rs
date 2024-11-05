// src/user-profile/handlers.rs

use crate::user_profile::db::ProfileData;
use crate::user_profile::profile::Profile;

pub fn create_and_save_profile(
    alias: String, 
    skills: Vec<String>, 
    bio: String, 
    wallet_address: String,
    file_path: &str
) {
    let mut data = ProfileData::load_from_file(file_path);
    let new_profile = Profile::new(alias, skills, bio, wallet_address);
    data.add_profile(new_profile);
    data.save_to_file(file_path);
}

pub fn list_profiles(file_path: &str) {
    let data = ProfileData::load_from_file(file_path);
    for profile in data.profiles {
        println!("{}", profile.display());
    }
}
