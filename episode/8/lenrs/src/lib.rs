#![feature(proc_macro)]
#![feature(proc_macro_path_invoc)]

extern crate pyo3;

use pyo3::prelude::*;

#[py::modinit(_lenrs)]
fn init(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "len")]
    fn len(py: Python, obj: PyObject) -> PyResult<PyObject> {
        if let Ok(s) = obj.extract::<String>(py) {
            return Ok(s.len().to_object(py));
        }
        if let Ok(v) = obj.extract::<Vec<String>>(py) {
            return Ok(v.len().to_object(py));
        }
        Err(PyErr::new::<pyo3::exc::TypeError, _>("Not supported"))
    }

    Ok(())
}