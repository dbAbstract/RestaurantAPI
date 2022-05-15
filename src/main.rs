#![feature(proc_macro_hygiene, decl_macro)]

// Imports
extern crate rocket;
use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;
use rand::*;

#[derive(Serialize, Debug)]

// Every item on the menu is an Item instance.
struct Item {
    item_id: i64,   // uniquely identifies every menu item
    quantity: i64,  // quantity of each item ordered at a table
    prep_time: i64, // time (minutes) taken for preparation of menu item
}

/*
ItemList is a struct that serves as a buffer for when multiple or single rows of menu items
are retrieved from the database.
*/
#[derive(Serialize, Debug)]
struct ItemList {
    items: Vec<Item> 
}

#[derive(Serialize)]
struct StatusMessage {
    message: String
}

// API END POINTS

// 0. Home page
#[get("/")]
fn index() -> &'static str {
    // Prints out "to the browser page"
    "Welcome!"
}

// 1. POST a number of items into table endpoint
#[post("/item/<item_id>/<quantity>/<table_num>")]
fn add_item(item_id: i64, quantity:i64, table_num: i64) -> Result<Json<StatusMessage>, String> {

    //generates random number from 5-15
    let t = rand::thread_rng().gen_range(5..16); 

    // Connects to the database
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    // Ensures that the table that waiter is serving exists in the database
    db_connection
            .execute(&format!("create table if not exists table_{} (
                item_id integer primary key,
                quantity integer not null,
                prep_time integer not null
            );", table_num), rusqlite::NO_PARAMS).unwrap();
    
    // Preparing the SQLite statement for table entry
    let mut statement =
        match db_connection.prepare(&format!("insert into table_{} (item_id, quantity, prep_time) values
        ({},{},{}) on conflict(item_id) do update set quantity = quantity + {};"
            , table_num, item_id, quantity, t, quantity)) {
                Ok(statement) => statement,
                Err(_) => return Err("Failed to prepare query".into()),
        };
    
    // Executing the SQLite statement
    let results = statement.execute(rusqlite::NO_PARAMS);

    // Matching the results from the SQLite execution to ensure API behaved as expected
    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} menu item inserted!", rows_affected),
        })),
        Err(_) => Err("Failed to insert menu item".into()),
    }
}

// 2. DEL an item from a table endpoint
#[delete("/item/<item_id>/<table_num>")]
fn delete_item(item_id: i64, table_num: i64) -> Result<Json<StatusMessage>, String> {

    // Connecting to the Database
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };
    
    // Preparing the SQlite statement
    let mut statement = match db_connection.prepare(&format!("delete from table_{} where item_id = {};", table_num, item_id)) {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    // Executing the SQlite statement
    let results = statement.execute(rusqlite::NO_PARAMS);

    // Matching results to ensure that Deletion statement did in fact execute as expected
    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} menu item deleted!", rows_affected),
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

    /*
    Rusqlite QueryMap function is a mapping function that first executes the SQlite statement. The rows that are 
    returned from the SQlite statement execution are then mapped over and row elements are inserted into a iterable
    */
    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(Item {
            item_id: row.get(0)?,
            quantity: row.get(1)?,
            prep_time: row.get(2)?,
        })
    });

    // Takes the iterable return type from the QueryMap function and stores it into a collection (Vector in this case)
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
#[get("/item/<table_num>/<item_id>")]
fn get_specific_item(table_num: i64, item_id: i64) -> Result<Json<ItemList>, String>{

    // Connecting to database
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    // Prepares SQL statement for the GET query
    let mut statement = match db_connection
    .prepare(&format!("select item_id, quantity, prep_time 
        from table_{} where item_id = {}", table_num, item_id)) {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
    };

    // Applies a mapping function over the SQL row and returns an iterable.
    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
    // Ok returns an Item struct with fields that have been parsed in from the query map iterable
    Ok(Item {
        item_id: row.get(0)?,
        quantity: row.get(1)?,
        prep_time: row.get(2)?,
    })
    });

    // the results variable above is a Result enum and needs to be matches for error handling
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

// 5. UPDATE quantity of some specified item 
#[put("/item/<table_num>/<item_id>/<new_quantity>")]
fn update_quantity(item_id: i64, table_num: i64, new_quantity: i64) -> Result<Json<StatusMessage>, String> {

    // Connecting to database
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    // Preparing SQLite statement
    let mut statement =
        match db_connection.prepare(&format!("update table_{} 
            set quantity = {}
            where item_id = {};", table_num, new_quantity, item_id)) {
                Ok(statement) => statement,
                Err(_) => return Err("Failed to prepare query".into()),
        };
    
    // Executing SQLite statement
    let results = statement.execute(rusqlite::NO_PARAMS);

    // Matching the result of the previous execution to ensure that the SQLite column has been updated
    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} row updated!", rows_affected),
        })),
        Err(_) => Err("Failed to update table".into()),
    }
}


fn main() {
    // Connects to database
    let _db_connection = Connection::open("data.sqlite").unwrap();

    // Starts the rocket web server (in dev mode by default)
    rocket::ignite()
    .mount(
        "/",
        routes![index, get_all_items, add_item, delete_item, get_specific_item, update_quantity]
    )
    .launch();
}

// Instantiates an instance of a Rocket HTTP Server for testing purposes
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/"
        , routes![index, get_all_items, add_item, delete_item, get_specific_item, update_quantity])
}

// Unit Testing the endpoints
#[cfg(test)]
mod test {
    use super::*;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_index() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Welcome!".into()));
    }
    #[test]
    fn test_get_all_items() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/item/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_specific_items() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/item/1/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}