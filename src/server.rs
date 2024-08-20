use std::{error::Error, net::SocketAddr};

use rusqlite::Connection;
use stat_service::{Empty, RecordsResponse};
use tonic::{transport::Server, Request, Response, Status};

pub mod stat_service {
    tonic::include_proto!("statservice"); 
}

#[derive(Debug, Clone)]
pub struct StatServer{
    pub sqlite_db: String, 
}

impl StatServer {
    fn new() -> Self{
        Self { sqlite_db: "db/city_database.db".to_string() }
    }
}

#[tonic::async_trait]
pub trait GrpcStatServer {
    async fn get_records_count(&self, request: Request<Empty>) -> Result<Response<RecordsResponse>, Status>;
}


impl GrpcStatServer for StatServer {
    async fn get_records_count(&self, request: Request<Empty>) -> Result<Response<RecordsResponse>, Status> {
        // Connect to the db or return error
        let connection = match Connection::open(&self.sqlite_db){
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


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse::<SocketAddr>()?;

    let server: StatServer = StatServer::new(); 

    Server::builder()
        .add_service(GrpcStatServer::new())
        .serve(addr)
        .await?;

    Ok(())
}