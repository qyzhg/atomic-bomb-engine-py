use pyo3::{pyfunction, PyObject, PyResult, Python};
use pyo3::types::{PyDict, PyList};

#[pyfunction]
pub(crate) fn endpoint(py: Python,
            name: String,
            url: String,
            method: String,
            timeout_secs: u64,
            weight: u32,
            json: Option<PyObject>,
            form_data: Option<PyObject>,
            headers: Option<PyObject>,
            cookies: Option<String>,
            assert_options: Option<&PyList>
) -> PyResult<PyObject>{
    let dict = PyDict::new(py);
    dict.set_item("name", name)?;
    dict.set_item("url", url)?;
    dict.set_item("method", method)?;
    dict.set_item("timeout_secs", timeout_secs)?;
    dict.set_item("weight", weight)?;
    if let Some(json) = json{
        dict.set_item("json", json)?;
    };
    if let Some(form_data) = form_data{
        dict.set_item("form_data", form_data)?;
    };
    if let Some(headers) = headers{
        dict.set_item("headers", headers)?;
    };
    if let Some(cookies) = cookies {
        dict.set_item("cookies", cookies)?;
    };
    if let Some(assert_options) = assert_options{
        dict.set_item("assert_options", assert_options)?;
    }
    Ok(dict.into())
}
