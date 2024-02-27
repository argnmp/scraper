use std::{error::Error, fs, io::Write, path::PathBuf, thread::{self, sleep}, time::Duration};

use clap::Parser;
use clipboard::Clipboard;
use io::Io;
use template::Template;
use tokio::{io::AsyncWriteExt, sync::{mpsc, oneshot}};

mod template;
mod io;
mod clipboard;

#[derive(Parser)]
struct Cli{
    template: Option<String>,
    
    #[arg(short, long, value_name = "FILE_PATH")]
    file: Option<PathBuf>
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cli = Cli::parse();
    let (sender, mut receiver) = mpsc::channel(100);
    let (clipboard_sender, clipboard_receiver) = mpsc::channel(100);

    let mut io = Io::new(sender.clone()); 
    let mut clipboard = Clipboard::new(sender.clone(), clipboard_receiver)?; 

    let io_handle = thread::spawn(move || -> Result<(), Box<dyn Error + Sync + Send>> {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(1)
            .enable_all()
            .build()?
            .block_on(async {
                io.run().await.unwrap();
            });
        Ok(())
    });
    let clipboard_handle = thread::spawn(move || -> Result<(), Box<dyn Error + Sync + Send>> {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()?
            .block_on(async {
                clipboard.run().await.unwrap();
            });
        Ok(())
    });

    let mut template = match cli.template {
        Some(raw) => {
            Template::new(raw)?
        },
        None => {
            match cli.file {
                Some(path) => {
                    let raw = fs::read_to_string(path)?;
                    Template::new(raw)?
                },
                None => {
                    return Err("Specify template or template file path".into());
                }
            }
        }
    };

    let core_handle = thread::spawn(move || -> Result<(), Box<dyn Error + Sync + Send>> {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async {
                'out: loop {
                    if let Some(name) = template.get_current_target() {
                        print!("[capturing {} ...]\n", name);
                        print!(">> ");
                        std::io::stdout().flush().unwrap();
                    }
                    else {
                        println!("[capture finished, send to clipboard]");
                        clipboard_sender.send(clipboard::ClipboardCommand::Set(template.get_buf().to_string())).await.unwrap();
                        
                        template.reset();
                        // clipboard_sender.send(clipboard::ClipboardCommand::Exit).await.unwrap();
                        continue;
                    }

                    match template.check_resolved() {
                        Some(s) => {
                            println!("\n{}", s); 
                        },
                        None => {
                            match receiver.recv().await {
                                Some(s) => {
                                    if let Some(text) = template.replace(&s) {
                                        println!("\n{}", text); 
                                    }
                                    else {
                                        break 'out;
                                    }

                                },
                                None => {
                                }
                            }
                        },
                    }
                }
            });
        Ok(())
    });

    // do not wait for io
    let _ = io_handle.join().unwrap();
    let _ = clipboard_handle.join().unwrap();
    let _ = core_handle.join().unwrap();
    
    Ok(())
}
