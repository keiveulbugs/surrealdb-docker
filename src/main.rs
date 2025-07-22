use ::{once_cell::sync::Lazy, surrealdb::Surreal, surrealdb::engine::any};
static DB: Lazy<Surreal<any::Any>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() {
    dbg!("Starting program to test SurrealDB connection");
    let remoteaddress = match std::env::var("SURREAL_BIND") {
        Ok(val) => {
            if !val.starts_with(|x: char| x.is_ascii_digit()) {
                val
            } else {
                format!("ws://{val}")
            }
        }
        Err(_) => "ws://127.0.0.1:8000".to_string(),
    };

    dbg!(
        "Using the following address to connect to database:",
        &remoteaddress
    );
    match DB.connect(remoteaddress).await {
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
