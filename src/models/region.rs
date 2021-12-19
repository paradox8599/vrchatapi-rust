/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */

/// Region : API/Photon region.

/// API/Photon region.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Region {
    #[serde(rename = "us")]
    Us,
    #[serde(rename = "use")]
    _Use,
    #[serde(rename = "eu")]
    Eu,
    #[serde(rename = "jp")]
    Jp,
    #[serde(rename = "unknown")]
    Unknown,

}

impl ToString for Region {
    fn to_string(&self) -> String {
        match self {
            Self::Us => String::from("us"),
            Self::_Use => String::from("use"),
            Self::Eu => String::from("eu"),
            Self::Jp => String::from("jp"),
            Self::Unknown => String::from("unknown"),
        }
    }
}

impl Default for Region {
    fn default() -> Region {
        Self::Us
    }
}



