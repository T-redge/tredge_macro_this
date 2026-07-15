use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Error> {
    /*let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=cricket", NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let q = client.query("SELECT * From player", &[]).await;
    let q = q.unwrap();*/

    Ok(())
}
