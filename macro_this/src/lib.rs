// Copyright © Viron Software ⨈
pub mod db_helper;
#[macro_export]
macro_rules! data_from_query {
    ($tuple_name: ident, $data: expr) => {
        pub trait GetDbType {
            fn get_db_type() -> tokio_postgres::types::Type;
        }

        impl GetDbType for i32 {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::INT4
            }
        }

        impl GetDbType for i64 {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::INT8
            }
        }

        impl GetDbType for Option<i64> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::INT8
            }
        }

        impl GetDbType for &str {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::TEXT
            }
        }

        impl GetDbType for uuid::Uuid {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::UUID
            }
        }

        impl GetDbType for chrono::DateTime<chrono::Utc> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::TIMESTAMPTZ
            }
        }

        impl GetDbType for Option<chrono::DateTime<chrono::Utc>> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::TIMESTAMPTZ
            }
        }

        impl GetDbType for Vec<u8> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::BYTEA
            }
        }

        impl GetDbType for bool {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::BOOL
            }
        }

        impl GetDbType for Vec<&str> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::TEXT_ARRAY
            }
        }

        impl GetDbType for Vec<i32> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::INT4_ARRAY
            }
        }

        impl GetDbType for Vec<i64> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::INT8_ARRAY
            }
        }

        impl GetDbType for Vec<rust_decimal::Decimal> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::NUMERIC_ARRAY
            }
        }

        impl GetDbType for Vec<chrono::DateTime<chrono::Utc>> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::TIMESTAMPTZ_ARRAY
            }
        }

        impl GetDbType for Vec<bool> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::BOOL_ARRAY
            }
        }

        impl GetDbType for std::net::IpAddr {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::INET
            }
        }

        impl GetDbType for rust_decimal::Decimal {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::NUMERIC
            }
        }

        impl GetDbType for Option<rust_decimal::Decimal> {
            fn get_db_type() -> tokio_postgres::types::Type {
                tokio_postgres::types::Type::NUMERIC
            }
        }
        async fn get_data_from_query<
            'a,
            $tuple_name: GetDbType + tokio_postgres::types::FromSql<'a>,
        >(
            query: &'a Result<Vec<tokio_postgres::Row>, Error>,
            file: &'static str,
            line: u32,
        ) -> Result<$tuple_name, String> {
            match query {
                Err(err) => Err($crate::db_helper::handle_error(err, file, line).await),
                Ok(query) => {
                    if query.len() != 1 {
                        let msg = "Unexpected DB response (rows)".to_string();
                        $crate::db_helper::create_alert_ex(file, line, &msg).await;
                        return Err(msg);
                    }
                    let db_val = query.first().unwrap();
                    if db_val.columns().len() != 1 {
                        let msg = "Unexpected DB response (columns)".to_string();
                        $crate::db_helper::create_alert_ex(file, line, &msg).await;
                        return Err(msg);
                    }
                    if *db_val.columns()[0].type_() != $tuple_name::get_db_type() {
                        let msg = "Unexpected DB response (column 1 type)".to_string();
                        $crate::db_helper::create_alert_ex(file, line, &msg).await;
                        return Err(msg);
                    }
                    let db_val1: Result<$tuple_name, Error> = db_val.try_get(1);
                    match db_val1 {
                        Err(err) => {
                            let msg = format!("{:?}", err);
                            $crate::db_helper::create_alert_ex(file, line, &msg).await;
                            Err(msg)
                        }
                        Ok(db_val1) => Ok(db_val1),
                    }
                }
            }
        }
    };
}
