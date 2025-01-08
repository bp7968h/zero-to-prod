use crate::domain::SubscriberEmail;
use reqwest::Client;

#[derive(Clone)]
pub struct EmailClient {
    _http_client: Client,
    _base_url: String,
    _sender: SubscriberEmail,
    _key: String,
}

impl EmailClient {
    pub fn new(_base_url: String, _sender: SubscriberEmail, key: &String) -> EmailClient {
        EmailClient {
            _http_client: Client::new(),
            _base_url,
            _sender,
            _key: key.to_owned(),
        }
    }

    pub async fn send_email(
        &self,
        _recipient: SubscriberEmail,
        _subject: &str,
        _html_content: &str,
        _text_content: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
