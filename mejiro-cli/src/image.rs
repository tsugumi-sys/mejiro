use config::MejiroConfig;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

pub fn add(config_path: &str, image_path: &str) -> io::Result<()> {
    let cfg = MejiroConfig::load_config(config_path);
    let src = Path::new(image_path);
    let dest_dir = Path::new(&cfg.images_dir);
    fs::create_dir_all(dest_dir)?;
    let dest = dest_dir.join(src.file_name().unwrap());
    fs::copy(src, &dest)?;
    println!("✅ Added image: {}", dest.display());
    Ok(())
}

pub fn list(config_path: &str) {
    let cfg = MejiroConfig::load_config(config_path);
    let dir = Path::new(&cfg.images_dir);
    if !dir.exists() {
        eprintln!("❌ images directory not found: {}", dir.display());
        return;
    }
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if let Ok(rel) = entry.path().strip_prefix(dir) {
            println!("{}", rel.display());
        }
    }
}
