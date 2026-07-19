use macro_this::{build_tuple, flatten_tup, generic_type_checker, get_tuple};
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
    get_tuple!(T1, T2, T3);
    let res = get_data_from_query::<i32, String, String>(&q, "", 0).await;
    if let Err(e) = res {
        println!("{e}");
    }
    Ok(())
}
