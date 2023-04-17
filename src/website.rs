use serde_json::{json};
use std::{collections::HashMap, fs::File, io::Cursor};
use url::Url;

use tiny_http::{Header, Response, Server};

use crate::{config::ProConfig, get_projects, open_project, remove_project};

type JsonResp = Response<Cursor<Vec<u8>>>;

fn err_response(message: &str) -> JsonResp {
    let json_header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
    let error_json = format!("{{\"error\": \"{}\"}}", message);
    let response = Response::from_string(error_json)
        .with_header(json_header)
        .with_status_code(500);
    return response;
}

fn ok_reponse(message: &str) -> JsonResp {
    let json_header = Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..]).unwrap();
    let error_json = format!("{{\"message\": \"{}\"}}", message);
    let response = Response::from_string(error_json)
        .with_header(json_header)
        .with_status_code(200);
    return response;
}

pub fn start_server(config: ProConfig) -> Result<(), ()> {
    let url = "0.0.0.0:8000";
    let server = Server::http(url).unwrap();
    println!("INFO: Starting server at {url}");
    for request in server.incoming_requests() {
        let endpoint = request.url();
        match endpoint {
            "/" => {
                let file = File::open("public/index.html").map_err(|err| {
                    eprintln!("ERROR: Can't open file because of {err}");
                })?;
                let response = Response::from_file(file);
                let _ = request.respond(response);
            }
            x if x.starts_with("/open") => {
                let full_url = "https://".to_owned() + url + endpoint;
                let parsed_url = Url::parse(&full_url).map_err(|_| eprintln!("Can't parse url"))?;
                let mut params: HashMap<String, String> = HashMap::new();
                for (k, v) in parsed_url.query_pairs() {
                    params.insert(k.to_string(), v.to_string());
                }
                if !params.contains_key("project") {
                    let _ =
                        request.respond(err_response("You should provide 'project' query param"));
                    continue;
                }
                let open = open_project(
                    params.get("project").unwrap(),
                    &config.project_path,
                    &config.code_editor,
                    config.editor_flags.to_owned(),
                );
                if open.is_err() {
                    let _ =
                        request.respond(err_response("Project With given name does not exists"));
                } else {
                    let _ = request.respond(ok_reponse("ok"));
                }
            }
            "/list" => {
                let projects: Vec<_> = get_projects(&config.project_path);
                let _json_response = json!({ "projects": projects, "count": projects.len() });
                
            }
            x if x.starts_with("/remove") => {
                let full_url = "https://".to_owned() + url + endpoint;
                let parsed_url = Url::parse(&full_url).map_err(|_| eprintln!("Can't parse url"))?;
                let mut params: HashMap<String, String> = HashMap::new();
                for (k, v) in parsed_url.query_pairs() {
                    params.insert(k.to_string(), v.to_string());
                }
                if !params.contains_key("project") {
                    let _ =
                        request.respond(err_response("You should provide 'project' query param"));
                    continue;
                }
                let remove = remove_project(params.get("project").unwrap(), &config.project_path);
                if remove.is_err() {
                    let _ =
                        request.respond(err_response("Project With given name does not exists"));
                } else {
                    let _ = request.respond(ok_reponse("ok"));
                }
            }

            _ => {
                let response = Response::from_string("Not found");
                let _ = request.respond(response);
            }
        }
    }
    Ok(())
}
