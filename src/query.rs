/// Enum --> K/V pairs
pub trait QueryValues {
    fn values(&self) -> std::collections::HashMap<String, String>;
}

impl QueryValues for std::collections::HashMap<String, String> {
    fn values(&self) -> std::collections::HashMap<String, String> {
        self.clone()
    }
}

impl QueryValues for Vec<(&str, &str)> {
    fn values(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        for (key, value) in self {
            map.insert(key.to_string(), value.to_string());
        }
        map
    }
}
