mod text;
mod boolean;

pub use text::Text;
pub use boolean::Boolean;

pub trait Field {}

pub enum FieldType {
    Text(Text),
    Boolean(Boolean)
}

impl Field for FieldType {}
