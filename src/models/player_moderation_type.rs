/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayerModerationType {
    #[serde(rename = "mute")]
    Mute,
    #[serde(rename = "unmute")]
    Unmute,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "unblock")]
    Unblock,
    #[serde(rename = "hideAvatar")]
    HideAvatar,
    #[serde(rename = "showAvatar")]
    ShowAvatar,

}

impl ToString for PlayerModerationType {
    fn to_string(&self) -> String {
        match self {
            Self::Mute => String::from("mute"),
            Self::Unmute => String::from("unmute"),
            Self::Block => String::from("block"),
            Self::Unblock => String::from("unblock"),
            Self::HideAvatar => String::from("hideAvatar"),
            Self::ShowAvatar => String::from("showAvatar"),
        }
    }
}



