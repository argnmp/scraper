use std::{collections::HashMap, error::Error};

#[derive(Debug)]
pub struct Template {
    id: usize,
    acc: i32,
    buf: String,
    reference: Vec<TemplatePtr>,
    resolved: HashMap<String, usize>,
}
#[derive(Debug)]
pub struct TemplatePtr{
    id: usize,
    name: String,
    pos: (usize, usize),
    buf: Option<String>, 
}

impl Template {
    pub fn new(raw: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut reference = Vec::new();
        
        let re = regex::Regex::new(r"\[\[([^\[\]]*)\]\]").unwrap();
         
        let mut id = 0;
        for captures in re.captures_iter(&raw) {
            let pos = captures.get(0).unwrap();
            let name = captures.get(1).unwrap();
            reference.push(TemplatePtr{ id, name: name.as_str().into(), pos: (pos.start(), pos.end()), buf: None});
            id += 1;
        }
        
        Ok(Self {
            id: 0,
            acc: 0,
            buf: raw,
            reference,
            resolved: HashMap::new(),
        })
    }

    pub fn check_resolved(&mut self) -> Option<&str> {
        match self.reference.get_mut(self.id) {
            Some(ptr) => {
                match self.resolved.get(&ptr.name){
                    Some(id) => {
                        if let Some(buf) = &self.reference[*id].buf.clone() {
                            self.replace(&buf)
                        }
                        else {
                            None
                        }
                        
                    },
                    None => {
                        None
                    }    
                }
            },
            None => {
                None
            }
        }
        
    }

    pub fn replace(&mut self, target: &str) -> Option<&str> {
        match self.reference.get_mut(self.id) {
            Some(ptr) => {
                (*ptr).buf = Some(target.to_string());
                if self.acc.is_negative() {
                    (*ptr).pos.0 = ptr.pos.0.checked_sub(self.acc.wrapping_abs() as usize).unwrap();
                    (*ptr).pos.1 = ptr.pos.1.checked_sub(self.acc.wrapping_abs() as usize).unwrap();
                }
                else {
                    (*ptr).pos.0 = ptr.pos.0.checked_add(self.acc as usize).unwrap();
                    (*ptr).pos.1 = ptr.pos.1.checked_add(self.acc as usize).unwrap();
                }

                let mut update = String::new();
                if ptr.pos.0 > 0 {
                    update.push_str(&self.buf[0..ptr.pos.0]);
                }
                update.push_str(target);
                if ptr.pos.1 <= self.buf.len() {
                    update.push_str(&self.buf[ptr.pos.1..self.buf.len()]);
                }

                self.resolved.insert(ptr.name.clone(), ptr.id);
                
                self.id += 1;
                self.buf = update; 
                self.acc += target.len() as i32 - (ptr.pos.1 as i32 - ptr.pos.0 as i32);
                (*ptr).pos.1 = (*ptr).pos.0 + target.len();
                Some(&self.buf)
            },
            None => {
                None
            }
        }
    }
    
    pub fn get_current_target(&mut self) -> Option<&str> {
        match self.reference.get(self.id) {
            Some(ptr) => {
                Some(&ptr.name)
            },
            None => {
                None
            }
        } 
    }
    
    pub fn get_buf(&self) -> &str {
        &self.buf
    }

    pub fn reset(&mut self) {
        self.id = 0;
        self.acc = 0;
        self.resolved.clear();
    }
}

