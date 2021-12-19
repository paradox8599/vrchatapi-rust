/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WorldMetadata {
    /// WorldID be \"offline\" on User profiles if you are not friends with that user.
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "metadata")]
    pub metadata: serde_json::Value,
}

impl WorldMetadata {
    pub fn new(id: String, metadata: serde_json::Value) -> WorldMetadata {
        WorldMetadata {
            id,
            metadata,
        }
    }
}

