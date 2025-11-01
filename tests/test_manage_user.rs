mod common;
use anyhow::Result;
use common::db::TestDb;
use common::app::TestApp;
use reqwest::{Client, header};
use serde_json::json;

struct TestFixture {
    _db: TestDb,
    app: TestApp,
}

async fn setup_app() -> Result<TestFixture> {
    let db = TestDb::new().await?;
    let app = TestApp::spawn(&db._conn).await?;
    Ok(TestFixture { _db: db, app })
}

#[tokio::test]
async fn test_user_create() -> Result<()> {

    let setup = setup_app().await?;
    let app_url = setup.app.base_url.clone();

    // Create user
    let url = format!("{}/api/users", app_url);

    let data_create_user = json!({
        "user": {
            "username": "user1",
            "email": "user1@users.com",
            "password": "password"
        }
    });

    let res = Client::new()
        .post(url)
        .json(&data_create_user)
        .send()
        .await?;

    assert!(res.status().is_success());

    // Login user
    let url = format!("{}/api/users/login", app_url);

    let data_login_user = json!({
        "user": {
            "email": "user1@users.com",
            "password": "password"
        }
    });

    let res = Client::new()
        .post(url)
        .json(&data_login_user)
        .send()
        .await?;

    assert!(res.status().is_success());
    let json_res: serde_json::Value = res.json().await.unwrap();
    dbg!(&json_res);
    let token = json_res["user"]["token"].as_str();

    assert!(token.is_some());

    // GET user
    let url = format!("{}/api/user", app_url);

    let res = Client::new()
        .get(url)
        .header(header::AUTHORIZATION, format!("Token {}", token.unwrap()))
        .send()
        .await?;

    assert!(res.status().is_success());

    let json_res: serde_json::Value = res.json().await?;
    //dbg!(&json_res);
    let user = json_res.get("user").unwrap();
    dbg!(user);

    assert_eq!(user["email"].as_str(), Some("user1@users.com"));
    assert_eq!(user["username"].as_str(), Some("user1"));




    return Ok(());
}
