#[derive(Debug)]
pub struct Boolean(bool);

impl Boolean {
    pub fn new(default: bool) -> Boolean {
        Boolean(default)
    }
}
