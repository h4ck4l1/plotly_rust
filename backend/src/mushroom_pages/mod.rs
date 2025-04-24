

use crate::BackendError;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

pub mod mushroom_cap_diameter;

pub fn fitter(data: Vec<f64>) -> Result<(),BackendError> {
    Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let path = sys.getattr("path")?;
        
        // Add conda site-packages path
        path.call_method1("append", ("/home/sohail/anaconda3/lib/python3.12/site-packages",))?;
        
        let py_data = PyList::new(py, data)?;
        let fitter = py.import("fitter")?;
        let fitter_instance = fitter.getattr("Fitter")?;
        let fitter_call = fitter_instance.call1((py_data,))?;
        fitter_call.call_method0("fit")?;
        let summary = fitter_call.call_method0("summary")?;
        let least_results = summary.
        
        Ok(())
    })
}