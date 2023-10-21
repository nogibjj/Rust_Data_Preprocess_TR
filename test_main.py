from mylib.lib import get_median
import pandas as pd
import time
import psutil


def test_get_median():
    # Record the start time
    start_time = time.time()

    example_csv = "Electric_Vehicle_Population_Data.csv"
    df = pd.read_csv(example_csv)
    print(get_median(df["Electric Range"]))

    assert get_median(df["Electric Range"]) == 19.0
    end_time = time.time()
    # Calculate the elapsed time
    elapsed_time = end_time - start_time

    cpu_percent = psutil.cpu_percent()
    memory_info = psutil.virtual_memory()

    print(f"Elapsed time: {elapsed_time:.4f} seconds")
    print(f"CPU Usage: {cpu_percent}%")
    print(f"Memory Usage: {memory_info.percent}%")


if __name__ == "__main__":
    test_get_median()
