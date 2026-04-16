use crate::json_rpc::{JsonRpcRequest, JsonRpcResponse};
use anyhow::Result;

const DEFAULT_TIMEOUT_SECS: u64 = 30;

#[derive(Clone)]
pub struct HttpTransport {
    client: reqwest::Client,
    url: String,
    access_token: Option<String>,
}

impl HttpTransport {
    pub fn new(url: impl Into<String>, access_token: Option<String>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;

        Ok(Self {
            client,
            url: url.into(),
            access_token,
        })
    }

    /// 返回当前使用的 `access_token（用于缓存判断`）
    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    pub async fn call<T: for<'de> serde::Deserialize<'de> + Default>(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<T> {
        let request = JsonRpcRequest::new(method, params);

        let mut request_builder = self.client.post(&self.url).json(&request);
        if let Some(token) = &self.access_token {
            request_builder = request_builder.bearer_auth(token);
        }

        let response = request_builder.send().await?;

        // 先拿到完整 JSON 做日志（含 result 外层字段）
        let raw_json: serde_json::Value = response.json().await?;
        let rpc_response: JsonRpcResponse<T> = serde_json::from_value(raw_json.clone())?;

        // 统一记录 MCP 调用日志（含 request_id）
        if rpc_response.result.is_some() {
            let result_raw = &raw_json["result"];
            let rid = result_raw
                .get("request_id")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let code = result_raw
                .get("code")
                .and_then(serde_json::Value::as_i64)
                .unwrap_or(0);
            tracing::info!(method = method, request_id = rid, code = code, "MCP call");
        }

        if let Some(error) = rpc_response.error {
            anyhow::bail!("MCP error {}: {}", error.code, error.message);
        }

        rpc_response
            .result
            .ok_or_else(|| anyhow::anyhow!("No result in MCP response"))
    }
}
