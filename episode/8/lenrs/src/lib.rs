#![feature(proc_macro)]
#![feature(proc_macro_path_invoc)]

#[macro_use]
extern crate pyo3;

use pyo3::prelude::*;

#[py::modinit(_lenrs)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "len")]
    fn len(py: Python, s: &str) -> PyResult<PyObject> {
        Ok(s.len().to_object(py))
    }
    Ok(())
}
