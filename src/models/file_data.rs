/*
 * VRChat API Documentation
 *
 * ![VRChat API Banner](https://raw.githubusercontent.com/vrchatapi/vrchatapi.github.io/master/assets/apibanner.png)  # VRChat API Documentation This project is an [OPEN Open Source Project](https://openopensource.org)  Individuals making significant and valuable contributions are given commit-access to the project to contribute as they see fit. This project is more like an open wiki than a standard guarded open source project.  ## Disclaimer  This is the official response of the VRChat Team (from Tupper more specifically) on the usage of the VRChat API.  > **Use of the API using applications other than the approved methods (website, VRChat application) are not officially supported. You may use the API for your own application, but keep these guidelines in mind:** > * We do not provide documentation or support for the API. > * Do not make queries to the API more than once per 60 seconds. > * Abuse of the API may result in account termination. > * Access to API endpoints may break at any given time, with no warning.  As stated, this documentation was not created with the help of the official VRChat team. Therefore this documentation is not an official documentation of the VRChat API and may not be always up to date with the latest versions. If you find that a page or endpoint is not longer valid please create an issue and tell us so we can fix it.  ## Get in touch with us!  [https://discord.gg/qjZE9C9fkB#vrchat-api](https://discord.gg/qjZE9C9fkB)
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: me@ruby.js.org
 * Generated by: https://openapi-generator.tech
 */

/// FileData : 



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileData {
    #[serde(rename = "fileName")]
    pub file_name: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "md5")]
    pub md5: String,
    #[serde(rename = "sizeInBytes")]
    pub size_in_bytes: f32,
    #[serde(rename = "status")]
    pub status: crate::models::FileStatus,
    #[serde(rename = "category")]
    pub category: Category,
    #[serde(rename = "uploadId")]
    pub upload_id: String,
}

impl FileData {
    /// 
    pub fn new(file_name: String, url: String, md5: String, size_in_bytes: f32, status: crate::models::FileStatus, category: Category, upload_id: String) -> FileData {
        FileData {
            file_name,
            url,
            md5,
            size_in_bytes,
            status,
            category,
            upload_id,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "multipart")]
    Multipart,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "simple")]
    Simple,
}

