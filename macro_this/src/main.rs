use macro_this::data_from_query;
use tokio_postgres::{types::FromSql, Error, NoTls};
#[tokio::main]
async fn main() -> Result<(), Error> {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=cricket", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let q = client.query("SELECT * From player", &[]).await;
    let x = data_from_query!();

    Ok(())
}
