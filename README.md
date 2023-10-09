# DESIGN PYTHON LIBRARIES IN RUST USING MATURIN AND PYO3  

1. Install rust

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustc --version
    
    ```

2. Create virtual env using conda or pyenv

    ```sh
    conda create -n rustpy python=3.11
    conda activate rustpy
    ```

3. Install `maturin` from pypi  

    ```sh
    pip install maturin
    ```

4. Create new project and select `pyo3` bindings

    ```sh
    maturin new <project_name>
    cd <project_name>
    ```  

5. Update pyo3 in `Cargo.toml` if needed and design your library in `src/lib.rs`  

    ```sh
    cargo search pyo3
    ```

   here is a `lib.rs` example  
  
    ```rust
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_in_rust(a: f64, b: f64) -> f64 {
        a + b
    }

    /// A Python module implemented in Rust.
    #[pymodule]
    fn module(py: Python<'_>, m: &PyModule) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(sum_in_rust, m)?)?;
        Ok(())
    }
    ```

   note that the `pymodule` name must be the same in `<ptoject_name.toml>` and in `Cargo.toml` (name = <`pymodule name`>)  

6. Compile  

    compile without install package. Typing:

     ```sh
     maturin build --release
     ```  

    find .whl in `/target/wheels`  

     compile and install package. Typing:

     ```sh
     maturin build --release
     ```  

7. create python project and test package

     ```sh
     touch <python_project_name.py>
     ```

    import the module created in the python script, and use it:

     ```python
     import <module_name>

     result = <module_name.<functionname>>
     ```  

**References:**  

- `pyo3` documentarion, <https://docs.rs/pyo3/latest/pyo3/>  
- `maturin` user guide, <https://www.maturin.rs/>
