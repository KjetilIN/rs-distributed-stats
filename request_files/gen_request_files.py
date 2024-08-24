import random

# Define possible function names
functions = [
    "getPopulationofCountry",
    "getNumberofCities",
    "getNumberofCountries",
    "getNumberofCountriesMax"
]

# Define possible country names
countries = ["Netherlands", "Norway", "Sweden", "Switzerland", "Germany", "Canada", "Australia", "Belgium", "Israel", "Egypt"]

# Function to generate a random zone
def get_random_zone():
    return f"Zone:{random.randint(1, 5)}"

# Function to generate a random number
def get_random_number():
    return str(random.randint(100000, 9999999))

# Function to generate a line based on the function type
def generate_line():
    func = random.choice(functions)
    
    if func == "getPopulationofCountry":
        country = random.choice(countries)
        return f"{func} {country} {get_random_zone()}"
    
    elif func == "getNumberofCities":
        country = random.choice(countries)
        number = get_random_number()
        return f"{func} {country} {number} {get_random_zone()}"
    
    elif func == "getNumberofCountries":
        number1 = random.randint(1, 10)
        number2 = get_random_number()
        return f"{func} {number1} {number2} {get_random_zone()}"
    
    elif func == "getNumberofCountriesMax":
        number1 = random.randint(1, 10)
        number2 = get_random_number()
        number3 = get_random_number()
        return f"{func} {number1} {number2} {number3} {get_random_zone()}"

# Function to generate the file with random lines
def generate_file(filename, num_lines):
    with open(filename, 'w') as f:
        for _ in range(num_lines):
            line = generate_line()
            f.write(line + '\n')

# Generate a files
generate_file('./request_files/client_1.txt', 1000)
generate_file('./request_files/client_2.txt', 1000)
generate_file('./request_files/client_3.txt', 1000)
generate_file('./request_files/client_4.txt', 1000)
generate_file('./request_files/client_5.txt', 1000)
