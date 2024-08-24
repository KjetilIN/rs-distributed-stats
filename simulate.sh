#!/bin/bash

echo "Starting servers"

# Build the project
cargo build > /dev/null 2>&1

# Run all five servers in the background
cargo run --bin server 1 &
SERVER1_PID=$!
cargo run --bin server 2 &
SERVER2_PID=$!
cargo run --bin server 3 &
SERVER3_PID=$!
cargo run --bin server 4 &
SERVER4_PID=$!
cargo run --bin server 5 &
SERVER5_PID=$!

echo "Servers running.."

# Wait a few seconds to ensure servers are fully up (adjust as needed)
sleep 5

echo "Starting clients.."

# Run clients in the background and silence their output
cargo run --bin client request_files/client_1.txt 1 > /dev/null 2>&1 &
echo "Client 1 started"
CLIENT1_PID=$!
cargo run --bin client request_files/client_2.txt 2 > /dev/null 2>&1 &
echo "Client 2 started"
CLIENT2_PID=$!
cargo run --bin client request_files/client_3.txt 3 > /dev/null 2>&1 &
echo "Client 3 started"
CLIENT3_PID=$!
cargo run --bin client request_files/client_2.txt 4 > /dev/null 2>&1 &
echo "Client 4 started"
CLIENT4_PID=$!
cargo run --bin client request_files/client_3.txt 5 > /dev/null 2>&1 &
echo "Client 5 started"
CLIENT5_PID=$!

echo "Clients are running"

# Wait for all client processes to finish
wait $CLIENT1_PID
echo "Client 1 finished"
wait $CLIENT2_PID
echo "Client 2 finished"
wait $CLIENT3_PID
echo "Client 3 finished"
wait $CLIENT4_PID
echo "Client 4 finished"
wait $CLIENT5_PID
echo "Client 5 finished"

echo "All clients have finished"
echo "Stopping servers"

# Kill all server processes (if needed)
kill $SERVER1_PID
kill $SERVER2_PID
kill $SERVER3_PID
kill $SERVER4_PID
kill $SERVER5_PID

echo "Servers stopped"
