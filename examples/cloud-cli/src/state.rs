use crate::domain::{Bucket, Instance};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
#[allow(dead_code)]
pub struct CloudState {
    pub instances: Arc<Mutex<HashMap<String, Instance>>>,
    pub buckets: Arc<Mutex<HashMap<String, Bucket>>>,
    pub current_user: Arc<Mutex<Option<String>>>,
    pub auth_tokens: Arc<Mutex<HashMap<String, String>>>, // Token -> User
}

impl Default for CloudState {
    fn default() -> Self {
        let mut tokens = HashMap::new();
        tokens.insert("valid-token-123".to_string(), "admin".to_string());

        Self {
            instances: Arc::new(Mutex::new(HashMap::new())),
            buckets: Arc::new(Mutex::new(HashMap::new())),
            current_user: Arc::new(Mutex::new(None)),
            auth_tokens: Arc::new(Mutex::new(tokens)),
        }
    }
}

impl CloudState {
    #[allow(dead_code)]
    pub fn add_instance(&self, instance: Instance) {
        self.instances
            .lock()
            .unwrap()
            .insert(instance.id.clone(), instance);
    }

    pub fn list_instances(&self) -> Vec<Instance> {
        self.instances.lock().unwrap().values().cloned().collect()
    }

    pub fn terminate_instance(&self, id: &str) -> Option<Instance> {
        self.instances.lock().unwrap().remove(id)
    }

    #[allow(dead_code)]
    pub fn add_bucket(&self, bucket: Bucket) {
        self.buckets
            .lock()
            .unwrap()
            .insert(bucket.name.clone(), bucket);
    }

    pub fn list_buckets(&self) -> Vec<Bucket> {
        self.buckets.lock().unwrap().values().cloned().collect()
    }

    pub fn validate_token(&self, token: &str) -> Option<String> {
        self.auth_tokens.lock().unwrap().get(token).cloned()
    }

    #[allow(dead_code)]
    pub fn login(&self, token: &str) -> bool {
        if let Some(user) = self.validate_token(token) {
            *self.current_user.lock().unwrap() = Some(user);
            true
        } else {
            false
        }
    }
}
