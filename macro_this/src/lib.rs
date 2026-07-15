// Copyright © Viron Software ⨈
pub mod db_helper;
#[macro_export]
macro_rules! data_from_query {
    ($($g_name:ident),+) => {
            pub struct DataTuple($($g_name),+);
            pub async fn get_data_from_query<'a,$($g_name: $crate::db_helper::GetDbType + tokio_postgres::types::FromSql<'a>),+>(
                query: &'a Result<Vec<tokio_postgres::Row>, Error>,
                file: &'static str,
                line: u32,
            ) -> Result<($($g_name),+),String> {
                match query {
                    Err(err) => Err($crate::db_helper::handle_error(err,file,line).await),
                    Ok(query) => {
i                       let data_tup = DataTuple($($g_name)+)
                        if query.len() < 1 {
                            let msg = "Unexpected DB response (rows)".to_string();
                            $crate::db_helper::create_alert_ex(file, line, &msg).await;
                            return Err(msg)
                        }
                        let db_val = query.first().unwrap();
                        if db_val.columns().len() < 1 {
                            let msg = "Unexpected DB response (columns)".to_string();
                            $crate::db_helper::create_alert_ex(file, line, &msg).await;
                            return Err(msg);
                        }
                        let mut col_type = db_val.columns().iter();
                        'type_check: loop {
                            match col_type.next() {
                                Some(col) => {
                                    println!("{}",col.type_());
                                    if *col.type_() !=  {
                                        let msg = "Unexpected DB response (column 1 type)".to_string();
                                        $crate::db_helper::create_alert_ex(file, line, &msg).await;
                                        return Err(msg);
                                    }
                                }
                                None => break 'type_check
                            }
                        }
                            let msg = "End of macro!".to_string();
                                $crate::db_helper::create_alert_ex(file, line, &msg).await;
                                return Err(msg);
                    }
                }
            }
        };
}
