pub mod config;
pub mod tui;
pub mod website;
use colored::{ColoredString, Colorize};
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

fn get_projects(dir_url: &str) -> Vec<String> {
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
        Ok(..) => println!("Project {} succesfully removed", project_name),
        Err(e) => {
            eprintln!("Can't remove {} because of:\n{}", project_name, e);
            exit(1);
        }
    }
}

fn pr_type_to_str(pr_type: ProjectType) -> ColoredString {
    use ProjectType::*;
    match pr_type {
        Typescript => format!("{:?}", pr_type).blue(),
        Rust => format!("{:?}", pr_type).red(),
        Elixir => format!("{:?}", pr_type).purple(),
        Clojure => format!("{:?}", pr_type).green(),
        ClojureScript => format!("{:?}", pr_type).green(),
    }
}

fn format_typed_pr(pr_name: &str, space_count: usize, str_pr_type: ColoredString) -> String {
    format!(
        "\t{} {} - {}",
        pr_name.bold(),
        " ".repeat(space_count),
        str_pr_type
    )
}

fn format_untyped_pr(pr_name: &str) -> String {
    format!("\t{}", pr_name.bold())
}

fn proj_to_str(pr_name: &str, space_count: usize, project_type: Option<ProjectType>) -> String {
    match project_type {
        Some(pr_type) => {
            let str_pr_type = pr_type_to_str(pr_type);
            format_typed_pr(pr_name, space_count, str_pr_type)
        }
        None => format_untyped_pr(pr_name),
    }
}

fn gen_comps(project_dir: &str) {
    let comp = format!(
        r#"
_pro() {{
    local line state

    _arguments -C \
               "1: :->cmds" \
               "*::arg:->args"
    case "$state" in
        cmds)
            _values "pro command" \
                    "list[list all projects in project directory]" \
                    "create[create new project from template]" \
                    "open[open project in editor]" \
                    "remove[rm -rf directory]"  \
                    "path[get full project path]" \
                    "help[display help message]" \
                    "version[display version]" \
            ;;
        args)
            case $line[1] in
                path | remove | open)
                    _select_project_cmd
                    ;;
            esac
            ;;
    esac
}}
_select_project_cmd() {{
     local oomph_dirs oomph_dirs=(-/ {})
     _files -W oomph_dirs -g '*'
}}
compdef _pro pro
"#,
        project_dir
    );
    println!("{}", comp);
}

fn list_dir(dir_url: &str, lang: Option<&str>) {
    let projects = get_projects(dir_url);
    let max_len_pr = projects
        .iter()
        .fold(0, |acc, v| if v.len() > acc { v.len() } else { acc });

    if let Some(lang_raw) = lang {
        projects.into_iter().for_each(|f| {
            let project_type = get_project_language(&f, dir_url);
            if let Some(pr_type) = project_type {
                let pr_type_str = format!("{pr_type:?}").to_lowercase();
                if pr_type_str == lang_raw {
                    println!("{}", proj_to_str(&f, max_len_pr - f.len(), project_type))
                }
            }
        });
        return;
    }

    projects.into_iter().for_each(|f| {
        let project_type = get_project_language(&f, dir_url);
        println!("{}", proj_to_str(&f, max_len_pr - f.len(), project_type))
    });
}

fn check_exists(project_dir: &str, project_name: &str) {
    if !get_projects(project_dir).contains(&project_name.to_string()) {
        eprintln!("Project with provided name does not exists");
        std::process::exit(1);
    }
}

fn open_project(project_name: &str, dir_url: &str, code_editor: &str, editor_flags: Vec<String>) {
    check_exists(dir_url, project_name);
    let current_dir = format!("{}/{}", dir_url, project_name);
    println!("{}", code_editor);

    let mut cmd = &mut Command::new(code_editor);

    cmd.current_dir(&current_dir) // For neovim to open project dir correctly
        .arg(&current_dir);

    for flag in editor_flags {
        cmd = cmd.arg(flag);
    }

    let result = cmd.output();

    result.unwrap_or_else(|e| {
        eprintln!(
            "Can't open project '{}' in editor because of:\n{}",
            project_name, e
        );
        exit(1);
    });
}

fn create_project(project_name: &str, dir_url: &str) {
    let result = Command::new("pnpx")
        .arg("degit")
        .arg("tyrkinn/frontend-templates/chakra-jotai-vitest")
        .arg(format!("{}/{}", dir_url, project_name))
        .output();

    result.unwrap_or_else(|e| {
        eprintln!("Can't create project '{}' because of {}", project_name, e);
        exit(1)
    });
}

fn get_project_path(project_name: &str, projects_dir: &str) -> String {
    format!("{}/{}", projects_dir, project_name)
}

fn read_dir_files(dir_path: &str) -> Vec<String> {
    fs::read_dir(dir_path)
        .unwrap()
        .map(|x| x.unwrap().file_name().into_string().unwrap())
        .collect()
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
    let project_full_path: String = get_project_path(&project_name, &projects_dir);
    let project_files = read_dir_files(&project_full_path);

    for &key in projects_hashmap.keys() {
        if project_files.contains(&key.to_owned()) {
            return Some(projects_hashmap[key]);
        }
    }
    None
}

fn create_project_dir(project_dir: &str) {
    fs::create_dir_all(project_dir).unwrap_or_else(|e| {
        eprintln!("Can't create config dir because of:\n{}", e);
        exit(1)
    });
}

fn create_default_config(projects_dir: &str) -> ProConfig {
    let default_config = ProConfig {
        project_path: projects_dir.to_owned(),
        code_editor: "neovide".to_string(),
        editor_flags: Vec::new(),
    };
    config::create_config_file();
    config::write_config(&default_config);
    default_config
}

fn prepare_config() -> ProConfig {
    if !config::file_exists(config::config_path()) {
        let projects_path = config::at_home("projects");
        create_project_dir(&projects_path);
        create_default_config(&projects_path)
    } else {
        config::read_config()
    }
}

fn display_help_message() {
    println!(
        r#"
Pro CLI v0.1.0

Usage:
    pro version               -> Display Pro Cli version
    pro list [-lang LANGUAGE] -> List projects (filter by language if `-lang` flag provided)
    pro create <PROJECT_NAME> -> Create project
    pro path <PROJECT_NAME>   -> Get full project path
    pro remove <PROJECT_NAME> -> Remove project
    pro open <PROJECT_NAME>   -> Open project in vscode
    pro help                  -> Display this message
    pro comps                 -> Display zsh completions
    pro tui                   -> TODO: TUI interface"#
    );
}

fn main() {
    let config = prepare_config();

    let args: Vec<String> = std::env::args().skip(1).collect();

    let str_args: Vec<&str> = args.iter().map(String::as_str).collect();

    let pr_dir = config.project_path;

    match str_args[..] {
        ["list"] => list_dir(&pr_dir, None),
        ["list", "-lang", lang] => list_dir(&pr_dir, Some(lang)),
        ["version"] => println!("Pro CLI\nVersion: {}", env!("CARGO_PKG_VERSION")),
        ["open", pr_name] => open_project(
            &pr_name.to_owned(),
            &pr_dir,
            &config.code_editor,
            config.editor_flags,
        ),
        ["path", pr_name] => println!("{}", get_project_path(pr_name, &pr_dir)),
        ["create", pr_name] => create_project(pr_name, &pr_dir),
        ["remove", pr_name] => remove_project(&pr_name.to_owned(), &pr_dir),
        ["help"] => display_help_message(),
        ["comps"] => gen_comps(&pr_dir),
        _ => println!("Run `pro help` to get usage info"),
    }
}
