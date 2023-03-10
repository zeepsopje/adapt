use super::FieldType;

#[derive(Debug)]
pub struct List {
    pub fields: Vec<FieldType>
}

impl List {
    pub fn new() -> List {
        List {
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: FieldType) {
        self.fields.push(field);
    }
}
