use crate::parsing::Json;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Username(String);

impl Username {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Username {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Json for Username {
    fn to_json(&self) -> String {
        format!("\"{}\"", self.0)
    }
}

impl From<String> for Username {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Username {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl std::fmt::Display for Username {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username({})", self.0)
    }
}
