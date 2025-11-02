use std::time::Duration;

use anyhow::Result;
use sqlx::{Pool, Postgres};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt, core::ContainerPort, runners::AsyncRunner,
};

pub struct TestDb {
    pub _conn: String,
    pub _pool: Pool<Postgres>,
    _container: ContainerAsync<GenericImage>,
}

impl TestDb {
    pub async fn new() -> Result<Self> {
        let port_num = ContainerPort::from(5432);
        let image = GenericImage::new("postgres", "18-alpine")
            .with_exposed_port(port_num)
            .with_env_var("POSTGRES_DB", "postgres")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "postgres");

        let container = image.start().await?;
        let port = container.get_host_port_ipv4(port_num).await?;
        let uri = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
        let pool = loop {
            match sqlx::PgPool::connect(&uri).await {
                Ok(pool) => break pool,
                Err(_) => tokio::time::sleep(Duration::from_millis(100)).await,
            }
        };

        Ok(Self {
            _conn: uri,
            _pool: pool,
            _container: container,
        })
    }
}
