mod common;
use anyhow::Result;
use common::app::TestApp;
use common::db::TestDb;
use reqwest::Client;

#[tokio::test]
async fn test_db_connection_works() -> Result<()> {
    let db = TestDb::new().await?;
    let res: (i32,) = sqlx::query_as("select 1").fetch_one(&db._pool).await?;
    assert_eq!(res.0, 1);
    return Ok(());
}

#[tokio::test]
async fn test_app_startup_works() -> Result<()> {
    let db = TestDb::new().await?;
    let res: (i32,) = sqlx::query_as("select 1").fetch_one(&db._pool).await?;
    assert_eq!(res.0, 1);

    let app = TestApp::spawn(&db._conn).await?;
    assert!(app.child.id() > 0);

    println!("{:?}", app);
    let url = format!("{}/health", app.base_url);
    println!("TARGET: {}", &url);

    let res = Client::new().get(url).send().await?;

    assert!(res.status().is_success());

    return Ok(());
}
