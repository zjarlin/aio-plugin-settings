use aio_plugin_identity_model::SessionView;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response<T> {
    data: T,
}

pub(super) async fn load() -> Result<(SessionView, Vec<String>), String> {
    let session = get::<Option<SessionView>>("/api/auth/session")
        .await?
        .ok_or_else(|| "会话已失效".to_owned())?;
    let sources = if session.permissions.iter().any(|p| p == "plugin:manage") {
        get("/api/runtime/registries").await?
    } else {
        Vec::new()
    };
    Ok((session, sources))
}

pub(super) async fn save(source: String, remove: bool) -> Result<(), String> {
    let builder = if remove {
        Request::delete("/api/runtime/registries")
    } else {
        Request::post("/api/runtime/registries")
    };
    let response = builder
        .json(&serde_json::json!({ "source": source.trim() }))
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if response.ok() {
        Ok(())
    } else {
        Err(error(response).await)
    }
}

async fn get<T: for<'de> Deserialize<'de>>(path: &str) -> Result<T, String> {
    let response = Request::get(path).send().await.map_err(|e| e.to_string())?;
    if !response.ok() {
        return Err(error(response).await);
    }
    response
        .json::<Response<T>>()
        .await
        .map(|r| r.data)
        .map_err(|e| e.to_string())
}

async fn error(response: gloo_net::http::Response) -> String {
    let body = response.text().await.unwrap_or_default();
    serde_json::from_str::<serde_json::Value>(&body)
        .ok()
        .and_then(|v| v.get("error")?.as_str().map(str::to_owned))
        .unwrap_or(body)
}
