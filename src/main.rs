use clap::Parser;
use std::{
    fs::{self, DirEntry},
    process::Command,
};

const PROJECT_DIR_URL: &str = "/Users/tyrkinn/github/tyrkinn";
const TYPESCRIPT_INDICATOR_FILE: &str = "tsconfig.json";
const RUST_INDICATOR_FILE: &str = "Cargo.toml";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, default_value = PROJECT_DIR_URL)]
    project_dir: String,

    #[clap(short, long)]
    list_projects: bool,

    #[clap(short, long)]
    open_project: Option<String>,
}

enum ProjectType {
    Typescript,
    Rust
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn get_projects(dir_url: &String) -> Vec<std::string::String> {
    let dirs = fs::read_dir(dir_url).unwrap();
    dirs.filter_map(|f| f.ok())
        .filter(|f| f.file_type().unwrap().is_dir() && !is_hidden(f))
        .map(|f| f.file_name().into_string().unwrap())
        .collect::<Vec<String>>()
}

fn list_dir(dir_url: &String) {
    get_projects(dir_url)
        .into_iter()
        .for_each(|f| {
            let project_type = get_project_language(&f);
            if project_type.is_some() {
                let project_lang = match project_type.unwrap() {
                    ProjectType::Typescript => "Typescript",
                    ProjectType::Rust => "Rust"
                };
                println!("{} - {}", f, project_lang);
            }
            else {
                println!("{}", f);
            }
        })
}

fn open_project(project_name: &String, dir_url: &String) {
    if get_projects(dir_url).contains(project_name) {
        Command::new("code")
            .arg("-r")
            .arg(format!("{}/{}", dir_url, project_name))
            .output()
            .expect("Error");
    } else {
        println!("Project with provided name does not exists");
    }
}

fn get_project_language(project_name: &String) -> Option<ProjectType> {
    let project_full_path: String = format!("{}/{}", PROJECT_DIR_URL, project_name);
    let project_files = fs::read_dir(project_full_path)
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>();
    if project_files.contains(&TYPESCRIPT_INDICATOR_FILE.to_owned()) {
        return Some(ProjectType::Typescript);
    } else if project_files.contains(&RUST_INDICATOR_FILE.to_owned()) {
        return Some(ProjectType::Rust);
    } else {
        return None;
    }
}

fn main() {
    let args = Args::parse();
    if args.list_projects {
        list_dir(&args.project_dir)
    } else if args.open_project.is_some() {
        open_project(&args.open_project.unwrap(), &args.project_dir);
    }
}
