
//use self::rustc_serialize::json::{self, ToJson, Json, DecoderError};

#[derive(Debug, Hash, Serialize, Deserialize)]
pub struct Customer {
    pub name: String,
    pub last_name: String,
}