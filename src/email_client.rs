use crate::domain::SubscriberEmail;
use reqwest::Client;
use reqwest::multipart::Form;
use secrecy::{ExposeSecret, SecretBox};


pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    key: SecretBox<String>,
}

impl EmailClient {
    pub fn new(base_url: String, sender: SubscriberEmail, key: SecretBox<String>) -> EmailClient {
        EmailClient {
            http_client: Client::new(),
            base_url,
            sender,
            key,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = self.base_url.clone();
        let request_body = SendEmailRequest {
            from: self.sender.as_ref(),
            to: recipient.as_ref(),
            subject,
            html_body: html_content,
            text_body: text_content,
        };
        self.http_client.post(&url)
            .basic_auth("api", Some(self.key.expose_secret()))
            .multipart(request_body.into())
            .send()
            .await?;

        Ok(())
    }
}

struct SendEmailRequest<'a> {
    from: &'a str,
    to: &'a str,
    subject: &'a str,
    text_body: &'a str,
    html_body: &'a str,
}

impl<'a> From<SendEmailRequest<'a>> for Form {
    fn from(value: SendEmailRequest) -> Self {
        let mut form = Form::new();
        form = form
            .text("from", value.from.to_owned())
            .text("to", value.to.to_owned())
            .text("subject", value.subject.to_owned())
            .text("text", value.text_body.to_owned())
            .text("html", value.html_body.to_owned());

        form
    }
}


#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use wiremock::matchers::{basic_auth, method};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use secrecy::SecretBox;
    use wiremock_multipart::prelude::*;
    
    #[tokio::test]
    async fn send_email_sends_the_expected_request() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let api_key: Box<String> = Box::new(Faker.fake());
        let email_client = EmailClient::new(mock_server.uri(), sender, SecretBox::new(api_key.clone()));

        Mock::given(basic_auth("api", *api_key))
            .and(method("POST"))
            .and(NumberOfParts(5))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        // senf email
        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;

        // assert
    }
}