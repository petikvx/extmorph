use super::RansomProfile;

pub fn profile() -> RansomProfile {
    RansomProfile::new(
        "blackhunt2",
        "74df3452a6b9dcdba658af7a9cf5afb09cce51534f9bc63079827bf73075243b",
        vec![
            0x581F0, 0x581F2, 0x581F4, 0x581F6, 0x581F8, 0x581FA, 0x581FC, 0x581FE, 0x58200,
            0x5083E, 0x50840, 0x50842, 0x50844, 0x50846,
            0x510B8, 0x510BA, 0x510BC, 0x510BE, 0x510C0,
        ],
        false,
    )
}
