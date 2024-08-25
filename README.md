# Rust Distributed Service Simulation - Statistics Service

The system consists of a distributed International Statistics Service. The Application functionality is provided by a remote object residing at the server side. Client objects interact with the server through remote method invocations. The client can invoke the methods defined in the server’s remote interface specification.

> ⚠️ This is a simulation created for educational purposes. There is still unimplemented features, but the goal is to explore complex distributed systems development, and implement performance-enhancing features.

![image](https://github.com/user-attachments/assets/76cc3c60-2608-4a33-b467-496bb956575d)


## About the simulation

The simulation models a distributed system implemented in Rust, running on a local machine. To mimic real-world conditions, the code introduces artificial delays to simulate network latency. A "zone" represents a geographical area, and requests between different zones experience higher latency. The primary goal of this simulation is to evaluate the performance of a distributed system, with plans for future performance enhancements.

### Key Features:

- **5 gRPC Servers:** Each server operates in a distinct geographical zone.
- **Client Distribution:** 1000 clients are assigned to each server, totaling 5000 clients across the simulation.
- **Request Flow:** Clients first attempt to connect to the server within their own zone.
    - **In-Zone Requests:** Clients experience an `80 ms` delay to simulate network latency within the same zone.
    - **Cross-Zone Requests:** If a client needs to connect to a server outside its zone, a `170 ms` delay is applied to simulate the increased latency.

The simulation is finished when all 5000 clients have completed their gRPC requests.

## Results 

The following is box-plots of the most important statistics from the simulation: 

![Screenshot from 2024-08-24 20-11-04](https://github.com/user-attachments/assets/75edfab8-de78-45b6-94ae-a8f740c2e092)

![Screenshot from 2024-08-24 20-11-47](https://github.com/user-attachments/assets/1f3c8280-6185-4c5c-b04c-237d7b1f0d1c)

![Screenshot from 2024-08-24 20-12-24](https://github.com/user-attachments/assets/794c386a-ddb3-419c-bac7-e999bc3ce901)

## Usage

To run a server with `server_id` 1: <br>
```terminal
cargo run --bin server 1
```

The client binary uses a file of requests to simulate different clients connecting and executing a request.
To run the client with `client_id` 1: <br>
```terminal
cargo run --bin client request_files/client_1.txt 1
```

## Resources

csv2sqlite - Python script to load CSV to SQLite: <br>
https://github.com/rufuspollock/csv2sqlite 

Dataset for the statistics: <br>
https://public.opendatasoft.com/explore/dataset/geonames-all-cities-with-a-population-1000/table/?disjunctive.cou_name_en&sort=name

tonic (gRPC package) server client tutorial: <br>
https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md 
