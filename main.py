from mylib.lib import get_median
import time
import psutil
import pandas as pd

lOG_FILE = "python_times.md"


def log_encrypt(runtime):
    """adds to a query markdown file"""

    with open(lOG_FILE, "a") as file:
        file.write(f"\nThe running time is {runtime}\n\n\n")


def median_1():
    start_time = time.time()

    example_csv = "Electric_Vehicle_Population_Data.csv"
    df = pd.read_csv(example_csv)
    print(get_median(df["Electric Range"]))

    end_time = time.time()
    # Calculate the elapsed time
    elapsed_time = end_time - start_time

    cpu_percent = psutil.cpu_percent()
    memory_info = psutil.virtual_memory()

    print(f"Elapsed time: {elapsed_time:.4f} seconds")
    print(f"CPU Usage: {cpu_percent}%")
    print(f"Memory Usage: {memory_info.percent}%")

    log_encrypt(elapsed_time)

    return get_median(df["Electric Range"])


if __name__ == "__main__":
    median_1()
