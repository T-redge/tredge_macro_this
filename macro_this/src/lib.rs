// Copyright © Viron Software ⨈
pub mod db_helper;
#[macro_export]
macro_rules! generic_type_checker {
    ($t:ident) => {
        async fn type_check<
            'a,
            $t: $crate::db_helper::GetDbType + tokio_postgres::types::FromSql<'a>,
        >(
            c: &tokio_postgres::Column,
        ) -> bool {
            if *c.type_() != $t::get_db_type() {
                false
            } else {
                true
            }
        }
    };
}
#[macro_export]
macro_rules! get_tuple {
    ($($g_name:ident),*) => {
        pub async fn get_data_from_query<'a,$($g_name: $crate::db_helper::GetDbType + tokio_postgres::types::FromSql<'a>),+>(
            query: &'a Result<Vec<tokio_postgres::Row>, Error>,
            file: &'static str,
            line: u32,
        ) -> Result<($($g_name),+),String> {
            match query {
                Err(err) => Err($crate::db_helper::handle_error(err,file,line).await),
                Ok(query) => {
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
                    let mut c = db_val.columns().iter();
                    $(
                        generic_type_checker!($g_name);
                        let res = type_check::<i32>(c.next().unwrap()).await;
                        if res {
                            let val: Result<$g_name,Error> = db_val.try_get(0);
                            match val {
                                Err(e) => return Err(e.to_string()),
                                Ok(v) => Ok(v),
                            }
                        } else {
                            Err("Shit".to_string())
                        }
                    )*

                }
            }
        }
    };
}
