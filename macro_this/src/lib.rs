// Copyright © Viron Software ⨈
pub mod db_helper;

macro_rules! generic_type_checker {
    ($col:expr, $generic_name:ident) => {{
        let col: &tokio_postgres::Column = $col.next().unwrap();
        if *col.type_() != $generic_name::get_db_type() {
            Err(format!("Unexpected DB response (column types: received {}, expected {})", col.type_(), $generic_name::get_db_type()))
        } else {
            Ok(())
        }

    }};
    ($col:expr, $generic_name:ident, $($m_generic_name:ident),+) => {{
        match generic_type_checker!($col, $generic_name) {
            Err(e) => Err(e),
            Ok(()) => generic_type_checker!($col, $($m_generic_name),+)
        }
    }};
}

macro_rules! get_column_value {
    ($ind:expr,$db_row:expr,$generic_name:ident,$file:expr,$line:expr) => {{
        let res: Result<$generic_name, Error> = $db_row.try_get(*$ind);
        match res {
            Err(err) => {
                let msg = format!("{:?}", err);
                v_common::create_alert_ex($file, $line, &msg).await;
                return Err(msg)
            },
            Ok(val) => {
                *$ind += 1;
                val
            }
        }
    }};
}

macro_rules! generic_count {
    ($($g:ident),*) => {
        generic_count!(@count $($g),*)
    };
    (@count) => { 0 };
    (@count $head:ident $(, $tail:ident)*) => {
        1 + generic_count!(@count $($tail),*)
    };
}

macro_rules! build_list {
    ($fn_name:ident, $($g_name:ident),+) => {
        pub async fn $fn_name<'a, $($g_name: $crate::db_helper::GetDbType + tokio_postgres::types::FromSql<'a>),+>(
            query: &'a Result<Vec<tokio_postgres::Row>, Error>,
            file: &'static str,
            line: u32,
        ) -> Result<Vec<($($g_name),+)>,String>{
            match query {
                Err(err) => Err($crate::db_helper::handle_error(err,file,line).await),
                Ok(query) => {
                    if query.is_empty() {
                        return Ok(vec![]);
                    }
                    let db_val = query.first().unwrap();
                    let columns = db_val.columns();
                    if columns.len() != generic_count!($($g_name),+) {
                        let msg = "Unexpected DB response (columns)".to_string();
                        v_common::create_alert_ex(file, line, &msg).await;
                        return Err(msg);
                    }
                    let mut columns_iter = columns.iter();
                    let type_check = generic_type_checker!(columns_iter, $($g_name),+);
                    match type_check {
                        Err(msg) => {
                            v_common::create_alert_ex(file, line, &msg).await;
                            Err(msg)
                        },
                        Ok(()) => {
                            let mut result: Vec<($($g_name),+)> = Vec::with_capacity(query.len());
                            for db_row in query {
                                let mut i: usize = 0;
                                let row_tuple = (
                                    $(get_column_value!(&mut i, db_row, $g_name, file, line),)+
                                );
                                result.push(row_tuple);
                            }
                            Ok(result)
                        }
                    }
                }
            }
        }
    };
}

macro_rules! build_tuple {
    ($fn_name:ident,$($g_name:ident),+) => {
        pub async fn $fn_name<'a,$($g_name: GetDbType + FromSql<'a>),+>(
            query: &'a Result<Vec<Row>, Error>,
            file: &'static str,
            line: u32,
        ) -> Result<($($g_name),+), String>{
            match query {
                Err(err) => Err(handle_error(err,file,line).await),
                Ok(query) => {
                    if query.len() != 1 {
                        let msg = "Unexpected DB response (rows)".to_string();
                        v_common::create_alert_ex(file, line, &msg).await;
                        return Err(msg)
                    }
                    let db_val = query.first().unwrap();
                    let columns = db_val.columns();
                    if columns.len() != generic_count!($($g_name),+) {
                        let msg = "Unexpected DB response (columns)".to_string();
                        v_common::create_alert_ex(file, line, &msg).await;
                        return Err(msg);
                    }
                    let mut columns_iter = columns.iter();
                    let type_check = generic_type_checker!(columns_iter, $($g_name),+);
                    match type_check {
                        Err(msg) => {
                            v_common::create_alert_ex(file, line, &msg).await;
                            Err(msg)
                        },
                        Ok(()) => {
                            let mut i: usize = 0;
                            let row_tuple = (
                                $(get_column_value!(&mut i, db_val, $g_name, file, line),)+
                            );
                            Ok(row_tuple)
                        }
                    }
                }
            }
        }
    };
}

build_tuple!(get_2_tuple_from_query,  T1, T2);
build_tuple!(get_3_tuple_from_query,  T1, T2, T3);
build_tuple!(get_4_tuple_from_query,  T1, T2, T3, T4);
build_tuple!(get_5_tuple_from_query,  T1, T2, T3, T4, T5);
build_tuple!(get_6_tuple_from_query,  T1, T2, T3, T4, T5, T6);
build_tuple!(get_7_tuple_from_query,  T1, T2, T3, T4, T5, T6, T7);
build_tuple!(get_8_tuple_from_query,  T1, T2, T3, T4, T5, T6, T7, T8);
build_tuple!(get_9_tuple_from_query,  T1, T2, T3, T4, T5, T6, T7, T8, T9);
build_tuple!(get_10_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
build_tuple!(get_11_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
build_tuple!(get_12_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
build_tuple!(get_13_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
build_tuple!(get_14_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
build_tuple!(get_15_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
build_tuple!(get_16_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
build_tuple!(get_17_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
build_tuple!(get_18_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
build_tuple!(get_19_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
build_tuple!(get_20_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
build_tuple!(get_21_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
build_tuple!(get_22_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
build_tuple!(get_23_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
build_tuple!(get_24_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
build_tuple!(get_25_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
build_tuple!(get_26_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
build_tuple!(get_27_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
build_tuple!(get_28_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
build_tuple!(get_29_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
build_tuple!(get_30_tuple_from_query, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
build_list!(get_2_list_from_query,    T1, T2);
build_list!(get_3_list_from_query,    T1, T2, T3);
build_list!(get_4_list_from_query,    T1, T2, T3, T4);
build_list!(get_5_list_from_query,    T1, T2, T3, T4, T5);
build_list!(get_6_list_from_query,    T1, T2, T3, T4, T5, T6);
build_list!(get_7_list_from_query,    T1, T2, T3, T4, T5, T6, T7);
build_list!(get_8_list_from_query,    T1, T2, T3, T4, T5, T6, T7, T8);
build_list!(get_9_list_from_query,    T1, T2, T3, T4, T5, T6, T7, T8, T9);
build_list!(get_10_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
build_list!(get_11_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
build_list!(get_12_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
build_list!(get_13_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
build_list!(get_14_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
build_list!(get_15_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
build_list!(get_16_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
build_list!(get_17_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
build_list!(get_18_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
build_list!(get_19_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
build_list!(get_20_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
build_list!(get_21_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
build_list!(get_22_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
build_list!(get_23_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
build_list!(get_24_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
build_list!(get_25_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
build_list!(get_26_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
build_list!(get_27_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
build_list!(get_28_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
build_list!(get_29_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
build_list!(get_30_list_from_query,   T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
