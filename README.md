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


## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)


