use super::RansomProfile;

pub fn profile() -> RansomProfile {
    RansomProfile::new(
        "babuk",
        "9479a5dc61284ccc3f063ebb38da9f63400d8b25d8bca8d04b1832f02fac24de",
        vec![0xF0E2, 0xF0E4, 0xF0E6, 0xF0E8, 0xF0EA],
        false,
    )
}
