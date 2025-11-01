use std::{
    process::{Child, Command, Stdio},
    time::Duration,
};

use tokio::time::sleep;

use anyhow::Result;
use rand::Rng;

#[derive(Debug)]
pub struct TestApp {
    pub base_url: String,
    pub child: Child,
}

impl TestApp {
    pub async fn spawn(db_url: &str) -> Result<Self> {
        let port = rand::rng().random_range(8000..9000);
        let base_url= format!("http://127.0.0.1:{port}");

        dbg!(db_url);

        let mut cmd = Command::new("cargo");
        cmd.args(["run", "--bin", "realworld-axum-sqlx"])
            .env("DATABASE_URL", db_url)
            .env("APP_ENV", "test")
            .env("PORT", port.to_string())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let child = cmd.spawn().expect("Failed to launch application");

        // Wait until the app responds to HTTP before proceeding
        let client = reqwest::Client::new();
        for _ in 0..20 {
            let resp = client.get(format!("{base_url}/health")).send().await;
            if resp.is_ok() {
                let resp = resp.unwrap();
                if resp.status().is_success() {
                    println!("App is ready!");
                    return Ok(Self { base_url, child });
                } else {
                    println!("Response not OK: {:?}", resp);
                }
            } else {
                println!("client get failed: {:?}", resp);
            }
            sleep(Duration::from_millis(300)).await;
        }


        println!("Launching application on port {port}");

        Ok(Self {
            base_url,
            child,
        })
    }
}

impl Drop for TestApp {
    fn drop(&mut self) { 
        let _ = self.child.kill();
    }
}