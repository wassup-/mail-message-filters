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
    pub actions: Vec<Action>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Condition {
    Contains(Contains),
    EndsWith(EndsWith),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndsWith {
    pub field: Field,
    #[serde(rename = "ends_with")]
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contains {
    pub field: Field,
    #[serde(rename = "contains")]
    pub values: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Field {
    #[serde(rename = "from")]
    From,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Action {
    MoveTo(MoveTo),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveTo {
    #[serde(rename = "move_to")]
    pub folder: String,
}

use serde::{Deserialize, Serialize};
