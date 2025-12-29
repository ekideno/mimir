use clap::Args;
use std::path::Path;
use std::process::Command;

#[derive(Args)]
pub struct OpenArgs {
    /// Name of the file or subject to open
    pub name: String,
}

pub fn handle(args: &OpenArgs) {
    let path_file = Path::new(&args.name);
    let path_subject = Path::new("./subjects").join(&args.name);

    let path_to_open = if path_file.exists() {
        path_file
    } else if path_subject.exists() {
        path_subject.as_path()
    } else {
        eprintln!("File or subject '{}' not found", args.name);
        return;
    };

    #[cfg(target_os = "windows")]
    Command::new("cmd")
        .args(["/C", "start", "", path_to_open.to_str().unwrap()])
        .status()
        .unwrap();

    #[cfg(target_os = "macos")]
    Command::new("open").arg(path_to_open).status().unwrap();

    #[cfg(target_os = "linux")]
    Command::new("xdg-open").arg(path_to_open).status().unwrap();

    println!("Opened '{}'", path_to_open.display());
}
