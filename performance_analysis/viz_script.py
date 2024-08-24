import pandas as pd
import matplotlib.pyplot as plt

# Number of clients to analyze
clients = 5

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

# Combine all DataFrames into a single DataFrame
all_data = pd.concat(client_data, ignore_index=True)

# Ensure data is correctly formatted
print(all_data.head())

# Define plot functions
def plot_turnaround_time():
    plt.figure(figsize=(10, 6))
    all_data.boxplot(column="Turnaround Time", by="Client", grid=False, showfliers=False)
    plt.title('Turnaround Time by Client')
    plt.suptitle('')
    plt.xlabel('Client')
    plt.ylabel('Time (ms)')
    plt.show()

def plot_execution_time():
    plt.figure(figsize=(10, 6))
    all_data.boxplot(column="Execution Time", by="Client", grid=False, showfliers=False)
    plt.title('Execution Time by Client')
    plt.suptitle('')
    plt.xlabel('Client')
    plt.ylabel('Time (ms)')
    plt.show()

def plot_waiting_time():
    plt.figure(figsize=(10, 6))
    all_data.boxplot(column="Waiting Time", by="Client", grid=False, showfliers=False)
    plt.title('Waiting Time by Client')
    plt.suptitle('')
    plt.xlabel('Client')
    plt.ylabel('Time (ms)')
    plt.show()

# Main loop for switching between plots
def main():
   plot_turnaround_time()

# Run the main function
if __name__ == "__main__":
    main()
