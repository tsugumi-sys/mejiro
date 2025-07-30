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
    let path = dest.display();
    println!("✅ Added image: {path}");
    Ok(())
}

pub fn list(config_path: &str) {
    let cfg = MejiroConfig::load_config(config_path);
    let dir = Path::new(&cfg.images_dir);
    if !dir.exists() {
        let path = dir.display();
        eprintln!("❌ images directory not found: {path}");
        return;
    }
    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if let Ok(rel) = entry.path().strip_prefix(dir) {
            let rel_path = rel.display();
            println!("{rel_path}");
        }
    }
}
