#[derive(Debug)]
pub struct Text {
    pub value: String,
}

impl Text {
    pub fn new() -> Text {
        Text {
            value: String::new(),
        }
    }
}
