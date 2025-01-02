use common::{constants::TOKEN_NAME, printlog, types::types::CommandError};
use google_drive3::{hyper_rustls, hyper_util, yup_oauth2, DriveHub};
use hyper_util::client::legacy::connect::HttpConnector;

pub struct DriveManager {
    hub: Option<DriveHub<hyper_rustls::HttpsConnector<HttpConnector>>>,
}

impl DriveManager {
    pub async fn new() -> Result<Self, CommandError> {
        let client_id = std::env::var("GOOGLE_CLIENT_ID")
            .map_err(|_| CommandError::Error("Missing GOOGLE_CLIENT_ID".to_string()))?;
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
            .map_err(|_| CommandError::Error("Missing GOOGLE_CLIENT_SECRET".to_string()))?;

        printlog!("Client ID: {}", client_id);

        let secret = yup_oauth2::ApplicationSecret {
            client_id,
            client_secret,
            auth_uri: "https://accounts.google.com/o/oauth2/auth".to_string(),
            token_uri: "https://oauth2.googleapis.com/token".to_string(),
            redirect_uris: vec!["http://localhost:45955".to_string()],
            ..Default::default()
        };

        let data_path = crate::service::settings::get_data_path();
        let token_path = std::path::Path::new(&data_path.config_path).join(TOKEN_NAME);

        let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .persist_tokens_to_disk(token_path.clone())
        .build()
        .await
        .map_err(|e| {
            printlog!("Auth error: {}", e);
            CommandError::Error(e.to_string())
        })?;

        printlog!("Auth initialized, token path: {:?}", token_path);

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
            let result = hub
                .auth
                .get_token(&["https://www.googleapis.com/auth/drive.file"])
                .await;

            match result {
                Ok(_) => true,
                Err(e) => {
                    printlog!("Token validation error: {}", e);
                    false
                }
            }
        } else {
            false
        }
    }
}
