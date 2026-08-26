use macro_this::{build_list, build_tuple, generic_check, generic_type_checker, get_column_value};
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

    let q1 = client.query("SELECT * FROM player", &[]).await;
    build_list!(get_3_list, T1, T2, T3);
    let res_one = get_3_list::<i32, String, String>(&q1, "macro_this/main.rs", 16).await;
    let res_one = res_one.unwrap_or_default();

    println!("{:?}", res_one);

    let q2 = client
        .query("SELECT * FROM player WHERE player_id = 1", &[])
        .await;
    build_tuple!(get_3_tuple, T1, T2, T3);
    let res_two = get_3_tuple::<i32, String, String>(&q2, "macro_this/main.rs", 23).await;
    let res_two = res_two.unwrap_or_default();

    println!("{:?}", res_two);
    Ok(())
}
