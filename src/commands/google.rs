pub mod gdrive {

    use google_drive3::{
        self as drive3, api::Permission, hyper_rustls::HttpsConnector,
        hyper_util::client::legacy::connect::HttpConnector, yup_oauth2 as oauth2,
    };

    pub async fn instantiate_hub(
        path: &str,
    ) -> Result<drive3::api::DriveHub<HttpsConnector<HttpConnector>>, crate::utils::Error> {
        let secret = oauth2::read_service_account_key(path).await?;

        let auth = oauth2::ServiceAccountAuthenticator::builder(secret)
            .persist_tokens_to_disk("tokencachegdocs.json")
            .build()
            .await
            .expect("auth problem");

        let client = drive3::hyper_util::client::legacy::Client::builder(
            drive3::hyper_util::rt::TokioExecutor::new(),
        )
        .build(
            drive3::hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .expect("client problem")
                .https_or_http()
                .enable_http1()
                .build(),
        );

        Ok(drive3::DriveHub::new(client, auth))
    }

    pub async fn change_perms(
        hub: &drive3::api::DriveHub<HttpsConnector<HttpConnector>>,
        file_id: &str,
        permission_type: crate::utils::Perms,
    ) -> Result<Permission, crate::utils::Error> {
        let perm = match permission_type {
            crate::utils::Perms::Viewer() => "reader",
            crate::utils::Perms::Commenter() => "commenter",
            crate::utils::Perms::Editor() => "writer",
            crate::utils::Perms::Owner() => "owner",
        };

        println!("changing perms to {}", perm);

        let permission = google_drive3::api::Permission {
            role: Some(perm.to_string()),
            type_: Some("user".to_string()),
            email_address: Some("chaaskandregula@gmail.com".to_string()),
            ..Default::default()
        };

        let result = hub.permissions().create(permission, file_id).doit().await?;

        Ok(result.1)
    }
}

pub mod gdocs {
    use google_docs1::{
        self as docs1, hyper_rustls::HttpsConnector,
        hyper_util::client::legacy::connect::HttpConnector, yup_oauth2 as oauth2,
    };

    pub async fn instantiate_hub(
        path: &str,
    ) -> Result<docs1::api::Docs<HttpsConnector<HttpConnector>>, crate::utils::Error> {
        let secret = oauth2::read_service_account_key(path).await?;

        let auth = oauth2::ServiceAccountAuthenticator::builder(secret)
            .persist_tokens_to_disk("tokencachegdocs.json")
            .build()
            .await
            .expect("auth problem");

        let client = docs1::hyper_util::client::legacy::Client::builder(
            docs1::hyper_util::rt::TokioExecutor::new(),
        )
        .build(
            docs1::hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .expect("client problem")
                .https_or_http()
                .enable_http1()
                .build(),
        );

        Ok(docs1::api::Docs::new(client, auth))
    }
}
