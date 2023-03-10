use crate::field::{Field, FieldType};

#[derive(Debug)]
pub struct App<F: Field> {
    pub name: String,
    pub fields: Vec<F>
}

impl<F: Field> App<F> {
    pub fn new(name: &str) -> App<F> {
        App {
            name: name.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: F) {
        self.fields.push(field);
    }
}
