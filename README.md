# Week 8: Rewrite a Python Script in Rust

[![Rust CI/CD](https://github.com/nogibjj/Rust_Data_Preprocess_TR/actions/workflows/RustCI.yml/badge.svg)](https://github.com/nogibjj/Rust_Data_Preprocess_TR/actions/workflows/RustCI.yml)

[![Python CI/CD](https://github.com/nogibjj/Rust_Data_Preprocess_TR/actions/workflows/PythonCI.yml/badge.svg)](https://github.com/nogibjj/Rust_Data_Preprocess_TR/actions/workflows/PythonCI.yml)


## Overview
In the project, we take an existing Python script for data processing, which can get the median of a dataset, and rewrite it in Rust. Here, we also record their run times and highlight improvements in speed and resource usage. 

## Rust
### Introduction
The code consists of two Rust source files, lib.rs and main.rs, designed for data analysis and performance monitoring. In lib.rs, there is a function called calculate_median that calculates the median of a vector of floating-point values. It first clones the input vector, sorts the clone, and then calculates the median based on the sorted values. In main.rs, the code focuses on monitoring CPU usage, reading data from a CSV file, calculating the electric vehicle range median using the calculate_median function from lib.rs, and tracking memory usage and elapsed time. It leverages external crates like csv for CSV parsing and sys_info for memory information. The ps command is used to obtain the CPU usage of the current process, and the entire program provides a comprehensive example of utilizing Rust for data processing while monitoring system resource utilization. There is also a small error in the printed message, "ELetirc Range Median," which should be corrected to "Electric Range Median" for clarity.

### Preparation 
1. create a codespace 
2. build: using **cargo build** 
3. run: **cargo run**

### lint format, and test
1. format: make format 
2. lint: make lint 
3. test: make test 

![Screenshot 2023-10-21 at 10 38 19 PM](https://github.com/nogibjj/Rust_Data_Preprocess_TR/assets/104114843/86a358f0-3c09-4fef-afd3-2bed224cdb76)

## Python
### Introduction
The code serves the function of testing the correctness and performance of a Python function, median_1, which presumably calculates the median of a specific column in a CSV dataset. Its purpose is to validate that the calculated median matches an expected value (19.0) and to record essential performance metrics such as elapsed time, CPU usage, and memory usage. This testing framework helps ensure the accuracy and efficiency of the median_1 function when applied to real data, making it a valuable tool for verifying the reliability and resource utilization of this data analysis function in a broader project context.

### Preparation
1. create a codespace 
2. install required packages: make python_install 
3. run: python main.py

### Lint, format, and test
1. format code make python_format
2. lint code make python_lint
3. test coce make python_test

![Screenshot 2023-10-21 at 10 41 52 PM](https://github.com/nogibjj/Rust_Data_Preprocess_TR/assets/104114843/4cf192c0-18a2-4f7e-91ad-30b5f8bcb29f)


## **Comparison**
Python and Rust are get the correct median, which is 19.0. Here are their CPU usage, Memory usage, and operation time. 

- Python
CPU Usage: 36.9%
Memory Usage: 45.7%
Elapsed time: 192.3ms
![Screenshot 2023-10-21 at 10 44 57 PM](https://github.com/nogibjj/Rust_Data_Preprocess_TR/assets/104114843/8e751ebf-ac23-4aff-927f-1db3124e9b5a)

- Rust
CPU Usage:  0.80%
Memory Usage: 45.394943%
Elapsed time: 34.070125ms
![Screenshot 2023-10-21 at 10 42 25 PM](https://github.com/nogibjj/Rust_Data_Preprocess_TR/assets/104114843/694042eb-1545-4fdd-9b2b-cdd50c3c2e38)

The observed performance differences between Python and Rust can be attributed to their fundamental design and implementation characteristics. Python is an interpreted language known for its ease of use but tends to be slower due to its dynamic typing and interpreted nature, which results in higher CPU and elapsed time usage. In contrast, Rust is a compiled systems programming language designed for performance and safety, which is reflected in its minimal CPU and elapsed time usage. Rust's strong typing, memory management control, and optimizations at compile time contribute to its efficient resource usage and shorter execution times, making it a better choice for tasks that require high performance and resource efficiency.


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)


