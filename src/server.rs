use std::net::SocketAddr;

use rusqlite::Connection;

use stat_service::stat_methods_server::{StatMethods, StatMethodsServer};
use stat_service::{Empty, RecordsResponse};

use tonic::{transport::Server, Request, Response, Status};

pub mod stat_service {
    tonic::include_proto!("statservice"); 
}

#[derive(Debug, Default)]
pub struct StatServer{}

#[tonic::async_trait]
impl StatMethods for StatServer {
    async fn get_records_count(&self, _: Request<Empty>) -> Result<Response<RecordsResponse>, Status> {
        // Logging request
        println!("[INFO] Request to count records..");

        // Connect to the db or return error
        let connection = match Connection::open(&"db/city_database.db"){
            Ok(val) => val,
            Err(_) => {
                println!("[ERROR] Could not connect to SQLite DB");
                return Err(Status::new(tonic::Code::from_i32(500), "Internal server error"))
            },
        };

        // Query for counting 
        let query_statement = "SELECT COUNT(*) from cities";

        // Execute the query
        let record_count: i32 = match connection.query_row(&query_statement, [], |r| r.get(0)) {
            Ok(count) => count,
            Err(_) => {
                println!("[ERROR] Failed to execute query");
                return Err(Status::internal("Internal server error"));
            }
        };

        let response = RecordsResponse{
            records: record_count,
        }; 

        Ok(Response::new(response))
    }
}


#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse::<SocketAddr>()?;

    let server: StatServer = StatServer::default(); 

    // Logging that the server has started 
    println!("[INFO] Server started on {}", addr);

    Server::builder()
        .add_service(StatMethodsServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}