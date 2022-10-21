pub mod config;
use colored::Colorize;
use config::ProConfig;
use std::process::{exit, Command};
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
};

#[derive(Debug, Clone, Copy)]
enum ProjectType {
    Typescript,
    Rust,
    Elixir,
    Clojure,
    ClojureScript,
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

fn get_projects(dir_url: &str) -> Vec<std::string::String> {
    let dirs = fs::read_dir(dir_url).unwrap();
    dirs.filter_map(|f| f.ok())
        .filter(|f| f.file_type().unwrap().is_dir() && !is_hidden(f))
        .map(|f| f.file_name().into_string().unwrap())
        .collect::<Vec<String>>()
}

fn remove_project(project_name: &String, dir_url: &String) {
    
    let result = Command::new("rm")
        .arg("-rf")
        .arg(format!("{}/{}", dir_url, project_name))
        .output();

    match result {
        Ok(..) => {
            println!("Project {} succesfully removed", project_name);
        },
        Err(e) => {
            eprintln!("Can't remove {} because of {}", project_name, e);
            exit(1);
        }
    }
}

fn list_dir(dir_url: &str) {
    let projects = get_projects(dir_url);
    let max_len_pr = projects
        .iter()
        .fold(0, |acc, v| if v.len() > acc { v.len() } else { acc });

    projects.into_iter().for_each(|f| {
        use ProjectType::*;
        let project_type = get_project_language(&f, dir_url);
        if let Some(pr_type) = project_type {
            let space_count = max_len_pr - f.len();
            let colored_pr_type = match pr_type {
                Typescript => format!("{:?}", pr_type).blue(),
                Rust => format!("{:?}", pr_type).red(),
                Elixir => format!("{:?}", pr_type).purple(),
                Clojure => format!("{:?}", pr_type).green(),
                ClojureScript => format!("{:?}", pr_type).green(),
            };
            println!(
                "\t{} {} - {}",
                f.bold(),
                " ".repeat(space_count),
                colored_pr_type
            );
        } else {
            println!("\t{}", f.bold());
        }
    })
}

fn open_project(project_name: &String, dir_url: &str, code_editor: &str, editor_flags: Vec<String>) {
    if get_projects(dir_url).contains(project_name) {
        let result = Command::new(code_editor)
            .current_dir(format!("{}/{}", dir_url, project_name))
            .arg(".")
            .args(editor_flags)
            .output();

        match result {
            Ok(..) => {},
            Err(e) => {
                eprintln!("Can't open project '{}' in editor because of {}", project_name, e);
                exit(1);
            }
        }
    } else {
        println!("Project with provided name does not exists");
    }
}

fn create_project(project_name: &str, dir_url: &str) {
    let result = Command::new("pnpx")
        .arg("degit")
        .arg("tyrkinn/frontend-templates/chakra-jotai-vitest")
        .arg(format!("{}/{}", dir_url, project_name))
        .output();

    match result {
        Ok(..) => {},
        Err(e) => {
            eprintln!("Can't create project '{}' because of {}", project_name, e);
            exit(1)
        }
    }
}

fn get_project_path(project_name: &str, projects_dir: &str) {
    println!("{}/{}", projects_dir, project_name);
}

fn get_project_language(project_name: &str, projects_dir: &str) -> Option<ProjectType> {
    let projects_hashmap: HashMap<&str, ProjectType> = HashMap::from([
        ("tsconfig.json", ProjectType::Typescript),
        ("Cargo.toml", ProjectType::Rust),
        ("mix.exs", ProjectType::Elixir),
        ("deps.edn", ProjectType::Clojure),
        ("project.clj", ProjectType::Clojure),
        ("shadow-cljs.edn", ProjectType::ClojureScript),
    ]);
    let project_full_path: String = format!("{}/{}", projects_dir, project_name);
    let project_files = fs::read_dir(project_full_path)
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    for &key in projects_hashmap.keys() {
        if project_files.contains(&key.to_owned()) {
            return Some(projects_hashmap[key]);
        }
    }
    None
}

fn prepare_config() -> ProConfig {
    if !config::file_exists(config::config_path()) {
        let projects_path = config::at_home("projects");
        let default_config = ProConfig {
            project_path: projects_path.to_owned(),
            code_editor: "neovide".to_string(),
            editor_flags: Vec::new()
        };
        config::create_config_file();
        config::write_config(&default_config);
        match fs::create_dir_all(projects_path) {
            Ok(..) => {},
            Err(e) => {
                eprintln!("Can't create config dir because of {}", e);
                exit(1)
            }
        }
        default_config
    } else {
        config::read_config()
    }
}

fn display_help_message() {
    println!(
        r#"
Usage:
    pro list                  -> List projects
    pro create <PROJECT_NAME> -> Create project
    pro path <PROJECT_NAME>   -> Get full project path
    pro remove <PROJECT_NAME> -> Remove project
    pro open <PROJECT_NAME>   -> Open project in vscode
    pro help                  -> Display this message"#
    );
}

fn main() {
    let config = prepare_config();

    let args: Vec<String> = std::env::args().skip(1).collect();

    let str_args: Vec<&str> = args.iter().map(|v| &v[..]).collect();

    let pr_dir = config.project_path;

    match str_args[..] {
        ["list"] => list_dir(&pr_dir),
        ["open", pr_name] => open_project(&pr_name.to_owned(), &pr_dir, &config.code_editor, config.editor_flags),
        ["path", pr_name] => get_project_path(pr_name, &pr_dir),
        ["create", pr_name] => create_project(pr_name, &pr_dir),
        ["remove", pr_name] => remove_project(&pr_name.to_owned(), &pr_dir),
        ["help"] => display_help_message(),
        _ => println!("Run `pro help` to get usage info"),
    }
}
