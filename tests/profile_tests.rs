use my_project::user_profile::profile::Profile;
use my_project::user_profile::db::ProfileData;

#[test]
fn test_profile_creation() {
    let alias = "Solarian".to_string();
    let skills = vec!["Rust".to_string(), "Blockchain".to_string()];
    let bio = "Building decentralized systems.".to_string();
    let wallet_address = "0x123456789abcdef".to_string();
    
    let profile = Profile::new(alias.clone(), skills.clone(), bio.clone(), wallet_address.clone());

    assert_eq!(profile.alias, alias);
    assert_eq!(profile.skills, skills);
    assert_eq!(profile.bio, bio);
    assert_eq!(profile.wallet_address, wallet_address);
}

#[test]
fn test_profile_encryption() {
    let wallet_address = "0x123456789abcdef".to_string();
    let mut profile = Profile::new(
        "TestUser".to_string(),
        vec!["Rust".to_string()],
        "Testing encryption.".to_string(),
        wallet_address.clone(),
    );
    
    profile.encrypt_wallet_address().expect("Encryption failed");
    assert_ne!(profile.wallet_address, wallet_address);
    
    profile.decrypt_wallet_address().expect("Decryption failed");
    assert_eq!(profile.wallet_address, wallet_address);
}

#[test]
fn test_file_storage() {
    let alias = "Solarian".to_string();
    let profile = Profile::new(
        alias.clone(),
        vec!["Rust".to_string(), "Blockchain".to_string()],
        "Building decentralized systems.".to_string(),
        "0x123456789abcdef".to_string(),
    );
    
    let mut profile_data = ProfileData { profiles: vec![] };
    profile_data.add_profile(profile);

    let file_path = "test_profile_data.json";
    profile_data.save_to_file(file_path);

    let loaded_data = ProfileData::load_from_file(file_path);
    assert_eq!(loaded_data.profiles[0].alias, alias);
}
