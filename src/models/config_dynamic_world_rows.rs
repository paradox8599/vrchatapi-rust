/*
 * VRChat API Documentation
 *
 * ![VRChat API Banner](https://raw.githubusercontent.com/vrchatapi/vrchatapi.github.io/master/assets/apibanner.png)  # VRChat API Documentation This project is an [OPEN Open Source Project](https://openopensource.org)  Individuals making significant and valuable contributions are given commit-access to the project to contribute as they see fit. This project is more like an open wiki than a standard guarded open source project.  ## Disclaimer  This is the official response of the VRChat Team (from Tupper more specifically) on the usage of the VRChat API.  > **Use of the API using applications other than the approved methods (website, VRChat application) are not officially supported. You may use the API for your own application, but keep these guidelines in mind:** > * We do not provide documentation or support for the API. > * Do not make queries to the API more than once per 60 seconds. > * Abuse of the API may result in account termination. > * Access to API endpoints may break at any given time, with no warning.  As stated, this documentation was not created with the help of the official VRChat team. Therefore this documentation is not an official documentation of the VRChat API and may not be always up to date with the latest versions. If you find that a page or endpoint is not longer valid please create an issue and tell us so we can fix it.  ## Get in touch with us!  [https://discord.gg/qjZE9C9fkB#vrchat-api](https://discord.gg/qjZE9C9fkB)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigDynamicWorldRows {
    #[serde(rename = "index")]
    pub index: f32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "sortHeading")]
    pub sort_heading: String,
    #[serde(rename = "sortOrder")]
    pub sort_order: String,
    #[serde(rename = "sortOwnership")]
    pub sort_ownership: String,
    /// Tag to filter worlds for this row
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl ConfigDynamicWorldRows {
    pub fn new(index: f32, name: String, platform: String, sort_heading: String, sort_order: String, sort_ownership: String) -> ConfigDynamicWorldRows {
        ConfigDynamicWorldRows {
            index,
            name,
            platform,
            sort_heading,
            sort_order,
            sort_ownership,
            tag: None,
        }
    }
}


