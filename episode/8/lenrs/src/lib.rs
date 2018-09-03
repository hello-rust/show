extern crate pyo3;

use pyo3::prelude::*;

#[pymodinit]
fn init(_py: Python, m: &PyModule) -> PyResult<()> {
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
