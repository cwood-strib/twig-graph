mod context;
mod parse;

use std::env;
use std::io::Error;
use rayon::prelude::*;
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{mpsc, Arc};
use std::thread;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn is_twig(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.ends_with(".twig"))
        .unwrap_or(false)
}

fn handle_input_path(entry: DirEntry, context: Arc<context::AppContext>) {
    let path = entry.path().as_os_str().to_str().unwrap_or("");
    let walker = WalkDir::new(path).into_iter();

    rayon::scope(move |s| {
        for entry in walker
            .into_iter()
            .filter_entry(|entry| is_twig(entry) || is_dir(entry))
        {
            match entry {
                Ok(e) => {
                    if is_twig(&e) {
                        let clone = Arc::clone(&context);
                        s.spawn(move |_s1| parse::parse_entry(&e, clone));
                    } else {
                        ()
                    }
                }
                Err(_) => (),
            };  
        }
    });
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let cwd = env::current_dir().unwrap_or(PathBuf::new());
    println!("{:?}", args[1]);

    let (tx, rx) = mpsc::sync_channel(32);

    let context = Arc::new(context::AppContext { 
        output: tx,
        current_dir: cwd,
        root_dir: args[1].to_string()
    });

    let output_handler = thread::spawn(|| context::handle_context(rx));

    for entry in WalkDir::new(&args[1])
        .into_iter()
        .filter_entry(|entry| is_twig(entry) || is_dir(entry))
    {
        match entry {
            Ok(entry) => handle_input_path(entry, Arc::clone(&context)),
            Err(_) => ()
        }
    }

    println!("{:?}", context);

    drop(context);

    let _ = output_handler.join().unwrap();

    Ok(())
}
