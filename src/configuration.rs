#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct Configuration {
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub evolution_id: Option<String>,
    pub thunderbird_id: Option<String>,
    pub message_filters: Vec<MessageFilter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageFilter {
    pub title: String,
    pub conditions: Vec<Condition>,
    pub move_to: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Condition {
    Contains(Contains),
    EndsWith(EndsWith),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndsWith {
    pub field: String,
    #[serde(rename = "ends_with")]
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contains {
    pub field: String,
    #[serde(rename = "contains")]
    pub values: Vec<String>,
}

use serde::{Deserialize, Serialize};
