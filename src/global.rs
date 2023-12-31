use windows::Win32::Foundation::HINSTANCE;

// global variables
pub static mut DLL_MOUDLE: Option<HINSTANCE> = None;

// todo migrate the constants into a config file
pub const IME_NAME: &str = "Ajemi";
pub const IME_NAME_ASCII: &str = "Ajemi";
pub const IME_ID: &str = "C93D3D59-2FAC-40E0-ABC6-A3658749E2FA";
pub const LANG_ID: u16 = 0x409; // en-US
pub const LANG_PROFILE_ID: &str = "A411A7FC-A082-4B8A-8741-AA4A72613933";
pub const ICON_FILE: &str = "./res/icon.ico";

