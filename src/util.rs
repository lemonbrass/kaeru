use crate::error::Error;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process::{exit, Command};

pub fn run_command(command_str: &str) -> io::Result<()> {
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    if !Path::new(&shell).exists() {
        eprintln!("Shell '{}' not found.", shell);
        exit(1);
    }

    let mut child = Command::new(shell).arg("-c").arg(command_str).spawn()?;

    let status = child.wait()?;

    if !status.success() {
        eprintln!("Command failed with status: {}", status);
        exit(1);
    }

    Ok(())
}

pub fn get_contents_of(file: &str) -> io::Result<String> {
    let mut contents = String::new();
    File::open(file)?.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn yesnoprompt(msg: &str) -> bool {
    let out = prompt(msg).to_lowercase();
    return out == "y" || out.is_empty();
}

pub fn prompt(msg: &str) -> String {
    let mut input = String::new();
    print!("{}", msg);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

pub fn files_in_dir(dir_path: &str, extension: &str) -> io::Result<Vec<String>> {
    let mut toml_files = Vec::new();

    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension() == Some(std::ffi::OsStr::new(extension)) {
            if let Some(file_name) = path.to_str() {
                toml_files.push(file_name.to_string());
            }
        }
    }

    Ok(toml_files)
}

pub fn terminate_on_error<T>(value: Result<T, Error>) -> T {
    if let Err(err) = &value {
        eprintln!("ERROR: {}", err.msg);
        exit(err.code);
    }
    return value.unwrap();
}

pub fn conf_dir() -> String {
    let mut path = dirs::config_dir().unwrap();
    path.push("kaeru");
    path.to_string_lossy().to_string()
}

pub fn conf_file() -> String {
    let mut path = conf_dir();
    path.push_str("/config.toml");
    path
}

pub fn managers_dir() -> String {
    let mut path = conf_dir();
    path.push_str("/manager/");
    path
}

pub fn package_dir() -> String {
    let mut path = conf_dir();
    path.push_str("/packages/");
    path
}

pub fn gen_dir() -> String {
    let mut path = conf_dir();
    path.push_str("/gen/");
    path
}

pub fn create_file_with_contents(file_path: &str, contents: &str) {
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(contents.as_bytes())
        .expect("Failed to write to file");
}

pub fn mkdir_if_not_exists(dir: &str) -> io::Result<()> {
    if !fs::exists(dir)? {
        fs::create_dir_all(dir)?;
    }
    Ok(())
}
