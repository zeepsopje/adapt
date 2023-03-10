use super::FieldType;

#[derive(Debug)]
pub struct List {
    pub sub_fields: Vec<FieldType>
}

impl List {
    pub fn new() -> List {
        List {
            sub_fields: Vec::new(),
        }
    }

    pub fn add_sub_field(&mut self, field: FieldType) {
        self.sub_fields.push(field);
    }
}
