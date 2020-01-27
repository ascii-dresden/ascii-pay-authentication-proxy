use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
use std::marker::Sized;
use std::time::Duration;

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum IdentificationRequest {
    Barcode {
        code: String,
    },
    Nfc {
        id: String,
    },
    NfcSecret {
        id: String,
        challenge: String,
        response: String,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "kebab-case")]
pub enum IdentificationResponse {
    Account {
        #[serde(flatten)]
        account: Value,
    },
    Product {
        #[serde(flatten)]
        product: Value,
    },
    AuthenticationNeeded {
        id: String,
        key: String,
        challenge: String,
    },
    WriteKey {
        id: String,
        key: String,
        secret: String,
    },
    NotFound,
}

pub fn send_request<T, R>(url: &str, body: T) -> Option<R>
where
    T: Serialize + Sized,
    R: DeserializeOwned,
{
    let client = reqwest::blocking::Client::new();

    let response = client
        .post(url)
        .timeout(Duration::from_secs(10))
        .json(&body)
        .send()
        .ok()?;

    response.json().ok()
}

pub fn send_identify(body: IdentificationRequest) -> Option<IdentificationResponse> {
    let client = reqwest::blocking::Client::new();

    let response = client
        .post("http://localhost:8080/api/v1/identify")
        .timeout(Duration::from_secs(10))
        .json(&body)
        .send()
        .ok()?;

    if response.status().as_u16() == 404 {
        return Some(IdentificationResponse::NotFound);
    }

    response.json().ok()
}
