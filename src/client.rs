use csv::WriterBuilder;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;
use std::{env, fs::File, io::Read};

use stat_service::stat_methods_client::StatMethodsClient;
use stat_service::{
    NumberOfCitiesRequest, NumberOfCitiesResponse, NumberOfCountriesMaxRequest,
    NumberOfCountriesMaxResponse, NumberOfCountriesRequest, NumberOfCountriesResponse,
    PopulationRequest, PopulationResponse,
};
use tokio::sync::Semaphore;
use tonic::metadata::MetadataValue;
use tonic::{Request, Response, Status};

pub mod stat_service {
    tonic::include_proto!("statservice");
}

/// Create a connection to given server and sends request.
///
/// Sends a gRPC request for getting the population of a given country.
/// Needs the zone number of the client, vec of inputs and the server address with (with protocol).
/// Should be used in a thread. Does not crash or panic the program.
async fn create_client_and_get_population_of_country(
    client_zone: i32,
    inputs: Vec<String>,
    server_addr: String,
) -> Result<(), Status> {
    // Connect to server:
    let mut client = match StatMethodsClient::connect(server_addr).await {
        Ok(client) => client,
        Err(e) => {
            println!("[ERROR] Failed to connect to server: {}", e);
            return Err(Status::internal("Failed to connect to server"));
        }
    };
    // Get variables from the line
    assert!(inputs.len() == 3);
    let country_name = inputs[1].clone();
    let zone = inputs[2].chars().last().unwrap().to_digit(10).unwrap();

    // Build the request to the server
    let mut request = Request::new(PopulationRequest {
        country: country_name.to_string(),
    });

    // Set zone data as meta data in the request
    request
        .metadata_mut()
        .insert("client_zone", MetadataValue::from(client_zone));

    request
        .metadata_mut()
        .insert("request_zone", MetadataValue::from(zone));

    // Capture the start time
    let start = Instant::now();

    // Get the response
    let response: Response<PopulationResponse> = client.get_population_of_country(request).await?;

    // Calculate turn around, execution and wait time
    let turnaround_time = start.elapsed();
    let execution_ms = response
        .metadata()
        .get("execution")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let waiting_ms: u64 = turnaround_time.as_millis() as u64 - execution_ms;

    let population: i32 = response.get_ref().population;

    let write_res = write_client_log(
        &turnaround_time.as_millis(),
        &execution_ms,
        &waiting_ms,
        &client_zone,
    )
    .await;
    if !write_res.is_ok() {
        println!("[ERROR] Was not able to write to file");
        return Err(Status::internal("Unable to write to client file"));
    }

    // Print the result
    println!("[INFO] getPopulationofCountry {} {}, Population {}, (turnaround time: {} ms, execution time:
{} ms, waiting time: {} ms, processed by Server 1)", country_name, zone, population, turnaround_time.as_millis(), execution_ms, waiting_ms);

    Ok(())
}

/// Create a connection to given server and sends request.
///
/// Sends a gRPC request for getting the number of cities within a country where each city has at least the given amount of population.
/// Needs the zone number of the client, vec of inputs and the server address with (with protocol).
/// Should be used in a thread. Does not crash or panic the program.
async fn create_client_and_get_number_of_cities(
    client_zone: i32,
    inputs: Vec<String>,
    server_addr: String,
) -> Result<(), Status> {
    // Connect to server:
    let mut client = match StatMethodsClient::connect(server_addr).await {
        Ok(client) => client,
        Err(e) => {
            println!("[ERROR] Failed to connect to server: {}", e);
            return Err(Status::internal("Failed to connect to server"));
        }
    };

    // Get variables
    assert!(inputs.len() == 4);
    let country_name = inputs[1].clone();
    let min = match inputs[2].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("[ERROR] Failed to parse min variable: {}", inputs[2]);
            return Err(Status::internal("Failed to parse error"));
        }
    };
    let zone = inputs[3].chars().last().unwrap().to_digit(10).unwrap();

    // Build the request to the server
    let mut request = Request::new(NumberOfCitiesRequest {
        country: country_name.to_string(),
        min,
    });

    // Set zone data as meta data in the request
    request
        .metadata_mut()
        .insert("client_zone", MetadataValue::from(client_zone));

    request
        .metadata_mut()
        .insert("request_zone", MetadataValue::from(zone));

    // Capture the start time
    let start = Instant::now();

    // Get the response
    let response: Response<NumberOfCitiesResponse> = client.get_number_of_cities(request).await?;

    // Calculate turn around, execution and wait time
    let turnaround_time = start.elapsed();
    let execution_ms = response
        .metadata()
        .get("execution")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let waiting_ms: u64 = turnaround_time.as_millis() as u64 - execution_ms;

    let number_of_cities: i32 = response.get_ref().number_of_cities;

    // Write to the clients log.
    let write_res = write_client_log(
        &turnaround_time.as_millis(),
        &execution_ms,
        &waiting_ms,
        &client_zone,
    )
    .await;

    if !write_res.is_ok() {
        println!("[ERROR] Was not able to write to file");
        return Err(Status::internal("Unable to write to client file"));
    }

    // Print the result
    println!("[INFO] getNumberofCities for {} min: {}, Number of cities: {}, (turnaround time: {} ms, execution time:
{} ms, waiting time: {} ms, processed by Server 1)", country_name, min, number_of_cities, turnaround_time.as_millis(), execution_ms, waiting_ms);

    Ok(())
}

/// Create a connection to given server and sends request.
///
/// Sends a gRPC request for getting the number of countries that has the given amount of cities where each city has a given minimum population.
/// Needs the zone number of the client, vec of inputs and the server address with (with protocol).
/// Should be used in a thread. Does not crash or panic the program.
async fn create_client_and_get_number_of_countries(
    client_zone: i32,
    inputs: Vec<String>,
    server_addr: String,
) -> Result<(), Status> {
    // Connect to server:
    let mut client = match StatMethodsClient::connect(server_addr).await {
        Ok(client) => client,
        Err(e) => {
            println!("[ERROR] Failed to connect to server: {}", e);
            return Err(Status::internal("Failed to connect to server"));
        }
    };

    // Get variables
    assert!(inputs.len() == 4);
    let citycount = match inputs[1].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("[ERROR] Failed to parse min variable: {}", inputs[1]);
            return Err(Status::internal("Failed to parse error"));
        }
    };
    let min = match inputs[2].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("[ERROR] Failed to parse min variable: {}", inputs[2]);
            return Err(Status::internal("Failed to parse error"));
        }
    };
    let zone = inputs[3].chars().last().unwrap().to_digit(10).unwrap();

    let mut request = Request::new(NumberOfCountriesRequest { citycount, min });

    // Set zone data as meta data in the request
    request
        .metadata_mut()
        .insert("client_zone", MetadataValue::from(client_zone));

    request
        .metadata_mut()
        .insert("request_zone", MetadataValue::from(zone));

    // Capture the start time
    let start = Instant::now();

    // Get the response
    let response: Response<NumberOfCountriesResponse> =
        client.get_number_of_countries(request).await?;

    // Calculate turn around, execution and wait time
    let turnaround_time = start.elapsed();
    let execution_ms = response
        .metadata()
        .get("execution")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let waiting_ms: u64 = turnaround_time.as_millis() as u64 - execution_ms;

    let result: i32 = response.get_ref().result;

    let write_res = write_client_log(
        &turnaround_time.as_millis(),
        &execution_ms,
        &waiting_ms,
        &client_zone,
    )
    .await;
    if !write_res.is_ok() {
        println!("[ERROR] Was not able to write to file");
        return Err(Status::internal("Unable to write to client file"));
    }

    // Print the result
    println!("[INFO] getNumberofCountries with citycount: {} min: {}, Result: {}, (turnaround time: {} ms, execution time:
{} ms, waiting time: {} ms, processed by Server 1)", citycount, min, result, turnaround_time.as_millis(), execution_ms, waiting_ms);

    Ok(())
}

/// Create a connection to given server and sends request.
///
/// Sends a gRPC request for getting the number of countries that has the given amount of cities where each city has a given minimum population and less than a given maximum population.
/// Needs the zone number of the client, vec of inputs and the server address with (with protocol).
/// Should be used in a thread. Does not crash or panic the program.
async fn create_client_and_get_number_of_countries_max(
    client_zone: i32,
    inputs: Vec<String>,
    server_addr: String,
) -> Result<(), Status> {
    // Connect to server:
    let mut client = match StatMethodsClient::connect(server_addr).await {
        Ok(client) => client,
        Err(e) => {
            println!("[ERROR] Failed to connect to server: {}", e);
            return Err(Status::internal("Failed to connect to server"));
        }
    };

    // Get variables
    assert!(inputs.len() == 5);
    let citycount = match inputs[1].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("[ERROR] Failed to parse min variable: {}", inputs[1]);
            return Err(Status::internal("Failed to parse error"));
        }
    };

    let min = match inputs[2].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("[ERROR] Failed to parse min variable: {}", inputs[2]);
            return Err(Status::internal("Failed to parse error"));
        }
    };

    let max = match inputs[3].parse::<i32>() {
        Ok(val) => val,
        Err(_) => {
            println!("[ERROR] Failed to parse min variable: {}", inputs[3]);
            return Err(Status::internal("Failed to parse error"));
        }
    };

    let zone = inputs[4].chars().last().unwrap().to_digit(10).unwrap();

    let mut request = Request::new(NumberOfCountriesMaxRequest {
        citycount,
        min,
        max,
    });

    // Set zone data as meta data in the request
    request
        .metadata_mut()
        .insert("client_zone", MetadataValue::from(client_zone));

    request
        .metadata_mut()
        .insert("request_zone", MetadataValue::from(zone));

    // Capture the start time
    let start = Instant::now();

    // Get the response
    let response: Response<NumberOfCountriesMaxResponse> =
        client.get_number_of_countries_max(request).await?;

    // Calculate turn around, execution and wait time
    let turnaround_time = start.elapsed();
    let execution_ms = response
        .metadata()
        .get("execution")
        .unwrap()
        .to_str()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let waiting_ms: u64 = turnaround_time.as_millis() as u64 - execution_ms;

    let result: i32 = response.get_ref().result;

    let write_res = write_client_log(
        &turnaround_time.as_millis(),
        &execution_ms,
        &waiting_ms,
        &client_zone,
    )
    .await;

    if !write_res.is_ok() {
        println!("[ERROR] Was not able to write to file");
        return Err(Status::internal("Unable to write to client file"));
    }

    // Print the result
    println!("[INFO] getNumberofCountries with citycount: {} min: {}, max: {} Result: {}, (turnaround time: {} ms, execution time:
{} ms, waiting time: {} ms, processed by Server 1)", citycount, min, max, result, turnaround_time.as_millis(), execution_ms, waiting_ms);

    Ok(())
}

/// Write most important statistics to a log file.
///
/// Data such as turn around time, execution and waiting is written to the log file. Also the zone from where the client came from.
/// The data is written to `/log/client_data_z<ZONE>.csv`.
async fn write_client_log(
    turn_around_ms: &u128,
    execution_ms: &u64,
    waiting_ms: &u64,
    client_zone: &i32,
) -> Result<(), Box<dyn Error>> {
    // Build the file path based on the client zone
    let log_dir = "log";
    let file_name = format!("{}/client_data_z{}.csv", log_dir, client_zone);
    let path = Path::new(&file_name);

    // Create the directory if it does not exist
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    // Open or create the file
    let file: File = OpenOptions::new()
        .create(true) // Create the file if it does not exist
        .append(true) // Open the file in append mode
        .open(&file_name)?;

    // Create a CSV writer
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);

    // Write the record to the CSV file
    wtr.write_record(&[
        turn_around_ms.to_string(),
        execution_ms.to_string(),
        waiting_ms.to_string(),
    ])?;

    // Ensure all data is written to disk
    wtr.flush()?;

    Ok(())
}

/// Clean a log file for given client.
///
/// Called before writing to the log file.
/// Cleans the file with the following name: `./log/client_data_z<ZONE>.csv`.
async fn clean_client_log(client_zone: &i32) -> Result<(), Box<dyn Error>> {
    // Construct the file path based on the client zone
    let file_path = format!("./log/client_data_z{}.csv", client_zone);

    // Ensure the directory exists before attempting to open the file
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err("Could not open file".into());
    }

    // Open the file in write mode, which will truncate it
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)?;

    // Write an empty string to truncate the file
    file.write_all(b"")?;

    // Flush the changes to the file
    file.flush()?;

    Ok(())
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <file_path> <client_zone>", args[0]);
        return Ok(());
    }
    let file_path = &args[1];

    // Get the zone of the client
    let client_zone = args[2].parse::<i32>()?.clone();

    // Open the file asynchronously
    let mut file: File = File::open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let contents = Arc::new(contents);

    // Clean the log file
    clean_client_log(&client_zone).await?;

    // Process the file contents
    println!(
        "[INFO] Client (ZONE:{}) started with file: {}",
        client_zone, file_path
    );

    // Connect to the server
    let addr = "http://127.0.0.1:50051";

    // Create X amount of threads to simulate new clients connecting and doing a task
    // Semaphore is created with a limited amount of permits allowed
    let semaphore = Arc::new(Semaphore::new(10));

    let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    for line in lines {
        let inputs: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        if inputs.len() < 3 {
            println!("[ERR] Client found line with illegal values: {} ", line);
            continue;
        }
        let func_name = inputs[0].clone();
        let permit = semaphore.clone().acquire_owned().await.unwrap();

        tokio::spawn(async move {
            // Send requests based on the different function types
            match func_name.as_str() {
                "getPopulationofCountry" => {
                    let _ = create_client_and_get_population_of_country(
                        client_zone,
                        inputs,
                        addr.to_string(),
                    )
                    .await;
                }
                "getNumberofCities" => {
                    let _ = create_client_and_get_number_of_cities(
                        client_zone,
                        inputs,
                        addr.to_string(),
                    )
                    .await;
                }
                "getNumberofCountries" => {
                    let _ = create_client_and_get_number_of_countries(
                        client_zone,
                        inputs,
                        addr.to_string(),
                    )
                    .await;
                }
                "getNumberofCountriesMax" => {
                    let _ = create_client_and_get_number_of_countries_max(
                        client_zone,
                        inputs,
                        addr.to_string(),
                    )
                    .await;
                }
                unknown => {
                    println!("[ERROR] Unknown function name: {unknown}")
                }
            }

            // Drop the permit
            drop(permit);
        });
    }

    Ok(())
}
