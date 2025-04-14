use std::{ffi::CStr, fs::File, io::Read};
/// The `get_assets_dir` function constructs the path to the assets directory
pub fn get_assets_dir() -> std::path::PathBuf {
    let assets_dir = std::path::Path::new(&super::constants::HOME.as_os_str())
        .join(".local/share/glfwproject/assets");
    assets_dir
}

/// The `get_asset` function retrieves the asset file from the assets directory.
pub fn get_asset(path: &str) -> Result<std::fs::File, String> {
    let assets_dir = get_assets_dir();
    let asset_path = assets_dir.join(path);
    if asset_path.is_file() {
        Ok(File::open(&asset_path).unwrap())
    } else {
        Err(format!("Asset not found: {}", asset_path.display()))
    }
}

/// The `read_asset_to_vec` function reads the asset file into a null-terminated CString.
pub fn read_asset_to_cstr(path: &str) -> std::ffi::CString {
    let mut f = get_asset(path).unwrap();
    let mut buf = Vec::new();
    let _ = f.read_to_end(&mut buf);
    buf.push(0); // null terminator
    let cstr_res = unsafe { CStr::from_bytes_with_nul_unchecked(&buf).to_owned() };
    cstr_res
}
