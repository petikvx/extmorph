use super::RansomProfile;

pub fn profile() -> RansomProfile {
    RansomProfile::new(
        "abyss",
        "9243bdcbe30fbd430a841a623e9e1bcc894e4fdc136d46e702a94dad4b10dfdc",
        vec![0x978A, 0x978C, 0x978E, 0x9790, 0x9792],
        true, // Compress with UPX
    )
}
