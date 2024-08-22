use std::time::Instant;
use std::{env, fs::File, io::Read};

use stat_service::stat_methods_client::StatMethodsClient;
use stat_service::{
    Empty, NumberOfCitiesRequest, NumberOfCitiesResponse, NumberOfCountriesMaxRequest,
    NumberOfCountriesMaxResponse, NumberOfCountriesRequest, NumberOfCountriesResponse,
    PopulationRequest, PopulationResponse, RecordsResponse,
};
use tonic::metadata::MetadataValue;
use tonic::Response;

pub mod stat_service {
    tonic::include_proto!("statservice");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }
    let file_path = &args[1];

    let client_zone = 1;

    // Open the file asynchronously
    let mut file: File = File::open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Process the file contents
    println!("[INFO] Client read file: {}", file_path);

    // Connect to the server
    let addr = "127.0.0.1:50051";
    let mut client: StatMethodsClient<tonic::transport::Channel> =
        StatMethodsClient::connect(addr).await?;

    // Further processing can be done here
    for line in contents.lines() {
        let inputs: Vec<&str> = line.split(" ").into_iter().collect();
        if inputs.len() < 3 {
            println!("[ERR] Client found line with illegal values: {} ", line);
            continue;
        }
        let func_name = inputs[0];

        // Send requests based on the different function types
        match func_name {
            "getPopulationofCountry" => {
                // Get variables from the line
                assert!(inputs.len() == 3);
                let country_name = inputs[1];
                let zone = inputs[2].parse::<i32>()?;

                // Build the request to the server
                let mut request = tonic::Request::new(PopulationRequest {
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
                let response: Response<PopulationResponse> =
                    client.get_population_of_country(request).await?;

                // Calculate the duration
                let turnaround_time = start.elapsed();

                let population: i32 = response.get_ref().population;

                // Print the result
                println!("[INFO] getPopulationofCountry {} {}, Population {}, (turnaround time: {} ms, execution time:
XX ms, waiting time: XX ms, processed by Server 1)", country_name, zone, population, turnaround_time.as_millis())
            }
            "getNumberofCities" => {
                todo!()
            }
            "getNumberofCountries" => {
                todo!()
            }
            "getNumberofCountriesMax" => {
                todo!()
            }
            unknown => {
                println!("[ERROR] Unknown function name: {unknown}")
            }
        }
    }

    Ok(())
}
