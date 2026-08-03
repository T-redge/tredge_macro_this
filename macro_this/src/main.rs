use macro_this::{build_tuple, generic_type_checker, get_column_value};
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

    let q = client.query("SELECT * FROM player", &[]).await;
    build_tuple!(get_3_tuple, T1, T2, T3);
    let res = get_3_tuple::<i32, String, String>(&q, "macro_this/main.rs", 16).await;
    match res {
        Err(e) => {
            println!("{e}");
        }
        Ok(val) => {
            for row in val {
                println!("{row:?}");
            }
        }
    }
    build_tuple!(get_2_tuple, T1, T2);
    let res_two = get_2_tuple::<i32, String>(&q, "macro_this/main.rs", 26).await;
    match res_two {
        Err(e) => {
            println!("{e}");
        }
        Ok(val) => {
            for row in val {
                println!("{row:?}");
            }
        }
    }

    Ok(())
}
