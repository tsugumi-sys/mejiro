use std::fmt;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const RELEASE_BASE_URL: &str = "https://github.com/tsugumi-sys/mejiro/releases/latest/download";

#[derive(Debug)]
pub enum UpgradeError {
    UnsupportedPlatform { os: String, arch: String },
    Io(std::io::Error),
    CommandFailed { command: String, status: i32 },
    MissingExtractedBinary(PathBuf),
    MissingCurrentExecutableName,
    MissingHomeDirectory,
}

impl fmt::Display for UpgradeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UpgradeError::UnsupportedPlatform { os, arch } => {
                write!(f, "unsupported platform: {os}-{arch}")
            }
            UpgradeError::Io(err) => write!(f, "{err}"),
            UpgradeError::CommandFailed { command, status } => {
                write!(f, "`{command}` failed with exit status {status}")
            }
            UpgradeError::MissingExtractedBinary(path) => {
                let path = path.display();
                write!(f, "expected extracted binary at {path}")
            }
            UpgradeError::MissingCurrentExecutableName => {
                write!(f, "could not determine the current executable name")
            }
            UpgradeError::MissingHomeDirectory => {
                write!(f, "could not determine the home directory")
            }
        }
    }
}

impl std::error::Error for UpgradeError {}

impl From<std::io::Error> for UpgradeError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn upgrade() -> Result<(), UpgradeError> {
    let archive_name = archive_name_for_current_platform()?;
    let current_exe = std::env::current_exe()?;
    let executable_name = current_exe
        .file_name()
        .ok_or(UpgradeError::MissingCurrentExecutableName)?
        .to_os_string();
    let install_path = default_install_path(Path::new(&executable_name))?;

    let temp_dir = temp_work_dir()?;
    let archive_path = temp_dir.join(archive_name);
    let extract_dir = temp_dir.join("extract");
    fs::create_dir_all(&extract_dir)?;

    let download_url = format!("{RELEASE_BASE_URL}/{archive_name}");

    println!("Current version: {}", env!("CARGO_PKG_VERSION"));
    println!("Downloading latest release from: {download_url}");
    run_command(
        Command::new("curl")
            .arg("-fL")
            .arg("-o")
            .arg(&archive_path)
            .arg(&download_url),
        "curl",
    )?;

    println!("Extracting release archive...");
    run_command(
        Command::new("tar")
            .arg("-xzf")
            .arg(&archive_path)
            .arg("-C")
            .arg(&extract_dir),
        "tar",
    )?;

    let extracted_binary = extract_dir.join("mejiro-cli");
    if !extracted_binary.exists() {
        return Err(UpgradeError::MissingExtractedBinary(extracted_binary));
    }

    if let Some(parent) = install_path.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("Installing to {}", install_path.display());
    let staged_path = install_path.with_extension("tmp-upgrade");
    fs::copy(&extracted_binary, &staged_path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = fs::metadata(&staged_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&staged_path, perms)?;
    }

    fs::rename(&staged_path, &install_path)?;

    println!(
        "✅ Upgrade complete. Installed latest release to {}",
        install_path.display()
    );
    if current_exe != install_path {
        println!(
            "Note: the running binary is at {}. Future shells should resolve mejiro-cli from {}.",
            current_exe.display(),
            install_path.display()
        );
    }
    maybe_print_path_hint(&install_path);

    fs::remove_dir_all(&temp_dir)?;

    Ok(())
}

fn archive_name_for_current_platform() -> Result<&'static str, UpgradeError> {
    archive_name_for_platform(std::env::consts::OS, std::env::consts::ARCH)
}

fn archive_name_for_platform(os: &str, arch: &str) -> Result<&'static str, UpgradeError> {
    match (os, arch) {
        ("macos", "aarch64") => Ok("mejiro-cli-aarch64-apple-darwin.tar.gz"),
        ("macos", "x86_64") => Ok("mejiro-cli-x86_64-apple-darwin.tar.gz"),
        ("linux", "x86_64") => Ok("mejiro-cli-x86_64-unknown-linux-gnu.tar.gz"),
        _ => Err(UpgradeError::UnsupportedPlatform {
            os: os.to_string(),
            arch: arch.to_string(),
        }),
    }
}

fn temp_work_dir() -> Result<PathBuf, UpgradeError> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_millis();
    let dir =
        std::env::temp_dir().join(format!("mejiro-upgrade-{timestamp}-{}", std::process::id()));
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn default_install_path(executable_name: &Path) -> Result<PathBuf, UpgradeError> {
    let home = std::env::var_os("HOME").ok_or(UpgradeError::MissingHomeDirectory)?;
    Ok(user_bin_dir_from_home(Path::new(&home)).join(executable_name))
}

fn user_bin_dir_from_home(home: &Path) -> PathBuf {
    home.join(".local").join("bin")
}

fn maybe_print_path_hint(install_path: &Path) {
    let Some(bin_dir) = install_path.parent() else {
        return;
    };
    let path_var = std::env::var_os("PATH").unwrap_or_default();
    let in_path = std::env::split_paths(&path_var).any(|entry| entry == bin_dir);
    if !in_path {
        println!(
            "Add {} to your PATH to use the installed binary in new shells.",
            bin_dir.display()
        );
    }
}

fn run_command(command: &mut Command, label: &str) -> Result<(), UpgradeError> {
    let status = command.status()?;
    if status.success() {
        return Ok(());
    }

    Err(UpgradeError::CommandFailed {
        command: label.to_string(),
        status: status.code().unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_name_for_platform_macos_arm() {
        let archive = archive_name_for_platform("macos", "aarch64").unwrap();
        assert_eq!(archive, "mejiro-cli-aarch64-apple-darwin.tar.gz");
    }

    #[test]
    fn test_archive_name_for_platform_macos_x64() {
        let archive = archive_name_for_platform("macos", "x86_64").unwrap();
        assert_eq!(archive, "mejiro-cli-x86_64-apple-darwin.tar.gz");
    }

    #[test]
    fn test_archive_name_for_platform_linux_x64() {
        let archive = archive_name_for_platform("linux", "x86_64").unwrap();
        assert_eq!(archive, "mejiro-cli-x86_64-unknown-linux-gnu.tar.gz");
    }

    #[test]
    fn test_archive_name_for_platform_unsupported() {
        let err = archive_name_for_platform("linux", "aarch64").unwrap_err();
        assert!(matches!(err, UpgradeError::UnsupportedPlatform { .. }));
    }

    #[test]
    fn test_user_bin_dir_from_home() {
        let dir = user_bin_dir_from_home(Path::new("/tmp/mejiro-home"));
        assert_eq!(dir, PathBuf::from("/tmp/mejiro-home/.local/bin"));
    }
}
