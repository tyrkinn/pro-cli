# Pro cli

Simple cli for local project management

## Usage

```shell
  $ pro help                  # Get help message
  $ pro list                  # List projects
  $ pro create <PROJECT_NAME> # Bootstrap project from basic template (will be fixed hopefully)
  $ pro path   <PROJECT_NAME> # Get project absolute path
  $ pro remove <PROJECT_NAME> # Remove project directory
  $ pro open   <PROJECT_NAME> # Open project in code editor
  $ pro comps                 # Prints zsh comletions in stdout
```

## Configuration 

With first start Pro-cli will create config file in `YOUR_HOME_FOLDER/.config/pro/config.toml`

Config should have two fields: `project_path` and `code_editor`

Example config: 
```toml
project_path = "/home/projects/"
code_editor = "nvim"
```

## Features implemented

- [x] Open project with code editor
- [x] List projects in project dir
- [x] Bootstrap projects from template
- [x] Deleting project folder
- [x] Configuration in TOML file
- [x] User friendly error messages
- [x] Command line autosuggestions for projects
- [ ] User friendly installation. Add to cargo repo



# Build from source

```shell
  $ cargo build --release --bin pro --out-dir YOUR_BIN_DIR -Z unstable-options
```
