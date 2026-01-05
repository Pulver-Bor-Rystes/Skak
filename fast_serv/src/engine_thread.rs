use crate::parsing::Json;
pub struct EngineApi;

pub struct Bot {
    pub name: String,
    pub rating: usize,
}

impl Json for Bot {
    fn to_json(&self) -> String {
        format!("{{\"name\": {}, \"rating\": {}}}", self.name.to_json(), self.rating.to_json())
    }
}
