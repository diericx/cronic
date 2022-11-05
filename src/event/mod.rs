pub use self::repo::Repo;
use rocket::serde::Deserialize;
use serde::ser::{Serialize, SerializeStruct, Serializer};

mod repo;

#[derive(Eq, PartialEq, Debug, Deserialize)]
pub struct Event {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub source: String,
    pub code: i32,
    pub output: String,
    #[serde(skip_deserializing)]
    pub date: String,
}

impl Serialize for Event {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Event", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("source", &self.source)?;
        state.serialize_field("code", &self.code)?;
        state.serialize_field("output", &self.output)?;
        state.serialize_field("date", &self.date)?;
        state.end()
    }
}
