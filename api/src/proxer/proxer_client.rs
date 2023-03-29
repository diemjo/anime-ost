
use std::time::{Duration, Instant};

use reqwest::{Method, Request, Url};

use crate::{error::Error, result::Result, proxer::html};

use crate::models::AnimeUserEntry;

const PROXER_URL: &str = "https://proxer.me/login";
const PROXER_ANIME_URL: &str = "https://proxer.me/user/USER_ID/anime";
const REQUEST_PAUSE_DURATION_MS: u128 = 300;
pub(crate) struct ProxerClient {
    client: reqwest::Client,
    user: String,
    password: String,
    last_request: Instant,
}

impl ProxerClient {
    pub fn new(user: &str, password: &str) -> Result<Self> {
        // customize
        /*let mut bypasser = {
            cloudflare_bypasser::Bypasser::default()
                .retry(30)                      // retry times, it might be 10000, depends on your network environment, default 0 (infinity)
                //.random_user_agent(true)        // use random user agent, default false
                .user_agent("Mozilla/5.0")      // specify user agent manually, default ""
                .wait(5)                        // cloudflare's waiting time, but in my test it can be 0, default 0
        };

        let (cookie, user_agent);
            loop {
                if let Ok((c, ua)) =  bypasser.bypass(PROXER_URL) {
                    cookie = c;
                    user_agent = ua;
                    break;
                }
            }*/

        let headers = {
            let mut h = reqwest::header::HeaderMap::new();
            //h.insert(reqwest::header::COOKIE, cookie);
            h.insert(reqwest::header::USER_AGENT, "Mozilla/5.0".parse().unwrap());
            h
        };

        Ok(
            Self {
                client: reqwest::Client::builder()
                    .cookie_store(true)
                    .default_headers(headers)
                    .build()
                    .or_else(|e| Err(Error::ReqwestClientError(e)))?,
                user: user.to_string(),
                password: password.to_string(),
                last_request: Instant::now() - Duration::from_millis(REQUEST_PAUSE_DURATION_MS as u64)
            }
        )
    }

    pub async fn search_anime(&mut self, user: u32) -> Result<Vec<AnimeUserEntry>> {
        let url: Url = Url::parse(&PROXER_ANIME_URL.replace("USER_ID", user.to_string().as_str())).unwrap();
        let request = Request::new(Method::GET, url);

        if self.last_request.elapsed().as_millis() <= REQUEST_PAUSE_DURATION_MS {
            tokio::time::sleep(Duration::from_millis((REQUEST_PAUSE_DURATION_MS - self.last_request.elapsed().as_millis()) as u64)).await;
            self.last_request = Instant::now();
        }
        println!("Request: {} {}", request.method(), request.url());
        match self.try_search_once(user, &request).await {
            Ok(anime_list) => Ok(anime_list),
            Err(Error::ProxerAccessError()) => {
                println!("Not authorized, trying to log in as {}.", self.user);
                tokio::time::sleep(tokio::time::Duration::from_millis(REQUEST_PAUSE_DURATION_MS as u64)).await;
                let login_resp = self.login().await?;
                if login_resp.status().is_success() {
                    println!("Login successful, logged in as {}. Trying again.", self.user);
                    tokio::time::sleep(tokio::time::Duration::from_millis(REQUEST_PAUSE_DURATION_MS as u64)).await;
                    self.try_search_once(user, &request).await
                } else {
                    println!("Login failed as {}", self.user);
                    Err(Error::ReqwestProxerLoginError(self.user.clone(), login_resp.status().to_string()))
                }
            },
            Err(e) => Err(e)
        }
    }

    async fn try_search_once(&self, user: u32, request: &Request) -> Result<Vec<AnimeUserEntry>> {
        let request = request.try_clone().ok_or_else(||
            Error::ReqwestClientRequestError(format!("Cannot copy request {:?}", request.url().as_str()))
        )?;
        let resp = self.client.execute(request).await
            .or_else(|e| Err(Error::ReqwestClientError(e)))?;
        if resp.status().is_success() {
            let body = resp.text().await
                .or_else(|e| Err(Error::ReqwestClientError(e)))?;
            html::parse_anime_list(user, body)
        } else {
            Err(Error::ReqwestClientRequestError(format!("Invalid response status code: {}", resp.status().canonical_reason().unwrap_or(resp.status().as_str()))))
        }
    }

    async fn login(&self) -> Result<reqwest::Response> {
        let resp = self.client
            .post(PROXER_URL)
            .form(&[
                ("username", self.user.as_str()),
                ("password", &self.password.as_str()),
                ("remember", "1"),
                ("submit", "Login"),
                ("secretkey", ""),
                ("redirect", "")
            ])
            .send()
            .await
            .or_else(|e| Err(Error::ReqwestClientError(e)))?;
        Ok(resp)
    }
}