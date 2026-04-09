use reqwest::Client;
use serde_json::{json, Value};
use crate::db;

async fn execute(client: &Client, url: &str, token: &str, stmt: Value) -> Result<Value, String> {
    let body = json!({ "requests": [{ "type": "execute", "stmt": stmt }, { "type": "close" }] });
    let res = client
        .post(format!("{}/v2/pipeline", url))
        .bearer_auth(token)
        .json(&body)
        .send().await.map_err(|e| e.to_string())?
        .json::<Value>().await.map_err(|e| e.to_string())?;

    if let Some(err) = res["results"][0]["error"].as_str() {
        return Err(err.to_string());
    }
    Ok(res["results"][0]["response"]["result"].clone())
}

async fn exec_sql(client: &Client, url: &str, token: &str, sql: &str, args: Vec<Value>) -> Result<Value, String> {
    let stmt = json!({
        "sql": sql,
        "args": args.iter().map(|v| json!({ "type": "text", "value": v })).collect::<Vec<_>>()
    });
    execute(client, url, token, stmt).await
}

pub async fn push(url: &str, token: &str) -> Result<(), String> {
    let url = url.replace("libsql://", "https://");
    let url = url.as_str();
    let client = Client::new();

    // Ensure schema
    for sql in [
        "CREATE TABLE IF NOT EXISTS debates (id TEXT PRIMARY KEY, motion TEXT NOT NULL, infoslide TEXT NOT NULL DEFAULT '')",
        "CREATE TABLE IF NOT EXISTS speeches (debate_id TEXT NOT NULL, role TEXT NOT NULL, \
         speaker TEXT NOT NULL DEFAULT '', speech TEXT NOT NULL DEFAULT '', \
         rebuttal TEXT NOT NULL DEFAULT '', poi TEXT NOT NULL DEFAULT '', \
         PRIMARY KEY (debate_id, role))",
    ] {
        exec_sql(&client, url, token, sql, vec![]).await?;
    }
    // Migrate remote schema if infoslide column is missing
    exec_sql(&client, url, token,
        "ALTER TABLE debates ADD COLUMN infoslide TEXT NOT NULL DEFAULT ''", vec![]).await.ok();

    for (id, motion) in db::get_debates() {
        let infoslide = db::get_debate(&id).map(|d| d.infoslide.clone()).unwrap_or_default();
        exec_sql(&client, url, token,
            "INSERT OR REPLACE INTO debates (id, motion, infoslide) VALUES (?, ?, ?)",
            vec![json!(id), json!(motion), json!(infoslide)]).await?;

        if let Some(debate) = db::get_debate(&id) {
            for role in ["PM","LO","DPM","DLO","MG","MO","GW","OW"] {
                let s = debate.get_speech(role);
                exec_sql(&client, url, token,
                    "INSERT OR REPLACE INTO speeches (debate_id, role, speaker, speech, rebuttal, poi) \
                     VALUES (?, ?, ?, ?, ?, ?)",
                    vec![json!(id), json!(role), json!(s.speaker), json!(s.speech),
                         json!(s.rebuttal), json!(s.poi)]).await?;
            }
        }
    }
    Ok(())
}

pub async fn pull(url: &str, token: &str) -> Result<(), String> {
    let url = url.replace("libsql://", "https://");
    let url = url.as_str();
    let client = Client::new();

    let debates_res = exec_sql(&client, url, token, "SELECT id, motion, infoslide FROM debates", vec![]).await?;
    let rows = debates_res["rows"].as_array().cloned().unwrap_or_default();

    for row in rows {
        let id        = row[0]["value"].as_str().unwrap_or("").to_string();
        let motion    = row[1]["value"].as_str().unwrap_or("").to_string();
        let infoslide = row[2]["value"].as_str().unwrap_or("").to_string();
        db::upsert_debate(&id, &motion, &infoslide);

        let speeches_res = exec_sql(&client, url, token,
            "SELECT role, speaker, speech, rebuttal, poi FROM speeches WHERE debate_id = ?",
            vec![json!(id)]).await?;

        for srow in speeches_res["rows"].as_array().cloned().unwrap_or_default() {
            let role     = srow[0]["value"].as_str().unwrap_or("").to_string();
            let speaker  = srow[1]["value"].as_str().unwrap_or("").to_string();
            let speech   = srow[2]["value"].as_str().unwrap_or("").to_string();
            let rebuttal = srow[3]["value"].as_str().unwrap_or("").to_string();
            let poi      = srow[4]["value"].as_str().unwrap_or("").to_string();
            db::save_speech(&id, &role, &speaker, &speech, &rebuttal, &poi);
        }
    }
    Ok(())
}
