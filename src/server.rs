use rusqlite::Connection;


#[derive(Default, Debug)]
pub struct StatServer{
    pub sqlite_db: str, 
}

pub trait StatMethods {
    fn new() -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
    fn get_records_count(&self) -> Result<usize, ()>;
}

impl StatMethods for StatServer {
    fn new() -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        Ok(Self { sqlite_db: "db/city_database.db" })
    }
    
    fn get_records_count(&self) -> Result<usize, ()> {
        // Connect to the db
        let connection: Connection = Connection::open(&self.sqlite_db)?;

        // Query for counting 
        let query_statement = "SELECT COUNT(*) from cities";

        // Execute the query
        let res: usize = connection.query_row(&query_statement, [], |r| r.get(0)).unwrap();

        println!(res);
    }
}


pub fn start_server(){
    todo!()
}