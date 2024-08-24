import pandas as pd
import matplotlib.pyplot as plt

# Number of clients to analyze
clients = 1

# Prepare the list of client files
client_files = [f"./log/client_data_z{i + 1}.csv" for i in range(clients)]
print(client_files)

# Read each client's data into a DataFrame
client_data = []
for file in client_files:
    try:
        df = pd.read_csv(file, header=None, names=["Turnaround Time", "Execution Time", "Waiting Time"])
        client_data.append(df)
    except Exception as e:
        print(f"Error reading {file}: {e}")

# Add a Client identifier to each DataFrame and keep track of original index
for i, df in enumerate(client_data):
    df["Client"] = f"Client {i+1}"
    df["Request Index"] = df.index  # Keep the original index for plotting

# Combine all DataFrames into a single DataFrame
all_data = pd.concat(client_data, ignore_index=True)

# Ensure data is correctly formatted
print(all_data.head())

# Plot the data
plt.figure(figsize=(14, 8))
for client in all_data["Client"].unique():
    subset = all_data[all_data["Client"] == client]
    plt.plot(subset["Request Index"].values, subset["Turnaround Time"].values, label=f"{client} - Turnaround Time")
    plt.plot(subset["Request Index"].values, subset["Execution Time"].values, label=f"{client} - Execution Time", linestyle="--")
    plt.plot(subset["Request Index"].values, subset["Waiting Time"].values, label=f"{client} - Waiting Time", linestyle=":")

plt.xlabel("Request Index")
plt.ylabel("Time (seconds)")
plt.title("Time Metrics Across Clients")
plt.legend(loc="upper left")
plt.show()