use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Store {
    path: PathBuf,
    data: Arc<Mutex<HashMap<String, String>>>,
}

impl Store {
    pub fn new(path: PathBuf) -> Self {
        let data = if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_else(|_| "{}".to_string());
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Self {
            path,
            data: Arc::new(Mutex::new(data)),
        }
    }

    pub fn set(&self, key: String, value: String) -> Result<()> {
        let mut data = self.data.lock().unwrap();
        data.insert(key, value);
        self.save(&data)
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    pub fn delete(&self, key: &str) -> Result<bool> {
        let mut data = self.data.lock().unwrap();
        let removed = data.remove(key).is_some();
        if removed {
            self.save(&data)?;
        }
        Ok(removed)
    }

    pub fn list(&self) -> HashMap<String, String> {
        let data = self.data.lock().unwrap();
        data.clone()
    }

    fn save(&self, data: &HashMap<String, String>) -> Result<()> {
        let content = serde_json::to_string_pretty(data)?;
        fs::write(&self.path, content)?;
        Ok(())
    }
}
