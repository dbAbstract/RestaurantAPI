#![feature(proc_macro_hygiene, decl_macro)]

extern crate serde_json;
use rocket::{*, http::Status};
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;
use rand::*;

#[derive(Serialize, Debug)]
struct Item {
    item_id: i64,
    quantity: i64,
    prep_time: i64,
}

#[derive(Serialize, Debug)]
struct ItemList {
    items: Vec<Item>
}
#[derive(Serialize)]
struct StatusMessage {
    message: String
}

// API END POINTS
// Home page
#[get("/")]
fn index() -> &'static str {
    "Welcome!"
}


// 1. POST a number of items into table endpoint
#[post("/item/<item_id>/<quantity>/<table_num>")]
fn add_item(item_id: i64, quantity:i64, table_num: i64) -> Result<Json<StatusMessage>, String> {
    let t = rand::thread_rng().gen_range(5..16);
    let item = Item{
        item_id: item_id,
        quantity: quantity,
        prep_time: t
    };

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    db_connection
            .execute(&format!("create table if not exists table_{} (
                item_id integer primary key,
                quantity integer not null,
                prep_time integer not null
            );", table_num), rusqlite::NO_PARAMS).unwrap();
    
    let mut statement =
        match db_connection.prepare(&format!("insert into table_{} (item_id, quantity, prep_time) values
        ({},{},{}) on conflict(item_id) do update set quantity = quantity + {};"
            , table_num, item_id, quantity, t, quantity)) {
                Ok(statement) => statement,
                Err(_) => return Err("Failed to prepare query".into()),
        };
    
    let results = statement.execute(rusqlite::NO_PARAMS);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(_) => Err("Failed to insert menu item".into()),
    }
}

// 2. DEL an item from a table endpoint
#[delete("/item/<item_id>/<table_num>")]
fn delete_item(item_id: i64, table_num: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };
    
    let mut statement = match db_connection.prepare(&format!("delete from table_{} where item_id = {};", table_num, item_id)) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let results = statement.execute(rusqlite::NO_PARAMS);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} row deleted!", rows_affected),
        })),
        Err(_) => Err("Failed to delete menu item".into()),
    }

}

// 3. GET all items for a table endpoint
#[get("/item/<table_num>")]
fn get_all_items(table_num: i64) -> Result<Json<ItemList>, String> {
    
    // Connecting to database
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    // Prepares SQL statement for the GET query
    let mut statement = match db_connection
        .prepare(&format!("select item_id, quantity, prep_time from table_{}", table_num)) {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        };

    // Accumulates all the rows from the table into a query map
    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(Item {
            item_id: row.get(0)?,
            quantity: row.get(1)?,
            prep_time: row.get(2)?,
        })
    });

    // 
    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<_>> = rows.collect();

            match collection {
                Ok(items) => {
                    println!("{:?}", items);
                    return Ok(Json(ItemList { items }));
                },
                Err(_) => Err("Could not collect items".into()),
            }
        }
        
        Err(_) => Err((&format!("Failed to fetch menu items for table_{}", table_num)).into()),
    }
}

// 4. GET specified item 

// 5. UPDATE quantity of some specified item 




fn main() {

    let db_connection = Connection::open("data.sqlite").unwrap();

    rocket::ignite()
    .mount(
        "/",
        routes![index, get_all_items, add_item, delete_item]
    )
    .launch();
}
