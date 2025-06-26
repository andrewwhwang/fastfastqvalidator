use pyo3::prelude::*;
use pyo3::exceptions::PyException;

use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
// use phf::phf_set;


fn run<P: AsRef<Path>>(filepath: P) -> Result<(), String> {
    let file = File::open(&filepath).map_err(|e| format!("Failed to open file: {}", e))?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    let mut read_ids = HashSet::new();

    let mut len_read_seq: usize = 0;
    let mut line_num: u128 = 0;

    // let allowed_bases: phf::Set<u8> = phf_set! {b'A', b'T', b'G', b'C', b'N', b'a', b't', b'g', b'c', b'n', b'\n'};

    // for (i, line_result) in reader.lines().enumerate() {
    while reader.read_until(b'\n', &mut buffer).map_err(|e| format!("Read error: {}", e))? != 0 {
        line_num += 1;

        match line_num % 4 {
            1 => { // read ID
                if !buffer.starts_with(b"@") {
                    return Err(format!(
                        "FastQ Format Error: Record 1 expected '@' but found: '{}'. Line Number: {}",
                        String::from_utf8_lossy(&buffer), line_num
                    ));
                }
                if !read_ids.insert(buffer.clone()) {
                    return Err(format!("Duplicate read detected: '{}'.", String::from_utf8_lossy(&buffer)));
                }
            }
            2 => {
                // if buffer.iter().any(|c| !allowed_bases.contains(c)) {
                //     return Err(format!("Line {}: Invalid characters in sequence '{}'", line_num, String::from_utf8_lossy(&buffer)));
                // }
                len_read_seq = buffer.len();
            }
            3 => {
                if !buffer.starts_with(b"+") {
                    return Err(format!(
                        "FastQ Format Error: Record 3 expected '+' but found: '{}'. Line Number: {}",
                        String::from_utf8_lossy(&buffer), line_num
                    ));
                }
            }
            0 => { // quality line
                if buffer.len() != len_read_seq {
                    return Err(format!(
                        "FastQ Format Error: Read Qual Length != Read Seq Length. Line Number: '{}'", line_num
                    ));
                }
            }
            _ => unreachable!(),
        }
        buffer.clear();
    }

    if read_ids.is_empty() {
        let fname = filepath.as_ref().file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or_else(|| "<unknown>".into());
        return Err(format!(
            "Input file: {} is empty. Please check and re-upload.",
            fname
        ));
    } else if line_num % 4 != 0 {
        let fname = filepath.as_ref().file_name()
            .map(|s| s.to_string_lossy())
            .unwrap_or_else(|| "<unknown>".into());
        return Err(format!(
            "Input file: {} is not in valid FastQ format. Line count = {} (not divisible by 4).",
            fname, line_num
        ));
    }

    Ok(())
}


#[pyclass(extends=PyException)]
struct FastqFormatError;


#[pyfunction]
fn validate(file_name:String) -> PyResult<()> { 
    let result = run(file_name);
    result.map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
}

/// A Python module implemented in Rust.
#[pymodule]
fn fq(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add("FastqFormatError", py.get_type::<FastqFormatError>())?;
    Ok(())
}
