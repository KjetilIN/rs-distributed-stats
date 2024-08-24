# Script for running the simulation 
echo "Starting servers"

# Build
run cargo build

# Run all five servers 
run cargo run --bin server
run cargo run --bin server
run cargo run --bin server
run cargo run --bin server

echo "Servers running.."
echo "Starting clients.."

# Run clients 
run cargo run --bin client request_files/client_1.txt 1
run cargo run --bin client request_files/client_2.txt 2
run cargo run --bin client request_files/client_3.txt 3
run cargo run --bin client request_files/client_4.txt 4
run cargo run --bin client request_files/client_5.txt 5

echo "Clients are running" 
