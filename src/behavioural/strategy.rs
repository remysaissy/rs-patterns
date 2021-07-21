
// Separation of concerns for report generation.

use std::collections::HashMap;

type Data = HashMap<String, u32>;

// Trait to implement the various report format available.
pub trait Formatter {
    fn format(&self, data: &Data, out: &mut String);
}

struct Text;
impl Formatter for Text {
    fn format(&self, data: &Data, out: &mut String) {
        for (k, v) in data.iter() {            
            let entry = format!("{} {}\n", k, v);
            out.push_str(&entry);
        }
    }
}

struct Json;
impl Formatter for Json {
    fn format(&self, data: &Data, out: &mut String) {
        out.push_str("[");
        for (k, v) in data.iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            out.push_str(&entry);            
            out.push_str(",");
        }
        out.pop();
        out.push_str("]");
    }
}

// Strategy pattern.
pub struct Report;
impl Report {
    pub fn generate<F: Formatter>(f: F, data: &Data, out: &mut String) {
        f.format(data, out);
    }
}

#[cfg(test)]
mod tests {
    
    use super::{Report, Text, Json, Data};

    #[test]
    fn test_render_as_text() {
        let mut data = Data::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        let mut out = String::from("");
        Report::generate(Text, &data, &mut out);
        assert!(out.contains("one 1"));
        assert!(out.contains("two 2"));
    }

    #[test]
    fn test_render_as_json() {
        let mut data = Data::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        let mut out = String::from("");
        Report::generate(Json, &data, &mut out);
        assert!(out.contains(r#"{"one":"1"}"#));
        assert!(out.contains(r#"{"two":"2"}"#));
    }
}