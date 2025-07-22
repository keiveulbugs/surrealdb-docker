use ::{once_cell::sync::Lazy, surrealdb::Surreal, surrealdb::engine::any};
static DB: Lazy<Surreal<any::Any>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() {
    dbg!("Starting program to test SurrealDB connection");
    match DB.connect("ws://127.0.0.1:8000/").await {
        Ok(val) => {
            dbg!("Succesfully started a connection with SurrealDB");
            val
        }

        Err(dbconnecterror) => {
            panic!("failed to start connection to the database with error:\n {dbconnecterror}")
        }
    };

    match DB
        .signin(surrealdb::opt::auth::Root {
            username: "root",
            password: "root",
        })
        .await
    {
        Ok(val) => {
            dbg!("Succesfully logged in as Root into the database");
            val
        }
        Err(dberror) => panic!("failed to login into the database: {dberror}"),
    };

    match DB.use_ns("testns").use_db("testdb").await {
        Ok(val) => {
            dbg!("Succesfully created a namespace and database");
            val
        }
        Err(dberror) => panic!("failed to use namespace or datebase: {dberror}"),
    };

    let result_of_query = DB
        .version()
        .await
        .expect("Failed to get the database version");
    dbg!("Version of database running:", result_of_query);
}
