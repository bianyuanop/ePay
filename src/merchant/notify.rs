use candid::{CandidType, Deserialize};
use serde::{Serialize};
use ic_cdk::api::management_canister::http_request::{HttpHeader, CanisterHttpRequestArgument, HttpMethod, http_request};

#[derive(CandidType, Deserialize, Clone)]
pub struct Notifier {
    pub host: String,
    pub address2notify: String
}

#[derive(CandidType, Deserialize, Serialize)]
pub enum Notification {
    OrderPaid(u64),
    OrderCommented(u64),
}

impl Notifier {
    pub async fn notify(&self, notification: Notification ) -> Result<(), String>{
        let request_headers = vec![
            HttpHeader {
                name: "Host".to_string(),
                value: format!("{}", self.host)
            },
            HttpHeader {
                name: "User-Agent".to_string(),
                value: "epay_notify_service".to_string()
            }
        ];

        let body = serde_json::to_string(&notification).unwrap().as_bytes().to_vec();
        

        let request = CanisterHttpRequestArgument {
            headers: request_headers,
            url: self.address2notify.clone(),
            method: HttpMethod::POST,
            body: Some(body),
            max_response_bytes: Some(200),
            transform: None
        };

        match http_request(request).await {
            Ok((_, )) => {
                Ok(())
            },
            Err((r, m)) => {
                Err(format!("The http_request resulted into error. RejectionCode: {r:?}, Error: {m}").into())
            }
        }
    }
}