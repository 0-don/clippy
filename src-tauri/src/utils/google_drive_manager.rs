use common::{constants::TOKEN_NAME, types::types::CommandError};
use google_drive3::{hyper_rustls, hyper_util, yup_oauth2, DriveHub};
use hyper_util::client::legacy::connect::HttpConnector;

pub struct DriveManager {
    hub: Option<DriveHub<hyper_rustls::HttpsConnector<HttpConnector>>>,
}

impl DriveManager {
    pub async fn new() -> Result<Self, CommandError> {
        let secret = yup_oauth2::ApplicationSecret {
            client_id: "YOUR_CLIENT_ID.apps.googleusercontent.com".to_string(),
            client_secret: "".to_string(),
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            redirect_uris: vec!["http://127.0.0.1:1420/oauth/callback".to_string()],
            ..Default::default()
        };

        // Use the config directory for token storage
        let data_path = crate::service::settings::get_data_path();
        let token_path = std::path::Path::new(&data_path.config_path).join(TOKEN_NAME);

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk(token_path)
        .build()
        .await
        .map_err(|e| CommandError::Error(e.to_string()))?;

        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
                .build(
                    hyper_rustls::HttpsConnectorBuilder::new()
                        .with_native_roots()
                        .unwrap()
                        .https_or_http()
                        .enable_http1()
                        .build(),
                );

        let hub = DriveHub::new(client, auth);
        Ok(Self { hub: Some(hub) })
    }

    pub async fn is_authenticated(&self) -> bool {
        if let Some(hub) = &self.hub {
            hub.auth
                .get_token(&["https://www.googleapis.com/auth/drive.file"])
                .await
                .is_ok()
        } else {
            false
        }
    }
}
