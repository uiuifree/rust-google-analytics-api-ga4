use crate::error::GoogleApiError;
use std::sync::OnceLock;

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn client() -> &'static reqwest::Client {
    CLIENT.get_or_init(reqwest::Client::new)
}

pub(crate) struct HttpClient;

impl HttpClient {
    pub async fn get<T>(token: &str, url: &str) -> Result<T, GoogleApiError>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = client()
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;
        Self::parse_response(response).await
    }

    pub async fn post<T, U>(token: &str, url: &str, params: U) -> Result<T, GoogleApiError>
    where
        T: for<'de> serde::Deserialize<'de>,
        U: serde::Serialize,
    {
        let response = client()
            .post(url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&params)
            .send()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;
        Self::parse_response(response).await
    }

    async fn parse_response<T>(response: reqwest::Response) -> Result<T, GoogleApiError>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|e| GoogleApiError::Connection(e.to_string()))?;
        if !status.is_success() {
            return Err(GoogleApiError::Response(status.as_u16(), body));
        }
        serde_json::from_str(&body).map_err(|_| GoogleApiError::JsonParse(body))
    }
}
