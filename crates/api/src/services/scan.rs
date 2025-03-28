use std::sync::{Arc, Mutex};

use models::{files::File, scan_groups::ScanGroup, DbUtils};
use storage_finder::PathTree;
use tokio::task::AbortHandle;

use crate::error::{ApiError, ReportError};

use super::{Service, Value};

pub struct ScanService {
    handle: Mutex<Option<AbortHandle>>,
}

impl ScanService {
    pub fn new() -> Self {
        Self {
            handle: Default::default(),
        }
    }
}
fn extract_ctx(ctx: &Vec<Value>) -> Option<(Option<bool>, Option<String>)> {
    if ctx.len() == 2 {
        Some((Some(ctx[0].as_bool()?), Some(ctx[1].as_str()?.to_string())))
    } else if ctx.len() == 1 {
        if let Some(bool) = ctx[0].as_bool() {
            Some((Some(bool), None))
        } else {
            Some((None, Some(ctx[0].as_str()?.to_string())))
        }
    } else if ctx.is_empty() {
        Some((None, None))
    } else {
        None
    }
}

impl Service for ScanService {
    fn start_with_ctx_internal(
        &self,
        id: String,
        ctx: Vec<Value>,
        callback: Box<dyn FnOnce(&str, &Vec<Value>) + Send>,
    ) {
        self.handle.lock().unwrap().replace(
            tokio::task::spawn(async move {
                if let Some((scan_detect, id)) = extract_ctx(&ctx) {
                    let groups = match id {
                        Some(id) => vec![ScanGroup::get(&id)
                            .await
                            .map_err(ApiError::from)?
                            .ok_or(ApiError::NotFoundInDb)?],
                        None => ScanGroup::all().await.map_err(ApiError::from)?,
                    };
                    let mut detect = vec![];
                    let mut no_detect = vec![];
                    //TODO: dont load every path
                    let mut tree = PathTree::default();
                    for item in File::paths().await.map_err(ApiError::from)? {
                        tree.insert(&item);
                    }
                    let tree = Arc::new(tree);

                    for group in groups {
                        if scan_detect.is_none() || scan_detect == Some(true) {
                            let tree = tree.clone();
                            detect.push(async move {
                                if let Some(path) = group.data.detect_path {
                                    // TODO: rerun detect on file system changes
                                    let detected =
                                        storage_finder::parse_library(&path, &tree).await;
                                    File::add_entries(detected).await.map_err(ApiError::from)?;
                                }
                                Ok::<(), ReportError>(())
                            });
                        }
                        if scan_detect.is_none() || scan_detect == Some(false) {
                            let tree = tree.clone();

                            no_detect.push(async move {
                                let detected =
                                    storage_finder::parse_library(&group.data.path, &tree).await;
                                File::add_entries(detected).await.map_err(ApiError::from)?;
                                Ok::<(), ReportError>(())
                            });
                        }
                    }
                    for future in detect {
                        future.await?;
                    }
                    for future in no_detect {
                        future.await?;
                    }
                }
                callback(&id, &ctx);
                Ok::<_, ReportError>(())
            })
            .abort_handle(),
        );
    }

    fn stop(&self) {
        if let Some(v) = self.handle.lock().unwrap().take() {
            v.abort();
        }
    }

    fn channel(&self) -> super::Channel {
        super::Channel::Io
    }

    fn running(&self) -> bool {
        self.handle
            .lock()
            .unwrap()
            .as_ref()
            .map(|v| !v.is_finished())
            .unwrap_or_default()
    }
}
