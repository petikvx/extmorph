use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RansomProfile {
    pub name: String,
    pub expected_sha256: String,
    pub offsets: Vec<usize>,
    pub compress: bool,
}

impl RansomProfile {
    pub fn new(name: &str, sha256: &str, offsets: Vec<usize>, compress: bool) -> Self {
        Self {
            name: name.to_string(),
            expected_sha256: sha256.to_string(),
            offsets,
            compress,
        }
    }
}

pub mod abyss;
pub mod babuk;
pub mod blackhunt2;
pub mod chaos52;
pub mod trigona;

pub fn get_profile(name: &str) -> Option<RansomProfile> {
    match name.to_lowercase().as_str() {
        "abyss" => Some(abyss::profile()),
        "babuk" => Some(babuk::profile()),
        "blackhunt2" => Some(blackhunt2::profile()),
        "chaos52" => Some(chaos52::profile()),
        "trigona" => Some(trigona::profile()),
        _ => None,
    }
}

pub fn list_profiles() -> Vec<String> {
    vec![
        "abyss".to_string(),
        "babuk".to_string(),
        "blackhunt2".to_string(),
        "chaos52".to_string(),
        "trigona".to_string(),
    ]
}
