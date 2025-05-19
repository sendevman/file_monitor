use anyhow::Result;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher, Event, EventKind};
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tokio::signal;
use std::sync::mpsc as std_mpsc;

pub async fn watch_inbox(inbox_path: PathBuf) -> Result<()> {
    let (tx, mut rx) = mpsc::unbounded_channel();

    let (std_tx, std_rx) = std_mpsc::channel();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            std_tx.send(res).expect("failed to send event over std channel");
        },
        Config::default(),
    )?;

    watcher.watch(&inbox_path, RecursiveMode::Recursive)?;

    println!("Started watching folder: {}", inbox_path.display());

    tokio::task::spawn_blocking(move || {
        while let Ok(event) = std_rx.recv() {
            let _ = tx.send(event);
        }
    });

    let ctrl_c = signal::ctrl_c();
    tokio::pin!(ctrl_c);

    loop {
        tokio::select! {
            maybe_event = rx.recv() => {
                if let Some(event_res) = maybe_event {
                    match event_res {
                        Ok(event) => {
                            print_event(&event, &inbox_path);
                        }
                        Err(e) => eprintln!("Watch error: {:?}", e),
                    }
                } else {
                    break;
                }
            }
            _ = &mut ctrl_c => {
                println!("\nCtrl-C detected, stopping watcher...");
                break;
            }
        }
    }

    Ok(())
}

fn print_event(event: &Event, inbox_path: &Path) {
    let inbox_str = inbox_path.to_string_lossy();
    for path in &event.paths {
        let path_str = path.to_string_lossy();
        if let Some(pos) = path_str.find(&*inbox_str) {
            let rel_str = &path_str[pos + inbox_str.len()..];
            let event_str = match event.kind {
                EventKind::Create(_) => "[NEW]",
                EventKind::Modify(_) => "[MOD]",
                EventKind::Remove(_) => "[DEL]",
                _ => return,
            };

            println!("{} {}", event_str, rel_str);
        }
    }
}
