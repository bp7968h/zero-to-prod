use crate::domain::SubscriberEmail;
use reqwest::Client;
use secrecy::SecretBox;

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    key: String
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail, key: &String) -> EmailClient {
        EmailClient { http_client: Client::new(), base_url, sender, key: key.to_owned() }
    }

    pub async fn send_email(&self, recipient: SubscriberEmail, subject: &str, html_content: &str, text_content: &str) -> Result<(), String> {
        todo!()
    }
}