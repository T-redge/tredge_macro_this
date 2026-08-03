// Copyright © Viron Software ⨈
pub mod db_helper;
#[macro_export]
macro_rules! generic_type_checker {
    //Checks type for singular generic, returns column name and type on failure
    ($col:expr, $generic_name:ident) => {{
        //typing the $col makes sure that is the type that is passed into the function
        let col: &tokio_postgres::Column = $col.next().unwrap();
        if *col.type_() != $generic_name::get_db_type() {
            let name: &str = col.name();
            let t: String = col.type_().to_string();
            Err(name.to_string() + ":" + &t)
        } else {
            Ok(())
        }
    }};
    //unfolds repeating variables to enable checking a singular generic if one fails it doesnt
    //continue unraveling and returns error
    ($col:expr, $generic_name:ident, $($m_generic_name:ident),+) => {{
        match generic_type_checker!($col, $generic_name) {
            Err(e) => return Err(e),
            Ok(()) => match generic_type_checker!($col, $($m_generic_name),+) {
                Err(e) => return Err(e),
                Ok(()) => {
                    Ok(())
                }
            }
        }
    }};
}
#[macro_export]
macro_rules! get_column_value {
    ($ind:expr,$db_row:expr,$generic_name:ident) => {{
        let i: &mut usize = &mut $ind;
        let row: &tokio_postgres::Row = $db_row;
        let res: Result<$generic_name, Error> = row.try_get(*i);
        match res {
            Err(e) => return Err(e.to_string()),
            Ok(val) => {
                *i += 1;
                val
            }
        }
    }};
}
//Added ability to create the fn name so the macro can be reused without compilation errors
#[macro_export]
macro_rules! build_tuple {
    ($fn_name:ident,$($g_name:ident),+) => {
        pub async fn $fn_name<'a,$($g_name:'static +  $crate::db_helper::GetDbType + tokio_postgres::types::FromSql<'a>),+>(
            query: &'a Result<Vec<tokio_postgres::Row>, Error>,
            file: &'static str,
            line: u32,
        ) -> Result<Vec<($($g_name),+)>,String>{
            match query {
                Err(err) => Err($crate::db_helper::handle_error(err,file,line).await),
                Ok(query) => {
                    //Checks query returned any data,
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
                    //Ensures Generic types match with database types
                    let type_check = generic_type_checker!(c, $($g_name),+);
                    match type_check {
                        Err(e) => return Err(e),
                        Ok(()) => {
                            let mut result:Vec<($($g_name),+)> = Vec::with_capacity(query.len());
                            for db_row in query {
                                //iterates over columns in row without having to write boilerplate
                                //for each type
                                let mut i: usize = 0;
                                let row_tuple = (
                                    $(get_column_value!(i,db_row,$g_name),)+
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
