use std::{str::FromStr, sync::Arc};
use sqlx::{postgres::{PgConnectOptions, PgPoolOptions}, Connection, Executor, PgConnection, PgPool};
use tokio::sync::RwLock;
use reqwest::cookie::Jar;
use auth_service::{
    app_state::{AppState, BannedTokenStoreType, TwoFACodeStoreType}, 
    get_postgres_pool, get_redis_client, 
    services::{
        //hashmap_two_fa_code_store::HashmapTwoFACodeStore, 
        //hashmap_user_store::HashmapUserStore, 
        //hashset_banned_token_store::HashsetBannedTokenStore, 
        mock_email_client::MockEmailClient, 
        postgres_user_store::PostgresUserStore, 
        redis_banned_token_store::RedisBannedTokenStore, 
        redis_two_fa_code_store::RedisTwoFACodeStore,
    },
    utils::constants::{test, DATABASE_URL, DEFAULT_REDIS_HOSTNAME}, 
    Application
};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub banned_token_store: BannedTokenStoreType,
    pub two_fa_code_store: TwoFACodeStoreType,
    pub http_client: reqwest::Client, 
    pub db_name: String,
    pub clean_up_called: bool,
}

impl TestApp {
    #[tracing::instrument(name = "Creating test app", skip_all)]
    pub async fn new() -> Self {
        let (pg_pool,db_name) = configure_postgresql().await;
        //let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
        let user_store = Arc::new(RwLock::new(PostgresUserStore::new(pg_pool)));
        
        let redis_conn = Arc::new(RwLock::new(configure_redis()));
        
        //let banned_token_store= Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
        let banned_token_store= Arc::new(RwLock::new(RedisBannedTokenStore::new(redis_conn.clone())));
        
        //let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
        let two_fa_code_store= Arc::new(RwLock::new(RedisTwoFACodeStore::new(redis_conn)));
        
        let email_client = Arc::new(MockEmailClient);

        let app_state = AppState::new(
            user_store,
            banned_token_store.clone(),
            two_fa_code_store.clone(),
            email_client
        );

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread.
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let cookie_jar = Arc::new(Jar::default());
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(2)) 
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap(); // Create a Reqwest http client instance

        // Create new `TestApp` instance and return it
        TestApp {
            address,
            cookie_jar,
            banned_token_store,
            two_fa_code_store,
            http_client,
            db_name,
            clean_up_called: false,
        }
    }

     pub async fn clean_up (&mut self){ 
        if self.clean_up_called {
            return;
        }
        delete_database(&self.db_name).await;
        self.clean_up_called = true;
    } 

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa<Body>(&self, body: &Body) -> reqwest::Response 
        where
          Body: serde::Serialize
        {
            self.http_client
                .post(&format!("{}/verify-2fa", &self.address))
                .json(body)
                .send()
                .await
                .expect("Failed to execute request.")
    }

    pub async fn post_verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }
}

impl Drop for TestApp {
    fn drop(&mut self) {
        if !self.clean_up_called {
            panic!("TestApp::clean_up was not called before dropping TestApp");
        }
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

async fn configure_postgresql() -> (PgPool, String) {
    let postgresql_conn_url = DATABASE_URL.to_owned();

    // We are creating a new database for each test case, and we need to ensure each database has a unique name!
    let db_name = Uuid::new_v4().to_string();
    configure_database(&postgresql_conn_url, &db_name).await;
    let postgresql_conn_url_with_db = format!("{}/{}", postgresql_conn_url, db_name);

    // Create a new connection pool and return it
    (get_postgres_pool(&postgresql_conn_url_with_db)
        .await
        .expect("Failed to create Postgres connection pool!"), db_name)

}

async fn configure_database(db_conn_string: &str, db_name: &str) {
    // Create database connection
    let connection = PgPoolOptions::new()
        .connect(db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");
    // Create a new database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database.");
    // Connect to new database
    let db_conn_string = format!("{}/{}", db_conn_string, db_name);
    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");
    // Run migrations against new database
    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database");
}

async fn delete_database(db_name: &str) {
    let postgresql_conn_url: String = DATABASE_URL.to_owned();

    let connection_options = PgConnectOptions::from_str(&postgresql_conn_url)
        .expect("Failed to parse PostgreSQL connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect to Postgres");

    // Kill any active connections to the database
    connection
        .execute(
            format!(
                r#"
                SELECT pg_terminate_backend(pg_stat_activity.pid)
                FROM pg_stat_activity
                WHERE pg_stat_activity.datname = '{}'
                  AND pid <> pg_backend_pid();
        "#,
                db_name
            )
            .as_str(),
        )
        .await
        .expect("Failed to drop the database.");

    // Drop the database
    connection
        .execute(format!(r#"DROP DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to drop the database.");
}

fn configure_redis() -> redis::Connection {

    get_redis_client(DEFAULT_REDIS_HOSTNAME.to_owned())
        .expect("Failed to get Redis client")
        .get_connection()
        .expect("Failed to get Redis connection")
}