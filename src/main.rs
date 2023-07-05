use consts::{ROSHAMBO_NAME, ROSHAMBO_PATH};
use cpython::{PyList, PyModule, PyResult, PyString, Python, PythonObjectWithCheckedDowncast, ObjectProtocol};
mod consts;
mod lib;

fn module_from_str(py: Python<'_>, name: &str, source: &str) -> PyResult<PyModule> {
    let m = PyModule::new(py, name)?;

    let builtins = cpython::PyModule::import(py, "builtins").unwrap();
    m.dict(py).set_item(py, "__builtins__", &builtins).unwrap();

   // OR
    m.add(py, "__builtins__", py.import("builtins")?)?;
    let m_locals = m.get(py, "__dict__")?.extract(py)?;

    // To avoid multiple import, and to add entry to the cache in `sys.modules`.
    let sys = cpython::PyModule::import(py, "sys").unwrap();
    sys.get(py, "modules").unwrap().set_item(py, name, &m).unwrap();

    // Finally, run the moduke
    py.run(source, Some(&m_locals), None)?;
    Ok(m)
}

fn call_python_script(py: Python, fn_sig: &str, args: Vec<&str>) -> PyResult<PyString> {
    let sys = py.import("sys")?;
    PyList::downcast_from(py, sys.get(, "path")?)?.insert_item(py, 0, "pyth");
    let module = py.import("fibo")?;

    let sys = PyModule::import(py, "sys")?;
    sys.add(py, "path", "../mock_data")?; // replace with the directory of your Python script

    let script = PyModule::import(py, "Roshambo")?; // replace with your Python script's name
    let res = script.call(py, fn_sig, (&args,), None)?;
    Ok(res.extract(py)?)
}

fn main() {
    let fn_sig = "newChallengeIssued";
    let args = vec![
        "7055966458134493094026343887509056529495816437298370325219319591575818818703",
        "bc1p6jfkeede6acjhf376y200cqmuhqr95myk9na6uxyvzka4exv5n8qjq2zhz",
    ];

    let gil = Python::acquire_gil();
    let py = gil.python();
    match call_python_script(py, fn_sig, args) {
        Ok(output) => println!("Python script output: {:?}", output.to_string(py)),
        Err(e) => println!("Error: {:?}", e),
    }
}
