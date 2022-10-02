pub mod config;
use config::ProConfig;
use std::process;
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

fn remove_project(project_name: &String, dir_url: &String) {
    process::Command::new("rm")
        .arg("-rf")
        .arg(format!("{}/{}", dir_url, project_name))
        .output()
        .expect("Cannot remove project with given name");
}

fn list_dir(dir_url: &String) {
    get_projects(dir_url).into_iter().for_each(|f| {
        let project_type = get_project_language(&f, dir_url);
        if project_type.is_some() {
            println!("{} - {:?}", f, project_type.unwrap());
        } else {
            println!("{}", f);
        }
    })
}

fn open_project(project_name: &String, dir_url: &String) {
    if get_projects(dir_url).contains(project_name) {
        process::Command::new("code")
            .arg("-r")
            .arg(format!("{}/{}", dir_url, project_name))
            .output()
            .expect("Error");
    } else {
        println!("Project with provided name does not exists");
    }
}

fn create_project(project_name: &String, dir_url: &String) {
    process::Command::new("pnpx")
        .arg("degit")
        .arg("tyrkinn/frontend-templates/chakra-jotai-vitest")
        .arg(format!("{}/{}", dir_url, project_name))
        .output()
        .expect("Error occured while creating project");
}

fn get_project_path(project_name: &String, projects_dir: &String) {
    println!("{}/{}", projects_dir, project_name);
}

fn get_project_language(project_name: &String, projects_dir: &String) -> Option<ProjectType> {
    let projects_hashmap: HashMap<&str, ProjectType> = HashMap::from([
        ("tsconfig.json", ProjectType::Typescript),
        ("Cargo.toml", ProjectType::Rust),
        ("mix.exs", ProjectType::Elixir),
        ("deps.edn", ProjectType::Clojure),
        ("project.clj", ProjectType::Clojure),
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
    return None;
}

fn prepare_config() -> ProConfig {
    if !config::file_exists(config::config_path()) {
        let projects_path = config::at_home("projects");
        let default_config = ProConfig {
            project_path: projects_path.to_owned(),
        };
        config::create_config_file();
        config::write_config(&default_config);
        fs::create_dir_all(projects_path).expect("Can't create projects dir");
        return default_config;
    } else {
        return config::read_config();
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

    let pr_dir = config.project_path.to_owned();

    match str_args[..] {
        ["list"] => list_dir(&pr_dir),
        ["open", pr_name] => open_project(&pr_name.to_owned(), &pr_dir),
        ["path", pr_name] => get_project_path(&pr_name.to_owned(), &pr_dir),
        ["create", pr_name] => create_project(&pr_name.to_owned(), &pr_dir),
        ["remove", pr_name] => remove_project(&pr_name.to_owned(), &pr_dir),
        ["help"] => display_help_message(),
        _ => println!("Run `pro help` to get usage info"),
    }
}
