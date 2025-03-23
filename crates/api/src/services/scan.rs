use models::scan_groups::ScanGroup;
use models::Record;

pub struct ScanService {}

impl ScanService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn scan(&self, id: Record<ScanGroup>) {
        //TODO:
    }
}
