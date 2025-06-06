/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ServiceQueueStats : Statistics about the user's currently queued service request
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServiceQueueStats {
    #[serde(rename = "estimatedServiceDurationSeconds")]
    pub estimated_service_duration_seconds: i32,
}

impl ServiceQueueStats {
    /// Statistics about the user's currently queued service request
    pub fn new(estimated_service_duration_seconds: i32) -> ServiceQueueStats {
        ServiceQueueStats {
            estimated_service_duration_seconds,
        }
    }
}
