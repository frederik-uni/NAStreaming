use std::{
    collections::{HashMap, HashSet, VecDeque},
    sync::{Arc, Mutex},
};

use scan::ScanService;

use crate::error::{ApiError, ApiResult};

pub mod auth;
mod ffprobe;
mod hash;
mod rename;
pub mod scan;
mod split_files;
mod update_metadata;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Channel {
    Scraper,
    Io,
}

impl Channel {
    pub fn blocking() -> Vec<Self> {
        vec![Channel::Scraper, Channel::Io]
    }

    pub fn non_blocking() -> Vec<Self> {
        vec![Channel::Scraper]
    }
}

pub enum Value {
    String(String),
    Bool(bool),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

pub trait Service {
    fn start_with_ctx_internal(&self, ctx: Vec<Value>, callback: Box<dyn FnOnce() + Send>);
    fn stop(&self);
    fn channel(&self) -> Channel;
    fn running(&self) -> bool;
}
pub struct Services {
    sel: Option<Arc<Mutex<Services>>>,
    services: HashMap<String, Box<dyn Service + Send>>,
    queue: HashMap<Channel, VecDeque<(String, Vec<Value>)>>,
    blocked_channels: HashSet<Channel>,
}

impl Services {
    pub fn services(&self) -> Vec<String> {
        self.services.keys().cloned().collect()
    }

    pub fn get_states(&self) -> HashMap<String, bool> {
        self.services
            .iter()
            .map(|(id, service)| (id.to_owned(), service.running()))
            .collect()
    }

    fn is_running(&self, id: &str) -> bool {
        self.services
            .get(id)
            .map_or(false, |service| service.running())
    }

    fn update(&mut self) {
        let used = self.blocked_channels.clone();
        for channel in Channel::non_blocking() {
            for (id, ctx) in self.queue.entry(channel).or_default().drain(..) {
                self.services
                    .get(&id)
                    .expect("Service checked before")
                    .start_with_ctx_internal(ctx, Box::new(|| {}));
            }
        }
        for channel in Channel::blocking()
            .into_iter()
            .filter(|v| !used.contains(v))
        {
            if let Some((id, ctx)) = self.queue.entry(channel).or_default().pop_front() {
                self.blocked_channels.insert(channel);
                let sel = self.sel.clone().expect("Not correctly initialized");
                let callback = Box::new(move || {
                    let mut lock = sel.lock().unwrap();
                    lock.blocked_channels.remove(&channel);
                    lock.update();
                });
                self.services
                    .get(&id)
                    .expect("Service checked before")
                    .start_with_ctx_internal(ctx, callback);
            }
        }
    }
    pub fn start_with_ctx(&mut self, id: &str, ctx: Vec<Value>) -> ApiResult<()> {
        let service = self.services.get(id).ok_or(ApiError::NotFoundInDb)?;
        self.queue
            .entry(service.channel())
            .or_default()
            .push_back((id.to_owned(), ctx));
        self.update();
        Ok(())
    }
}

impl Services {
    pub fn new() -> Arc<Mutex<Self>> {
        let mut map: HashMap<String, Box<dyn Service + Send>> = HashMap::new();
        map.insert("scan".to_owned(), Box::new(ScanService::new()));
        let sel = Arc::new(Mutex::new(Services {
            sel: None,
            services: map,
            queue: Default::default(),
            blocked_channels: Default::default(),
        }));
        let self_copy = sel.clone();
        sel.lock().unwrap().sel = Some(self_copy);
        sel
    }
}
