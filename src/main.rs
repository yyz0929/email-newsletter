use email_newsletter::configuration::get_configuration;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    let connection_string = configuration.database.connection_string();
    let connection = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    email_newsletter::startup::run(listener, connection).await.unwrap();
}
