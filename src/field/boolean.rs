use super::Field;

pub struct Boolean(bool);

impl Boolean {
    pub fn new(default: bool) -> Boolean {
        Boolean(default)
    }
}

impl Field for Boolean {}
