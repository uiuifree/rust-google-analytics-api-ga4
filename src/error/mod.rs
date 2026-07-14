use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub enum GoogleApiError {
    /// ネットワークレベルの失敗（接続不可、タイムアウトなど）
    Connection(String),
    /// レスポンスボディのJSONパース失敗
    JsonParse(String),
    /// APIが非2xxステータスを返した場合（ステータスコードとレスポンスボディ）
    Response(u16, String),
}

impl Display for GoogleApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GoogleApiError::Connection(e) => write!(f, "connection error: {}", e),
            GoogleApiError::JsonParse(e) => write!(f, "json parse error: {}", e),
            GoogleApiError::Response(status, body) => {
                write!(f, "api error (status {}): {}", status, body)
            }
        }
    }
}

impl std::error::Error for GoogleApiError {}
