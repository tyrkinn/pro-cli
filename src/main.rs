use clap::{arg, Arg, Command};
use std::process;
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
};
const PROJECT_DIR_URL: &str = "/Users/tyrkinn/github/tyrkinn";
fn config_args<'a>() -> Command<'a> {
    // TODO: Rewrite using "arg!" macro
    Command::new("Pro cli")
        .about("Simple cli to manage and create projects")
        .arg(
            Arg::new("dir")
                .long("dir")
                .short('d')
                .takes_value(true)
                .default_value(PROJECT_DIR_URL),
        )
        .arg(
            Arg::new("list_projects")
                .long("list")
                .short('l')
                .takes_value(false),
        )
        .arg(
            Arg::new("open_project")
                .long("open")
                .short('o')
                .takes_value(true),
        )
        .arg(
            Arg::new("get_path")
                .short('p')
                .long("path")
                .takes_value(true),
        )
        .arg(
            Arg::new("create_project")
                .short('c')
                .long("create")
                .takes_value(true),
        )
        .arg(arg!(-r --remove <PROJECT_NAME> "Remove project"))
}

#[derive(Debug, Clone, Copy)]
enum ProjectType {
    Typescript,
    Rust,
    Elixir,
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
        let project_type = get_project_language(&f);
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

fn get_project_language(project_name: &String) -> Option<ProjectType> {
    let projects_hashmap: HashMap<&str, ProjectType> = HashMap::from([
        ("tsconfig.json", ProjectType::Typescript),
        ("Cargo.toml", ProjectType::Rust),
        ("mix.exs", ProjectType::Elixir),
    ]);
    let project_full_path: String = format!("{}/{}", PROJECT_DIR_URL, project_name);
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

fn main() {
    // TODO: Get rid of else-if statements

    let args = config_args().get_matches();
    let projects_dir = args.value_of("dir").unwrap().to_owned();
    if args.is_present("list_projects") {
        list_dir(&projects_dir);
    } else if args.value_of("open_project").is_some() {
        let project_name = args.value_of("open_project").unwrap();
        open_project(&project_name.to_owned(), &projects_dir);
    } else if args.value_of("get_path").is_some() {
        let project_path = args.value_of("get_path").unwrap();

        get_project_path(&project_path.to_owned(), &projects_dir);
    } else if args.value_of("create_project").is_some() {
        let project_path = args.value_of("create_project").unwrap();
        create_project(&project_path.to_owned(), &projects_dir)
    } else if args.value_of("remove").is_some() {
        let project_path = args.value_of("remove").unwrap();
        remove_project(&project_path.to_owned(), &projects_dir)
    }
}
