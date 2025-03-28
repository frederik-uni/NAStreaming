use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::Sub,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use scan::ScanService;
use tokio::task::AbortHandle;

use crate::error::{ApiError, ApiResult};

pub mod auth;
mod ffprobe;
mod hash;
pub mod metadata;
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
    fn start_with_ctx_internal(
        &self,
        id: String,
        ctx: Vec<Value>,
        callback: Box<dyn FnOnce(&str, &Vec<Value>) + Send>,
    );
    fn stop(&self);
    fn channel(&self) -> Channel;
    fn running(&self) -> bool;
}
pub struct Services {
    sel: Option<Arc<Mutex<Services>>>,
    services: HashMap<String, Box<dyn Service + Send>>,
    queue: HashMap<Channel, VecDeque<(String, Vec<Value>)>>,
    blocked_channels: HashSet<Channel>,
    updated: HashMap<String, SystemTime>,
    update_duration: HashMap<String, Duration>,
    next_reload: Option<AbortHandle>,
}

fn log_updated(sel: &mut Services, id: &str, ctx: &Vec<Value>) {
    match id {
        "scan" => {
            if ctx.is_empty() {
                sel.update_process(id)
            } else if ctx.len() == 1 && ctx[0].as_bool() == Some(true) {
                sel.update_process(&format!("scan-detect"))
            }
        }
        _ => {}
    };
}

impl Services {
    fn find_next_reload(&self) -> Option<Duration> {
        if !self.queue.is_empty() {
            return None;
        }
        let mut durations = vec![];
        for (key, value) in &self.update_duration {
            if let Some(v) = self.updated.get(key) {
                let elapsed = SystemTime::now().duration_since(v.clone()).unwrap();
                if value < &elapsed {
                    return Some(Duration::new(0, 0));
                } else {
                    durations.push(value.sub(elapsed));
                }
            } else {
                return Some(Duration::new(0, 0));
            }
        }
        durations.into_iter().min_by(|a, b| a.cmp(b))
    }

    fn process_auto_runners(&mut self) {
        for (key, value) in &self.update_duration {
            if let Some(v) = self.updated.get(key) {
                let elapsed = SystemTime::now().duration_since(v.clone()).unwrap();
                if value < &elapsed {
                    //todo: add if doesnt exist
                }
            } else {
                //todo: add if doesnt exist
            }
        }
        self.start_next_reload();
    }

    fn update_process(&mut self, id: &str) {
        self.updated.insert(id.to_owned(), SystemTime::now());
        self.start_next_reload();
    }

    fn start_next_reload(&mut self) {
        let next = self.find_next_reload();
        if let Some(next) = next {
            log::debug!("Next task will be run in {:?}", next);
            if let Some(v) = &self.next_reload {
                v.abort();
            }
            if next == Duration::new(0, 0) {
                self.process_auto_runners();
                self.next_reload = None;
            } else {
                let sel = self.sel.clone().unwrap();
                self.next_reload = Some(
                    tokio::spawn(async move {
                        tokio::time::sleep(next).await;
                        sel.lock().unwrap().process_auto_runners();
                    })
                    .abort_handle(),
                );
            }
        }
    }

    pub fn services(&self) -> Vec<String> {
        let mut items: Vec<String> = self.services.keys().cloned().collect();
        items.push("scan-detect".to_owned());
        items
    }

    pub fn get_states(&self) -> HashMap<String, bool> {
        self.services
            .iter()
            .map(|(id, service)| (id.to_owned(), service.running()))
            .collect()
    }

    fn update(&mut self) {
        let used = self.blocked_channels.clone();
        for channel in Channel::non_blocking() {
            for (id, ctx) in self.queue.entry(channel).or_default().drain(..) {
                let sel = self.sel.clone().expect("Not correctly initialized");
                self.services
                    .get(&id)
                    .expect("Service checked before")
                    .start_with_ctx_internal(
                        id,
                        ctx,
                        Box::new(move |id, ctx| {
                            let mut lock = sel.lock().unwrap();
                            log_updated(&mut *lock, id, ctx);
                        }),
                    );
            }
        }
        for channel in Channel::blocking()
            .into_iter()
            .filter(|v| !used.contains(v))
        {
            if let Some((id, ctx)) = self.queue.entry(channel).or_default().pop_front() {
                self.blocked_channels.insert(channel);
                let sel = self.sel.clone().expect("Not correctly initialized");
                self.services
                    .get(&id)
                    .expect("Service checked before")
                    .start_with_ctx_internal(
                        id,
                        ctx,
                        Box::new(move |id, ctx| {
                            let mut lock = sel.lock().unwrap();
                            lock.blocked_channels.remove(&channel);
                            log_updated(&mut *lock, id, ctx);
                            lock.update();
                        }),
                    );
            }
        }
    }

    pub fn start_with_ctx(&mut self, id: &str, mut ctx: Vec<Value>) -> ApiResult<()> {
        let id = if id == "scan-detect" {
            ctx.insert(0, Value::Bool(true));
            "scan"
        } else {
            id
        };
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
            updated: HashMap::new(),
            update_duration: HashMap::new(),
            next_reload: None,
        }));
        let self_copy = sel.clone();
        sel.lock().unwrap().sel = Some(self_copy);
        sel.lock().unwrap().start_next_reload();

        sel
    }
}
