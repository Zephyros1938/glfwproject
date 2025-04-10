fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed-src/assets=true");

    use std::env;
    use std::fs;
    use std::path::Path;

    let assets_dir = Path::new(&env::home_dir().unwrap()).join(".local/share/glfwproject/assets");

    fs::create_dir_all(&assets_dir).expect("Failed to create assets directory");
    let src_assets_dir = Path::new("./src/assets"); // Adjust the path as needed
    if src_assets_dir.exists() {
        println!("Copying assets to {}", assets_dir.display());
        for entry in src_assets_dir.read_dir()?.into_iter() {
            let entry = entry?;
            println!("Copying {:?} ...", entry.path());
            let asset_path = entry.path();
            let target_path = assets_dir.join(asset_path.file_name().unwrap());
            if (asset_path.is_dir()) {
                if (!asset_path.exists()) {
                    fs::create_dir(&asset_path).unwrap();
                }
            } else {
                fs::copy(asset_path, target_path)?;
            }
        }
    } else {
        println!("Assets directory not found: {}", src_assets_dir.display());
    }

    Ok(())
}
