use serde::{Deserialize, Serialize};
use url::Url;
use wasm_bindgen::prelude::*;

use conference_core::{
    error::Result, network::Client, store::InMemoryStore, IDProvider,
    QAManager as InnerConferenceManager, Question, TimeProvider,
};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[wasm_bindgen]
pub struct ConferenceManager {
    inner: InnerConferenceManager<FetchHttpClient, InMemoryStore, JsTimeProvider, RandomIDProvider>,
}

pub struct FetchHttpClient {
    base_url: Url,
}

impl Client for FetchHttpClient {
    fn get_remote(
        &self,
    ) -> impl std::future::Future<Output = conference_core::error::Result<Vec<conference_core::Question>>>
    {
        async {
            let url = self.base_url.join("/v1/questions").unwrap();
            let mut opts = RequestInit::new();
            opts.method("GET");
            opts.mode(web_sys::RequestMode::Cors);

            let request = Request::new_with_str_and_init(url.as_str(), &opts).unwrap();
            web_sys::console::log_1(&serde_wasm_bindgen::to_value("Formed request").unwrap());
            let window = web_sys::window().expect("no global `window` exists");
            let resp_value: Response = JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap()
                .dyn_into()
                .unwrap();
            web_sys::console::log_1(&serde_wasm_bindgen::to_value("Got response").unwrap());

            let json = JsFuture::from(resp_value.json().unwrap()).await.unwrap();
            web_sys::console::log_1(&serde_wasm_bindgen::to_value("Got JSON").unwrap());
            web_sys::console::log_1(&json);

            let res = match serde_wasm_bindgen::from_value::<Vec<QuestionRecord>>(json) {
                Ok(val) => val,
                Err(e) => {
                    web_sys::console::log_1(
                        &serde_wasm_bindgen::to_value(&format!("Got Error!: {e}")).unwrap(),
                    );
                    vec![]
                }
            };
            web_sys::console::log_1(&serde_wasm_bindgen::to_value("Got Result").unwrap());
            Ok(res.into_iter().map(Into::into).collect())
        }
    }

    fn write(
        &self,
        values: Vec<conference_core::Question>,
    ) -> impl std::future::Future<Output = Result<()>> {
        async move {
            let url = self.base_url.join("/v1/questions").unwrap();
            let mut opts = RequestInit::new();
            opts.method("POST");
            opts.mode(web_sys::RequestMode::Cors);
            let header =
                serde_wasm_bindgen::to_value(&[["content-type", "application/json"]]).unwrap();
            opts.headers(&header);
            let value =
                serde_wasm_bindgen::to_value(&serde_json::to_string(&values).unwrap()).unwrap();
            opts.body(Some(&value));

            let request = Request::new_with_str_and_init(url.as_str(), &opts).unwrap();

            let window = web_sys::window().expect("no global `window` exists");
            let resp_value: Response = JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap()
                .dyn_into()
                .unwrap();

            Ok(())
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionRecord {
    content: String,
    id: f64,
    time_created: f64,
}

impl Into<Question> for QuestionRecord {
    fn into(self) -> Question {
        Question {
            content: self.content,
            id: self.id as u64,
            time_created: self.time_created as u128,
        }
    }
}

impl From<Question> for QuestionRecord {
    fn from(value: Question) -> Self {
        QuestionRecord {
            content: value.content,
            id: value.id as f64,
            time_created: value.time_created as f64,
        }
    }
}

pub struct RandomIDProvider {}

impl IDProvider for RandomIDProvider {
    fn new_id(&self) -> u64 {
        (js_sys::Math::random() * 1_000_000.0) as u64
    }
}

struct JsTimeProvider {}

impl TimeProvider for JsTimeProvider {
    fn now(&self) -> u128 {
        js_sys::Date::now() as u128
    }
}

#[wasm_bindgen]
impl ConferenceManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let base_url = "http://localhost:8282";
        Self {
            inner: InnerConferenceManager::new_with_tp(
                InMemoryStore::default(),
                FetchHttpClient {
                    base_url: Url::parse(&base_url).unwrap(),
                },
                JsTimeProvider {},
                RandomIDProvider {},
            ),
        }
    }

    #[wasm_bindgen]
    pub async fn get(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.inner.get_local().await.unwrap()).unwrap()
    }

    #[wasm_bindgen]
    pub async fn write(&self, value: &str) {
        self.inner.write(value).await.unwrap()
    }

    #[wasm_bindgen]
    pub async fn sync(&self) {
        self.inner.sync().await.unwrap()
    }
}

#[cfg(test)]
mod tests {}
