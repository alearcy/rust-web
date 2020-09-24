use std::collections::HashMap;

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>, // key/value delle queries nel path
}

#[derive(Debug)]
pub enum Value<'buf> {
    Sinlge(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split("&") {
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = s.find("=") {
                key = &sub_str[..i];
                val = &sub_str[i..];
            }
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Sinlge(prev_value) => {
                        *existing = Value::Multiple(vec![prev_value, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Sinlge(val));
        }
        QueryString { data }
    }
}