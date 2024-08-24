use std::env;
use std::net::SocketAddr;
use std::time::Instant;

use rusqlite::Connection;
use stat_service::stat_methods_server::{StatMethods, StatMethodsServer};
use stat_service::{
    Empty, NumberOfCitiesRequest, NumberOfCitiesResponse, NumberOfCountriesMaxRequest,
    NumberOfCountriesMaxResponse, NumberOfCountriesRequest, NumberOfCountriesResponse,
    PopulationRequest, PopulationResponse, RecordsResponse,
};
use tonic::metadata::MetadataValue;

use tonic::Code;
use tonic::{transport::Server, Request, Response, Status};

pub mod stat_service {
    tonic::include_proto!("statservice");
}

#[derive(Debug, Default)]
pub struct StatServer {}

#[tonic::async_trait]
impl StatMethods for StatServer {
    async fn get_records_count(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<RecordsResponse>, Status> {
        // Logging request
        println!("[INFO] Request to count records..");

        let start = Instant::now();

        // Connect to the db or return error
        let connection = match Connection::open(&"db/city_database.db") {
            Ok(val) => val,
            Err(_) => {
                println!("[ERROR] Could not connect to SQLite DB");
                return Err(Status::new(tonic::Code::Internal, "Internal server error"));
            }
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

        let mut response = Response::new(RecordsResponse {
            records: record_count,
        });

        // Get the execution time
        let execution_ms = start.elapsed().as_millis() as u64;

        // Insert execution as metadata
        response
            .metadata_mut()
            .insert("execution", MetadataValue::from(execution_ms));

        Ok(response)
    }

    async fn get_population_of_country(
        &self,
        request: Request<PopulationRequest>,
    ) -> Result<Response<PopulationResponse>, Status> {
        // Logging request
        println!("[INFO] Request to get population of the given country");

        let start = Instant::now();

        // Connect to the db or return error
        let connection = match Connection::open(&"db/city_database.db") {
            Ok(val) => val,
            Err(_) => {
                println!("[ERROR] Could not connect to SQLite DB");
                return Err(Status::new(Code::Internal, "Internal server error"));
            }
        };

        // Retrieve country name from request
        let country_name = &request.get_ref().country;
        if country_name.is_empty() {
            println!("[ERROR] Given country was empty");
            return Err(Status::new(Code::InvalidArgument, "Empty country given"));
        }

        // Prepare the SQL query
        let query_statement = "SELECT SUM(Population) FROM cities WHERE [Country name EN] = ?1";

        // Execute the query
        let population_count: i32 =
            match connection.query_row(query_statement, [country_name], |r| r.get(0)) {
                Ok(count) => count,
                Err(_) => {
                    println!("[ERROR] Failed to execute query");
                    return Err(Status::new(tonic::Code::Internal, "Internal server error"));
                }
            };

        // Create a response object
        let mut response = Response::new(PopulationResponse {
            population: population_count,
        });

        // Get the execution time
        let execution_ms = start.elapsed().as_millis() as u64;

        // Insert execution as metadata
        response
            .metadata_mut()
            .insert("execution", MetadataValue::from(execution_ms));

        Ok(response)
    }

    async fn get_number_of_cities(
        &self,
        request: Request<NumberOfCitiesRequest>,
    ) -> Result<Response<NumberOfCitiesResponse>, Status> {
        // Logging request
        println!("[INFO] Request to get number of cities with a minimum population");

        let start = Instant::now();

        // Connect to the db or return error
        let connection = match Connection::open(&"db/city_database.db") {
            Ok(val) => val,
            Err(_) => {
                println!("[ERROR] Could not connect to SQLite DB");
                return Err(Status::new(Code::Internal, "Internal server error"));
            }
        };

        // Retrieve country name from request
        let country_name = &request.get_ref().country;
        if country_name.is_empty() {
            println!("[ERROR] Given country was empty");
            return Err(Status::new(Code::InvalidArgument, "Empty country given"));
        }

        // Retrieve minimum amount from request
        let min = &request.get_ref().min;

        // Prepare the SQL query
        let query_statement =
            "SELECT COUNT(*) FROM cities WHERE [Country name EN] = ?1 AND [Population] > ?2";

        // Execute the query
        let city_count: i32 =
            match connection.query_row(query_statement, [country_name, &min.to_string()], |r| {
                r.get(0)
            }) {
                Ok(count) => count,
                Err(_) => {
                    println!("[ERROR] Failed to execute query");
                    return Err(Status::new(Code::Internal, "Internal server error"));
                }
            };

        // Create response
        let mut response = Response::new(NumberOfCitiesResponse {
            number_of_cities: city_count,
        });

        // Get the execution time
        let execution_ms = start.elapsed().as_millis() as u64;

        // Insert execution as metadata
        response
            .metadata_mut()
            .insert("execution", MetadataValue::from(execution_ms));

        Ok(response)
    }

    async fn get_number_of_countries(
        &self,
        request: Request<NumberOfCountriesRequest>,
    ) -> Result<Response<NumberOfCountriesResponse>, Status> {
        println!("[INFO] Request to get number of countries with a minimum population");

        // Capture the start time
        let start = Instant::now();

        // Connect to the db or return error
        let connection = match Connection::open(&"db/city_database.db") {
            Ok(val) => val,
            Err(_) => {
                println!("[ERROR] Could not connect to SQLite DB");
                return Err(Status::new(Code::Internal, "Internal server error"));
            }
        };

        // Retrieve country name from request
        let citycount: &i32 = &request.get_ref().citycount;
        let min_population: &i32 = &request.get_ref().min;

        // No need to query if the request is not good
        if citycount <= &0 || min_population <= &0 {
            return Err(Status::new(Code::Internal, "Internal server error"));
        }

        // Query for collecting all
        let query = "SELECT COUNT(*) FROM (SELECT COUNT(*) as citycount, MIN([Population]) as min FROM cities GROUP BY [Country name EN] HAVING citycount > ?1 and min > ?2)";

        // Execute the query
        let result_count: i32 =
            match connection.query_row(&query, [citycount, min_population], |r| r.get(0)) {
                Ok(count) => count,
                Err(_) => {
                    println!("[ERROR] Failed to execute query");
                    return Err(Status::new(Code::Internal, "Internal server error"));
                }
            };

        // Create the response
        let mut response = Response::new(NumberOfCountriesResponse {
            result: result_count,
        });

        // Get the execution time
        let execution_ms = start.elapsed().as_millis() as u64;

        // Insert execution as metadata
        response
            .metadata_mut()
            .insert("execution", MetadataValue::from(execution_ms));

        // Return the response
        Ok(response)
    }

    async fn get_number_of_countries_max(
        &self,
        request: Request<NumberOfCountriesMaxRequest>,
    ) -> Result<Response<NumberOfCountriesMaxResponse>, Status> {
        println!("[INFO] Request to get number of countries with a minimum population");

        let start = Instant::now();

        // Connect to the db or return error
        let connection = match Connection::open(&"db/city_database.db") {
            Ok(val) => val,
            Err(_) => {
                println!("[ERROR] Could not connect to SQLite DB");
                return Err(Status::new(Code::Internal, "Internal server error"));
            }
        };

        // Retrieve country name from request
        let citycount: &i32 = &request.get_ref().citycount;
        let min_population: &i32 = &request.get_ref().min;
        let max_population: &i32 = &request.get_ref().max;

        // No need to query if the request is not good
        if citycount <= &0 || min_population <= &0 || max_population <= &0 {
            return Err(Status::new(Code::Internal, "Internal server error"));
        }

        // Query for collecting all
        let query = "SELECT COUNT(*) FROM (SELECT COUNT(*) as citycount, MIN([Population]) as min, MAX([Population]) as max FROM cities GROUP BY [Country name EN] HAVING citycount > ?1 and min > ?2 and max < ?3)";

        // Execute the query
        let result_count: i32 =
            match connection.query_row(&query, [citycount, min_population, max_population], |r| {
                r.get(0)
            }) {
                Ok(count) => count,
                Err(_) => {
                    println!("[ERROR] Failed to execute query");
                    return Err(Status::new(Code::Internal, "Internal server error"));
                }
            };

        let mut response = Response::new(NumberOfCountriesMaxResponse {
            result: result_count,
        });

        // Get the execution time
        let execution_ms = start.elapsed().as_millis() as u64;

        // Insert execution as metadata
        response
            .metadata_mut()
            .insert("execution", MetadataValue::from(execution_ms));

        Ok(response)
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <Server ID>", args[0]);
        return Ok(());
    }
    let server_id = &args[1].parse::<u32>()?;

    // Creating serer addr
    let addr = format!("127.0.0.1:5{}000", server_id);
    let server_addr = addr.parse::<SocketAddr>()?;

    // Server creation 
    let server: StatServer = StatServer::default();

    // Logging that the server has started
    println!("[INFO] Server started on {}", addr);

    Server::builder()
        .add_service(StatMethodsServer::new(server))
        .serve(server_addr)
        .await?;

    Ok(())
}
