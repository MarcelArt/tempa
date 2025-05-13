use std::{path::Path, process::Command, sync::mpsc};

use notify::{event::{AccessKind, AccessMode, RemoveKind}, Error, EventKind, RecursiveMode, Watcher};

fn rebuild() -> Result<(), Error> {
    let output = Command::new("cargo")
        .arg("build")
        .arg("-q")
        .output()?;

    if !output.status.success() {
        eprintln!("Error building GDExtension");
    }

    Ok(())
}

pub fn execute() -> Result<(), String> {
    println!("Watching for changes...");
    println!("Press <Ctrl+C> to stop.");

    let (tx, rx) = mpsc::channel::<notify::Result<notify::Event>>();

    let mut watcher = notify::recommended_watcher(tx).map_err(|e| e.to_string())?;

    watcher.watch(Path::new("./src"), RecursiveMode::Recursive).map_err(|e| e.to_string())?;

    for res in rx {
        match res {
            Ok(event) => {
                // println!("event: {:?}", event);
                if let EventKind::Access(AccessKind::Close(AccessMode::Write)) | EventKind::Remove(RemoveKind::File) = event.kind {
                    println!("Rust code modified, rebuilding GDExtension...");
                    rebuild().map_err(|e| e.to_string())?;
                    println!("GDExtension built successfully!");
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}