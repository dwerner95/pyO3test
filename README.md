# pyO3test
Test a pure rust program including a [HDF5](https://github.com/aldanor/hdf5-rust) test in python using [pyO3](https://github.com/PyO3/pyo3)
## Requirements
- Rust
- HDF5 (Test hdf5-rust first)

## Installation
In the main folder, make sure you have setuptools_rust installed.

`python3 -m pip install setuptools_rust`

then simply install the library with:

`python3 -m pip install . `


## Usage
for whatever reason th "o" in pyO3 here is a zero:
```python3
import py03test
m = py03test.PyData()
m.run()
```


