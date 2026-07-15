use macro_this::data_from_query;
use tokio_postgres::{Error, NoTls};

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
    data_from_query!(T1, T2, T3);
    let x: Result<(i32, &str, &str), String> = get_data_from_query(&q, "", 0).await;
    match x {
        Err(e) => println!("{e}"),
        Ok(res) => println!("{res:?}"),
    }
    Ok(())
}
