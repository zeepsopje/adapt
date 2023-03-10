use crate::field::FieldType;

#[derive(Debug)]
pub struct App {
    pub name: String,
    pub fields: Vec<FieldType>,
}

impl App {
    pub fn new(name: &str) -> App {
        App {
            name: name.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: FieldType) {
        self.fields.push(field);
    }
}
