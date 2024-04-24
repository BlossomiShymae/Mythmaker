use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: i32,
    pub current_level: String,
    pub name: String,
    pub description: String,
    pub current_value: f32,
    pub next_threshold: f32,
    pub percentile: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub game_name: String,
    pub tag_line: String,
    pub profile_icon_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerSummary {
    pub overall_challenge_level: String,
}

#[derive(Debug, Clone)]
pub struct StandardError {
    pub message: String,
}

impl StandardError {
    pub fn new(message: String) -> StandardError {
        StandardError { message }
    }

    pub fn new_str(message: &str) -> StandardError {
        StandardError {
            message: message.to_owned(),
        }
    }

    pub fn from_error(error: impl Error) -> StandardError {
        StandardError {
            message: error.to_string(),
        }
    }
}

impl From<irelia::Error> for StandardError {
    fn from(value: irelia::Error) -> Self {
        StandardError {
            message: value.to_string(),
        }
    }
}

impl Error for StandardError {}

impl fmt::Display for StandardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something has gone wrong")
    }
}

impl From<StandardError> for String {
    fn from(value: StandardError) -> Self {
        value.message
    }
}
