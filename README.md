# Week 8: Rewrite a Python Script in Rust
## Overview
In the project, we take an existing Python script for data processing, which can get the median of a dataset, and rewrite it in Rust. Here, we also record their run times and highlight improvements in speed and resource usage. 

## Rust
### Preparation 
1. create a codespace 
2. build: using **cargo build** 
3. run: **cargo run**

### lint format, and test
1. format: make format 
2. lint: make lint 
3. test: make test 

## Python
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


