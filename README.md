# Rust Distributed Service - Statistics Service

The system consists of a distributed International Statistics Service. The Application functionality is provided by a remote object residing at the server side. Client objects interact with the server through remote method invocations. The client can invoke the methods defined in the server’s remote interface specification.

![image](https://github.com/user-attachments/assets/76cc3c60-2608-4a33-b467-496bb956575d)

## Features

- Proxy (load balancing) Server
- gRPC Server for remote method execution 
- SQLite database for all data

## Usage

TODO: write this

1. cargo run

## Resources

csv2sqlite - Python script to load CSV to SQLite: <br>
https://github.com/rufuspollock/csv2sqlite 

Dataset for the statistics: <br>
https://public.opendatasoft.com/explore/dataset/geonames-all-cities-with-a-population-1000/table/?disjunctive.cou_name_en&sort=name

