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

    // Calculate the duration
    let turnaround_time = start.elapsed();

    // TODO: Get execution time from the server as metadata

    let population: i32 = response.get_ref().population;

    // Print the result
    println!("[INFO] getPopulationofCountry {} {}, Population {}, (turnaround time: {} ms, execution time:
XX ms, waiting time: XX ms, processed by Server 1)", country_name, zone, population, turnaround_time.as_millis());

    Ok(())
}

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

    // Calculate the duration
    let turnaround_time = start.elapsed();

    // TODO: Get execution time from the server as metadata

    let number_of_cities: i32 = response.get_ref().number_of_cities;

    // Print the result
    println!("[INFO] getNumberofCities for {} min: {}, Number of cities: {}, (turnaround time: {} ms, execution time:
XX ms, waiting time: XX ms, processed by Server 1)", country_name, min, number_of_cities, turnaround_time.as_millis());

    Ok(())
}

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

    // Calculate the duration
    let turnaround_time = start.elapsed();

    // TODO: Get execution time from the server as metadata

    let result: i32 = response.get_ref().result;

    // Print the result
    println!("[INFO] getNumberofCountries with citycount: {} min: {}, Result: {}, (turnaround time: {} ms, execution time:
XX ms, waiting time: XX ms, processed by Server 1)", citycount, min, result, turnaround_time.as_millis());

    Ok(())
}

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

    // Calculate the duration
    let turnaround_time = start.elapsed();

    // TODO: Get execution time from the server as metadata

    let result: i32 = response.get_ref().result;

    // Print the result
    println!("[INFO] getNumberofCountries with citycount: {} min: {}, max: {} Result: {}, (turnaround time: {} ms, execution time:
XX ms, waiting time: XX ms, processed by Server 1)", citycount, min, max, result, turnaround_time.as_millis());

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

        tokio::spawn(async move{
            // Send requests based on the different function types
            match func_name.as_str() {
                "getPopulationofCountry" => {
                    let _ =
                        create_client_and_get_population_of_country(client_zone, inputs, addr.to_string())
                            .await;
                }
                "getNumberofCities" => {
                    let _ = create_client_and_get_number_of_cities(client_zone, inputs, addr.to_string())
                        .await;
                }
                "getNumberofCountries" => {
                    let _ =
                        create_client_and_get_number_of_countries(client_zone, inputs, addr.to_string())
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
