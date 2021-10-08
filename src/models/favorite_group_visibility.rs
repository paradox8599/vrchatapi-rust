/*
 * VRChat API Documentation
 *
 *
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */


/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FavoriteGroupVisibility {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "friends")]
    Friends,
    #[serde(rename = "public")]
    Public,

}

impl ToString for FavoriteGroupVisibility {
    fn to_string(&self) -> String {
        match self {
            Self::Private => String::from("private"),
            Self::Friends => String::from("friends"),
            Self::Public => String::from("public"),
        }
    }
}



