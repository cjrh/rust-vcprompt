use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::collections::HashMap;

pub fn venv_str() -> String {
    match env::var("VIRTUAL_ENV") {
        Ok(pth) => {
            let venv_path = Path::new(&pth);
            let mut file = match File::open(venv_path.join("pyvenv.cfg")) {
                Err(_why) => return "".to_string(),
                Ok(file) => file
            };
            let mut s = String::new();
            let python_version = match file.read_to_string(&mut s) {
                Err(_why) => return "".to_string(),
                Ok(_) => {
                    let mut h: HashMap<&str, &str> = HashMap::new();
                    let lines = s.split("\n");
                    for line in lines {
                        let items = line.split("=").collect::<Vec<&str>>();
                        if items.len() < 2 {
                            continue
                        };
                        h.insert(items[0].trim(), items[1].trim());
                    }
                    match h.get(&"version") {
                        Some(&ver) => ver,
                        _ => ""
                    }
                }

            };

            // Shorten the path to the venv
            let mut path_buffer = PathBuf::from(&pth);
            let mut parts = Vec::new();
            for p in path_buffer.components() {
                parts.push(p
                    .as_os_str()
                    .to_string_lossy()
                    .chars()
                    .nth(0)
                    .unwrap()
                    .to_string()
                );
            }
            format!(" [venv:{{red}}{}{{reset}}:{}]", parts.join("/"), python_version)
        }
        Err(_e) => "".to_string()
    }
}
