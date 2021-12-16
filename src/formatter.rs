use crate::parser::Object;

pub struct JsonFmt {}

impl JsonFmt {
    pub fn new() -> Self {
        JsonFmt {}
    }

    pub fn format(&self, object: &Object) -> String {
        match object {
            Object::Num(f) => f.to_string(),
            Object::String(s) => {
                format!("\"{}\"", s)
            }
            Object::Struct(fields) => {
                let mut s = "{".to_string();
                let mut iter = fields.iter().peekable();
                while let Some(field) = iter.next() {
                    s.push('"');
                    s.push_str(&field.0 .0); // TODO this is not legit
                    s.push('"');
                    s.push(':');
                    s.push_str(&self.format(&field.1));
                    if iter.peek().is_some() {
                        s.push(',');
                    }
                }
                s.push('}');
                s
            }
            Object::Array(a) => {
                let mut s = "[".to_string();
                for o in a {
                    s.push_str(&self.format(o));
                    s.push(',');
                }
                s.push(']');
                s
            }
            Object::None => "null".to_string(),
        }
    }
}
