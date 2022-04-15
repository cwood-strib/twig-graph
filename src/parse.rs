use ludtwig_parser::{ast::TwigStatement, parse, SyntaxNode};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{read_to_string, File};
use std::io::Error;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{mpsc, Arc};
use std::thread;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

use crate::context::{AppContext, Message, Connection};

#[derive(Debug)]
pub enum ParseError {
    FileError,
    TwigError,
}


pub fn parse_entry(entry: &DirEntry, mut context: Arc<AppContext>) {
    let path = entry.path().as_os_str().to_str().unwrap_or("");
    // println!("path {}", path);
    // let file = File::open(path)?;
    if let Ok(contents) = read_to_string(path).map_err(|_| ParseError::FileError) {
        if let Ok(tree) = ludtwig_parser::parse(&contents).map_err(|_| ParseError::TwigError) {
            let mut includeStatements: Vec<String> = Vec::new();

            for node in tree.iter() {
                // println!("{:?}", node);
                match node {
                    SyntaxNode::TwigStatement(node) => {
                        // println!("{:?}", node);
                        if let TwigStatement::Raw(raw) = node {
                            // println!("{:?}", raw);
                            // TODO fix this
                            // is this right with a space? idk if it is required
                            if raw.trim_left().starts_with("include ") {
                                // Parsing of actual path needs work to handle quotes, common.* setups, and with {} clauses 

                                let destination = raw.trim().replace("include", "");
                                context
                                    .output
                                    .send(Message::Connection(Connection {
                                        from: path.to_string(),
                                        to: destination,
                                        // Can we get rid of clone?
                                        cwd: context.current_dir.clone(),
                                        root_dir: context.root_dir.to_string()
                                    }))
                                    .unwrap_or(());
                                // Include points from current to included destination
                                // if let Some(records) = context.graph.get_mut(path) {

                                // }
                                // if let Some(records) = context) {
                                //     if let Some(existing) = records.graph.get_mut(path) {
                                //         println!("has existing");
                                //         existing.push(path.to_string());
                                //     } else {
                                //         println!("inserting");
                                //         records.graph.insert(path.to_string(), vec!(raw.trim().to_string()));
                                //     }
                                // }
                            } 
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}