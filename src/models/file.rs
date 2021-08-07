/*
 * VRChat API Documentation
 *
 * ![VRChat API Banner](https://raw.githubusercontent.com/vrchatapi/vrchatapi.github.io/master/assets/apibanner.png)  # VRChat API Documentation This project is an [OPEN Open Source Project](https://openopensource.org)  Individuals making significant and valuable contributions are given commit-access to the project to contribute as they see fit. This project is more like an open wiki than a standard guarded open source project.  ## Disclaimer  This is the official response of the VRChat Team (from Tupper more specifically) on the usage of the VRChat API.  > **Use of the API using applications other than the approved methods (website, VRChat application) are not officially supported. You may use the API for your own application, but keep these guidelines in mind:** > * We do not provide documentation or support for the API. > * Do not make queries to the API more than once per 60 seconds. > * Abuse of the API may result in account termination. > * Access to API endpoints may break at any given time, with no warning.  As stated, this documentation was not created with the help of the official VRChat team. Therefore this documentation is not an official documentation of the VRChat API and may not be always up to date with the latest versions. If you find that a page or endpoint is not longer valid please create an issue and tell us so we can fix it.  ## Get in touch with us!  [https://discord.gg/qjZE9C9fkB#vrchat-api](https://discord.gg/qjZE9C9fkB)
 *
 * The version of the OpenAPI document: 1.0.1
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */

/// File : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    #[serde(rename = "id")]
    pub id: String,
    /// 
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "mimeType")]
    pub mime_type: crate::models::MimeType,
    #[serde(rename = "extension")]
    pub extension: String,
    #[serde(rename = "tags")]
    pub tags: Vec<String>,
    #[serde(rename = "versions")]
    pub versions: Vec<crate::models::FileVersion>,
}

impl File {
    /// 
    pub fn new(id: String, name: String, owner_id: String, mime_type: crate::models::MimeType, extension: String, tags: Vec<String>, versions: Vec<crate::models::FileVersion>) -> File {
        File {
            id,
            name,
            owner_id,
            mime_type,
            extension,
            tags,
            versions,
        }
    }
}


