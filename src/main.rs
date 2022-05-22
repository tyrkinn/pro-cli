use std::{fs::{self, DirEntry}, process::Command, fmt::format, os::unix::prelude::CommandExt};
use clap::Parser;

const PROJECT_DIR_URL: &str = "/Users/tyrkinn/github/tyrkinn";

/// Cli to list and open projects
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory where projects are stored
    #[clap(short, long, default_value = PROJECT_DIR_URL)]
    project_dir: String,

    /// List projects
    #[clap(short, long)]
    list_projects: bool,

    /// Provide project name to open in vscode
    #[clap(short, long)]
    open_project: Option<String>,
}



fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn get_projects(dir_url: String) -> Vec<std::string::String> {
    let dirs = fs::read_dir(dir_url).unwrap();
    dirs.filter_map(|f| f.ok())
        .filter(|f| f.file_type().unwrap().is_dir() && !is_hidden(f))
        .map(|f| f.file_name().into_string().unwrap())
        .collect::<Vec<String>>()
}

fn list_dir(dir_url: String) {
        get_projects(dir_url)
        .into_iter()
        .for_each(|f| println!("{}", f))
}

fn open_project(project_name: String, dir_url: String) {
    println!("{:?}", get_projects(String::from(PROJECT_DIR_URL)));
    if get_projects(String::from(PROJECT_DIR_URL)).contains(&project_name) {
        Command::new("code")
        .arg("-r")
        .arg(format!("{}/{}", dir_url, project_name))
        .output()
        .expect("Error");
    } 
    else {
        println!("Project with provided name does not exists");
    }
}

fn main() {
    let args = Args::parse();
    if args.list_projects {
        list_dir(args.project_dir.clone())
    }
    if args.open_project.is_some() {
        open_project(args.open_project.unwrap(), args.project_dir);
    }
}
