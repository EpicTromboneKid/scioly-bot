use crate::utils::Error;

// Google Drive API
pub async fn drive(path: &str) -> Result<google_drive3::api::DriveHub<google_drive3::hyper_rustls::HttpsConnector<google_drive3::hyper_util::client::legacy::connect::HttpConnector>>, Error> {
    let secret = google_drive3::yup_oauth2::read_service_account_key(path).await?;
    let auth = google_drive3::yup_oauth2::ServiceAccountAuthenticator::builder(secret)
        .persist_tokens_to_disk("tokencachegdrive.json")
        .build()
        .await
        .map_err(|e| format!("Drive auth problem: {:?}", e))?;

    let client = google_drive3::hyper_util::client::legacy::Client::builder(
        google_drive3::hyper_util::rt::TokioExecutor::new(),
    ).build(
        google_drive3::hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("client problem")
            .https_or_http()
            .enable_http1()
            .build(),
    );

    Ok(google_drive3::DriveHub::new(client, auth))
}

// Google Docs API
pub async fn docs(path: &str) -> Result<google_docs1::api::Docs<google_docs1::hyper_rustls::HttpsConnector<google_docs1::hyper_util::client::legacy::connect::HttpConnector>>, Error> {
    let secret = google_docs1::yup_oauth2::read_service_account_key(path).await?;
    let auth = google_docs1::yup_oauth2::ServiceAccountAuthenticator::builder(secret)
        .persist_tokens_to_disk("tokencachegdocs.json")
        .build()
        .await
        .map_err(|e| format!("Docs auth problem: {:?}", e))?;

    let client = google_docs1::hyper_util::client::legacy::Client::builder(
        google_docs1::hyper_util::rt::TokioExecutor::new(),
    ).build(
        google_docs1::hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("client problem")
            .https_or_http()
            .enable_http1()
            .build(),
    );

    Ok(google_docs1::api::Docs::new(client, auth))
}

// Google Sheets API
pub async fn sheets(path: &str) -> Result<google_sheets4::api::Sheets<google_sheets4::hyper_rustls::HttpsConnector<google_sheets4::hyper_util::client::legacy::connect::HttpConnector>>, Error> {
    let secret = google_sheets4::yup_oauth2::read_service_account_key(path).await?;
    let auth = google_sheets4::yup_oauth2::ServiceAccountAuthenticator::builder(secret)
        .persist_tokens_to_disk("tokencachegsheets.json")
        .build()
        .await
        .map_err(|e| format!("Sheets auth problem: {:?}", e))?;

    let client = google_sheets4::hyper_util::client::legacy::Client::builder(
        google_sheets4::hyper_util::rt::TokioExecutor::new(),
    ).build(
        google_sheets4::hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("client problem")
            .https_or_http()
            .enable_http1()
            .build(),
    );

    Ok(google_sheets4::Sheets::new(client, auth))
}

pub fn get_link(event: &str, values: Vec<Vec<serde_json::Value>>) -> Option<String> {
    for row in values {
        if !row.is_empty() && row[0] == event {
            return Some(row[1].to_string());
        }
    }
    None
}

pub async fn change_perms(
    hub: &google_drive3::api::DriveHub<google_drive3::hyper_rustls::HttpsConnector<google_drive3::hyper_util::client::legacy::connect::HttpConnector>>,
    file_id: &str,
    permission_type: crate::utils::Perms,
    emails: &Vec<String>,
    change: (bool, &google_drive3::api::Permission),
) -> Result<Vec<(String, google_drive3::api::Permission)>, crate::utils::Error> {
    let mut out_vec = vec![];
    let perm = match permission_type {
        crate::utils::Perms::Viewer() => "reader",
        crate::utils::Perms::Commenter() => "commenter",
        crate::utils::Perms::Editor() => "writer",
        crate::utils::Perms::Owner() => "owner",
    };

    for email in emails {
        println!("email: {}", email);
        println!("changing perms to {}", perm);

        if change.0 {
            println!("{:?}", change.1);
            hub.permissions()
                .delete(file_id, change.1.id.as_ref().unwrap())
                .doit()
                .await?;
        }

        let permission = google_drive3::api::Permission {
            role: Some(perm.to_string()),
            type_: Some("user".to_string()),
            email_address: Some(email.to_string()),
            ..Default::default()
        };

        let result = hub
            .permissions()
            .create(permission.clone(), file_id)
            .doit()
            .await?;

        out_vec.push((email.to_string(), result.1));
    }
    Ok(out_vec)
}
