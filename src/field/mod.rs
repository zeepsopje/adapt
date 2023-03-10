mod text;
mod boolean;
mod list;

pub use text::Text;
pub use boolean::Boolean;
pub use list::List;

#[derive(Debug)]
pub enum FieldType {
    Text(Text),
    Boolean(Boolean),
    List(List),
}
