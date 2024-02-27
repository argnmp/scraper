use std::error::Error;

use tokio::{io::{stdin, AsyncReadExt, Stdin}, sync::mpsc};

pub struct Io {
    stdin: Stdin,
    sender: mpsc::Sender<String> 
}
impl Io {
    pub fn new(sender: mpsc::Sender<String>) -> Self {
        Io {
            stdin: stdin(),
            sender 
        }
    } 
    
    pub async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        loop {
            let mut str = String::new();
            let _ = self.stdin.read_to_string(&mut str).await.map_err(|_|{"standard io error"})?;
            self.sender.send(str).await?;           
        }
    }
}

