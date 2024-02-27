use std::{error::Error, sync::Arc, time::Duration};

use tokio::{sync::{mpsc, oneshot, Mutex}, task::JoinHandle, time::sleep};

pub enum ClipboardCommand {
    Exit,
    Set(String),
}
pub struct Clipboard {
    cb: arboard::Clipboard, 
    sender: mpsc::Sender<String>,
    receiver: mpsc::Receiver<ClipboardCommand>,
    current_text: Arc<Mutex<String>>,
}
impl Clipboard {
    pub fn new(sender: mpsc::Sender<String>, receiver: mpsc::Receiver<ClipboardCommand>) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            cb: arboard::Clipboard::new()?,
            sender,
            receiver,
            current_text: Arc::new(Mutex::new("".to_string()))
        })
    }
    async fn _run(&self) -> Result<JoinHandle<Result<(), Box<dyn Error + Send + Sync>>>, Box<dyn Error + Send + Sync>> {
        let sender = self.sender.clone();
        let mut fetch_cb = arboard::Clipboard::new()?;
        let current_text = self.current_text.clone();

        let capture_handle: JoinHandle<Result<(), Box<dyn Error + Send + Sync>>> = tokio::spawn(async move {
            let mut ctlk = current_text.lock().await;
            *ctlk = fetch_cb.get_text()?;
            std::mem::drop(ctlk);
            
            loop {
                let mut fetched;
                match fetch_cb.get_text() {
                    Ok(text) => {
                        fetched = text;
                    },
                    Err(e) => {
                        dbg!(e);
                        fetched = "test".to_string();
                        let mut retry_flag = false;
                        for _ in 0..100 {
                            if let Ok(text) =  fetch_cb.get_text() {
                                fetched = text;  
                                retry_flag = true;
                                break;
                            }      
                        }
                        if !retry_flag {
                            return Err("text fetch from clipboard failed".into());
                        }
                    }
                }
                let mut ctlk = current_text.lock().await;
                if *ctlk != fetched {
                    sender.send(fetched.clone()).await?;
                    *ctlk = fetched;
                }
                std::mem::drop(ctlk);
                sleep(Duration::from_millis(100)).await;
            } 
        });
        Ok(capture_handle)
    }
    pub async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let capture_handle = self._run().await?;
        loop {
            match self.receiver.recv().await {
                Some(com) => {
                    match com {
                        ClipboardCommand::Exit => {
                            capture_handle.abort();
                            return Ok(());
                        },
                        ClipboardCommand::Set(s) => {
                            let mut ctlk = self.current_text.lock().await;
                            *ctlk = s.clone();
                            self.cb.set_text(s).unwrap();
                            std::mem::drop(ctlk);
                        }
                    }
                },
                None => {

                }
            }
        }
    }
}
