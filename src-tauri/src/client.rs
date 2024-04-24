use std::collections::HashMap;

use irelia::{rest::LcuClient, RequestClient};

use crate::types::{Challenge, PlayerSummary, StandardError, Summoner};

#[tauri::command]
pub async fn get_local_player_challenges() -> Result<Vec<Challenge>, String> {
    pub async fn fetch() -> Result<Option<HashMap<String, Challenge>>, StandardError> {
        let client = RequestClient::new();
        let lcu_client = LcuClient::new(false)?;

        let res: Option<HashMap<String, Challenge>> = lcu_client
            .get("/lol-challenges/v1/challenges/local-player", &client)
            .await?;

        Ok(res)
    }

    let res = fetch().await?;

    let data = res.unwrap_or_default().values().cloned().collect();

    Ok(data)
}

#[tauri::command]
pub async fn get_local_player_categories() -> Result<HashMap<String, Challenge>, String> {
    pub async fn fetch() -> Result<Option<HashMap<String, Challenge>>, StandardError> {
        let client = RequestClient::new();
        let lcu_client = LcuClient::new(false)?;

        let res: Option<HashMap<String, Challenge>> = lcu_client
            .get("/lol-challenges/v1/challenges/category-data", &client)
            .await?;

        Ok(res)
    }

    let res = fetch().await?;

    let data = res.unwrap();

    Ok(data)
}

#[tauri::command]
pub async fn get_current_summoner() -> Result<Summoner, String> {
    pub async fn fetch() -> Result<Option<Summoner>, StandardError> {
        let client = RequestClient::new();
        let lcu_client = LcuClient::new(false)?;

        let res: Option<Summoner> = lcu_client
            .get("/lol-summoner/v1/current-summoner", &client)
            .await?;

        Ok(res)
    }

    let res = fetch().await?;

    let data = res.unwrap();

    Ok(data)
}

#[tauri::command]
pub async fn get_local_player_summary() -> Result<PlayerSummary, String> {
    pub async fn fetch() -> Result<Option<PlayerSummary>, StandardError> {
        let client = RequestClient::new();
        let lcu_client = LcuClient::new(false)?;

        let res: Option<PlayerSummary> = lcu_client
            .get(
                "/lol-challenges/v1/summary-player-data/local-player",
                &client,
            )
            .await?;

        Ok(res)
    }

    let res = fetch().await?;

    let data = res.unwrap();

    Ok(data)
}
