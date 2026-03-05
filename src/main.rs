use clap::Parser;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Parser)]
#[command(about = "Acronym store")]
struct Cli {
    /// Acronym key (or "commit")
    key: String,
    /// Value to set (optional)
    value: Option<String>,
}

fn acro_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".acro");
    path
}

fn read_acros() -> BTreeMap<String, String> {
    let path = acro_path();
    let mut map = BTreeMap::new();
    if let Ok(contents) = fs::read_to_string(&path) {
        for line in contents.lines() {
            if let Some((k, v)) = line.split_once('=') {
                map.insert(k.to_string(), v.to_string());
            }
        }
    }
    map
}

fn write_acros(map: &BTreeMap<String, String>) {
    let path = acro_path();
    let contents: String = map
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&path, contents + "\n").expect("Failed to write .acro file");
}

fn commit() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let status = Command::new("git")
        .args(["add", ".acro"])
        .current_dir(&dir)
        .status()
        .expect("Failed to run git");
    if !status.success() {
        eprintln!("git add failed");
        std::process::exit(1);
    }
    let status = Command::new("git")
        .args(["commit", "-m", "Update acronyms"])
        .current_dir(&dir)
        .status()
        .expect("Failed to run git");
    if !status.success() {
        eprintln!("git commit failed");
        std::process::exit(1);
    }
    let status = Command::new("git")
        .args(["push"])
        .current_dir(&dir)
        .status()
        .expect("Failed to run git");
    if !status.success() {
        eprintln!("git push failed");
        std::process::exit(1);
    }
}

fn main() {
    let cli = Cli::parse();

    if cli.key == "commit" {
        commit();
        return;
    }

    let key = cli.key.to_uppercase();

    match cli.value {
        Some(value) => {
            let mut map = read_acros();
            map.insert(key.clone(), value);
            write_acros(&map);
            println!("https://s3bw.github.io/acro/{key}.html");
        }
        None => {
            let map = read_acros();
            match map.get(&key) {
                Some(v) => println!("{v}"),
                None => {
                    eprintln!("Unknown acronym: {key}");
                    std::process::exit(1);
                }
            }
        }
    }
}
