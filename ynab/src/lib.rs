use app_config::YnabConfig;
use reqwest::{Method, RequestBuilder, Response, Url};
use serde::{de::DeserializeOwned, Deserialize};
use types::BudgetSummaryResponse;

pub mod types;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("error decoding resposne body: {error}\nbody: {body}")]
    Decode {
        error: serde_json::Error,
        body: String,
    },
}

#[derive(Deserialize)]
struct ResponseDataEnvelope<T> {
    data: T,
}

pub struct Client {
    reqwest: reqwest::Client,
    conf: YnabConfig,
}

impl Client {
    pub fn new(conf: YnabConfig) -> Self {
        let client = reqwest::Client::new();
        Self {
            reqwest: client,
            conf,
        }
    }

    /// Simply returns a clone of `base_url`, such that you can use `reqwest.Url`'s API for
    /// building the desired URL.
    pub fn build_url(&self) -> Url {
        // self.config.username = "asd".to_string();
        self.conf.base_url.clone()
    }

    /// Simple proxy to `reqwest::Client::request`.
    /// Once you're done building the request, it should be fed into this struct's `.send()`.
    pub fn request(&self, method: Method, url: Url) -> RequestBuilder {
        self.reqwest
            .request(method, url)
            .bearer_auth(self.conf.token.clone())
    }

    pub async fn send(&self, req: RequestBuilder) -> Result<Response, reqwest::Error> {
        let resp = req.send().await?;
        let resp = resp.error_for_status()?;
        Ok(resp)
    }

    pub async fn parse_json_resp<R>(resp: Response) -> Result<R, Error>
    where
        R: DeserializeOwned,
    {
        // Use `.text()` instead of `.json()` so that we can print the body if deserialization fails.
        let text = resp.text().await?;
        let envelope = serde_json::from_str::<ResponseDataEnvelope<R>>(&text).map_err(|error| {
            Error::Decode {
                error: { error },
                body: text,
            }
        })?;
        Ok(envelope.data)
    }
}

/// Budgets
impl Client {
    pub async fn get_budgets(&self) -> Result<BudgetSummaryResponse, Error> {
        let mut url = self.build_url().join("budgets").unwrap();
        url.query_pairs_mut()
            .append_pair("include_accounts", "true")
            .finish();

        // XXX
        println!("url: {}", url.to_string());
        let resp = self.send(self.request(Method::GET, url)).await?;
        Client::parse_json_resp::<BudgetSummaryResponse>(resp).await
    }
}
