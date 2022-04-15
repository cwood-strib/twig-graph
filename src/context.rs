use std::collections::{HashMap, HashSet};
use walkdir::{DirEntry, WalkDir};
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::fs;
use std::path::PathBuf;
use std::thread;


pub struct Connection {
    pub from: String,
    pub to: String,
    pub cwd: PathBuf,
    pub root_dir: String
}

type FilePath = String;

pub enum Message {
    Connection(Connection),
    Failure(FilePath)
}

#[derive(Debug)]
pub struct AppContext {
    pub output: SyncSender<Message>,
    pub root_dir: String,
    pub current_dir: PathBuf,
}

// THIS IS WRONG
fn make_absolute(relative_path: String, cwd: &PathBuf, root: String) -> String {
    let mut absolute = PathBuf::new();
    // CWD
    absolute.push(cwd);
    absolute.push(root);
    absolute.push(relative_path);

    match fs::canonicalize(absolute) {
        Ok(path) => path.to_str().unwrap_or("").to_string(),
        Err(_) => "".to_string()
    }
}

pub fn handle_context(rx: Receiver<Message>) -> i32 {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    let errors: Vec<FilePath> = Vec::new();
    let mut visited_count = 0;

    // receive all incoming messages until all sending ends are closed.
    while let Ok(msg) = rx.recv() {
        visited_count = visited_count + 1;

        match msg {
            Message::Connection(conn) => {
                if let Some(records) = graph.get_mut(&conn.from) {
                    records.push(conn.to);
                } else {
                    graph.insert(conn.from.to_string(), vec!(conn.to));
                }
            },
            Message::Failure(path) => {
                println!("failed on {}", path);
            }
        }
    }


    // println!("{:?}", graph);
    for key in graph.keys() {
        println!("{} => {:?}", key, graph.get(key).unwrap_or(&Vec::new()))
    }

    println!("Processed {} template files", visited_count);
    1
}