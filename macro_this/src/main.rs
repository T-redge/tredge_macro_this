use macro_this::{generic_type_checker, get_tuple};
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
    get_tuple!(T);
    let res: Result<i32, String> = get_data_from_query(&q, "", 0).await;
    match res {
        Err(e) => println!("{e}"),
        Ok(v) => println!("{v}"),
    }
    Ok(())
}
