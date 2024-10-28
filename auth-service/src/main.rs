use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::{
    app_state::AppState,  
    services::{
        hashmap_two_fa_code_store::HashmapTwoFACodeStore, 
        hashmap_user_store::HashmapUserStore, 
        hashset_banned_token_store::HashsetBannedTokenStore,
        mock_email_client::MockEmailClient}, 
    utils::constants::prod, Application
};

#[tokio::main]
async fn main() {
      // construct a subscriber that prints formatted traces to stdout
      // Start configuring a `fmt` subscriber
      let subscriber = tracing_subscriber::fmt()
            // Use a more compact, abbreviated log format
            .compact()
            // Display source code file paths
            .with_file(true)
            // Display source code line numbers
            .with_line_number(true)
            // Display the thread ID an event was recorded on
            .with_thread_ids(true)
            // Don't display the event's target (module path)
            .with_target(false)
            // Build the subscriber
            .finish();
      // use that subscriber to process traces emitted after this point
      tracing::subscriber::set_global_default(subscriber).unwrap();
    let user_store = Arc::new(RwLock::new(HashmapUserStore::default()));
    let banned_token_store= Arc::new(RwLock::new(HashsetBannedTokenStore::default()));
    let two_fa_code_store = Arc::new(RwLock::new(HashmapTwoFACodeStore::default()));
    let email_client = Arc::new(RwLock::new(MockEmailClient));
    let app_state = AppState::new(user_store,banned_token_store,two_fa_code_store, email_client);
    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}
