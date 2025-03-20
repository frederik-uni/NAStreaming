use std::collections::HashMap;

pub enum State {
    Planed,
    Running,
}
pub trait MetadataProvider {
    /// Init instance
    fn new(data: HashMap<String, String>) -> Result<Box<Self>, String>;
    /// Unique id
    fn id() -> &'static str;
    /// Display string
    fn name(&self) -> &'static str;
    /// State of development
    fn state(&self) -> State;
    /// Original site
    fn origin(&self) -> &'static str;
    /// Returns search instance
    fn search(&self) -> Option<Box<dyn SearchProvider>>;
    /// Returns info instance
    fn info(&self) -> Option<Box<dyn InfoProvider>>;
}

pub trait SearchProvider {}
pub trait InfoProvider {}
