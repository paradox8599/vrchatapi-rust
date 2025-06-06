/*
 * VRChat API Documentation
 *
 *
 * Contact: vrchatapi.lpv0t@aries.fyi
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UserSubscription :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserSubscription {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    /// Which \"Store\" it came from. Right now only Stores are \"Steam\" and \"Admin\".
    #[serde(rename = "store")]
    pub store: String,
    #[serde(rename = "steamItemId", skip_serializing_if = "Option::is_none")]
    pub steam_item_id: Option<String>,
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "period")]
    pub period: models::SubscriptionPeriod,
    #[serde(rename = "tier")]
    pub tier: i32,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "status")]
    pub status: models::TransactionStatus,
    #[serde(rename = "starts", skip_serializing_if = "Option::is_none")]
    pub starts: Option<String>,
    #[serde(rename = "expires")]
    pub expires: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "licenseGroups")]
    pub license_groups: Vec<String>,
    #[serde(rename = "isGift")]
    pub is_gift: bool,
    #[serde(rename = "isBulkGift")]
    pub is_bulk_gift: bool,
}

impl UserSubscription {
    pub fn new(
        id: String,
        transaction_id: String,
        store: String,
        amount: f64,
        description: String,
        period: models::SubscriptionPeriod,
        tier: i32,
        active: bool,
        status: models::TransactionStatus,
        expires: String,
        created_at: String,
        updated_at: String,
        license_groups: Vec<String>,
        is_gift: bool,
        is_bulk_gift: bool,
    ) -> UserSubscription {
        UserSubscription {
            id,
            transaction_id,
            store,
            steam_item_id: None,
            amount,
            description,
            period,
            tier,
            active,
            status,
            starts: None,
            expires,
            created_at,
            updated_at,
            license_groups,
            is_gift,
            is_bulk_gift,
        }
    }
}
