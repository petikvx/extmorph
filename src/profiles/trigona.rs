use super::RansomProfile;

pub fn profile() -> RansomProfile {
    RansomProfile::new(
        "trigona",
        "85f4088286ac1eedc94ad9dc6465e9e4b89d1cde3012f9949450fcc9f2b60431",
        vec![0x2F26C, 0x2F26E, 0x2F270, 0x2F272, 0x2F274, 0x2F276],
        true, // Compress with UPX
    )
}
