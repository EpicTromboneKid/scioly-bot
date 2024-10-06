pub mod google_drive {
    use drive3::{hyper, hyper_rustls, oauth2, DriveHub};
    use google_drive3 as drive3;

    use crate::utils::Error;

    pub async fn create_hub(
        path: String,
    ) -> Result<
        DriveHub<
            google_drive3::hyper_rustls::HttpsConnector<
                google_drive3::hyper::client::HttpConnector,
            >,
        >,
        Error,
    > {
        let secret: oauth2::ApplicationSecret = oauth2::read_application_secret(&path)
            .await
            .expect("where secret");

        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .build()
        .await
        .expect("problem w auth fam");

        let hub = DriveHub::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .expect("hi im in line 26")
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );
        Ok(hub)
    }
}

pub mod docs {
    use google_docs1::{hyper, hyper_rustls, oauth2, Docs};

    use crate::utils::Error;

    pub async fn create_hub(
        text: String,
    ) -> Result<
        Docs<
            google_drive3::hyper_rustls::HttpsConnector<
                google_drive3::hyper::client::HttpConnector,
            >,
        >,
        Error,
    > {
        let secret = oauth2::parse_application_secret(text)?;

        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .build()
        .await
        .expect("problem w auth fam");

        let hub = Docs::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .expect("hi im in line 26")
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            auth,
        );
        Ok(hub)
    }
}
