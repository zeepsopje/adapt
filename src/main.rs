mod app;
mod field;

use app::App;
use field::{Text, FieldType, List};

fn main() {
    let mut app = App::new("Todo app");
    let field = FieldType::Text(Text {
        value: String::from("test")
    });

    app.add_field(field);

    if let FieldType::Text(text) = &mut app.fields[0] {
        println!("{}", text.value);
    }

    let list = FieldType::List(List {
        sub_fields: vec![
            FieldType::List(List::new())
        ]
    });

    app.add_field(list);

    dbg!(app);
}
