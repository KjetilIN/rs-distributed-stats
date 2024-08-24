# Rust Distributed Service Simulation - Statistics Service

The system consists of a distributed International Statistics Service. The Application functionality is provided by a remote object residing at the server side. Client objects interact with the server through remote method invocations. The client can invoke the methods defined in the server’s remote interface specification.

> ⚠️ This is a simulation created for educational purposes. There is still unimplemented features, but the goal is to explore complex distributed systems development, and implement features that will improve the performance.

![image](https://github.com/user-attachments/assets/76cc3c60-2608-4a33-b467-496bb956575d)


## About the simulation

The image above illustrates the simulation. The simulation will be ran on a local machine, and therefore the code contains waits to simulate the network latency. A zone represent a geographical zone. 
A request from one zone to another will therefore take more time. The goal of the simulation is to implement a distributed system with Rust, and see the performance of the system. More performance features will be added. 

- 5 servers in 5 different zones.
- Each server is placed in a zone.
- Each zone has a set of clients.
- For each server, 1000 clients will connect to each server and do a gRPC request.
- Simulation is done when all 5000 clients has sent their requests.

Each client request has a designated "zone" they want to reach. Originally they try to request to their own server in their own zone. 
- If the client is in the correct zone, the client will wait `80 ms` (to simulate network latency).
- If the client is not in the correct zone, the client will wait `170 ms` (to simulate a request to a different zone). 


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
