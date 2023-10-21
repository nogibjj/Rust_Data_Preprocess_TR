# Week 8: Rewrite a Python Script in Rust
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

## **Comparison**
Python and Rust are get the correct median, which is 19.0. Here are their CPU usage, Memory usage, and operation time. 

- Python
CPU Usage: 15.1%
Memory Usage: 45.2%
Elapsed time: 174.3ms

- Rust
CPU Usage: 1.70%
Memory Usage: 45.65456%
Elapsed time: 454.6ms

Rust exhibits lower CPU usage compared to Python due to its compiled nature, fine-grained memory management, static typing, and efficient support for concurrency, which collectively result in optimized and performant code execution. Python, being an interpreted language with automatic memory management and a Global Interpreter Lock (GIL) in multi-threaded scenarios, tends to have higher CPU usage, particularly in CPU-bound tasks. However, the choice between Rust and Python should consider the specific requirements of the task, as Python's rich ecosystem and ease of use make it a valuable choice for many applications, while Rust excels in performance-critical and systems programming scenarios.

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)


