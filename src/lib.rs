#![allow(unused_imports)]
#![allow(dead_code)]

use pyo3::prelude::*;

// O interpretador Python vai interagir com esses símbolos dinamicamente em tempo de execução
extern "C" {
    fn prefetch_page(page_id: u64, score: f32) -> i32;
    fn insert_page_to_ram(page_id: u64, tokens_ptr: *const u32, len: usize) -> i32;
    fn read_tokens_from_ram(page_id: u64, output_ptr: *mut u32) -> i32;
}

pub const TOKENS_PER_PAGE: usize = 1024;

#[pyclass]
pub struct PyContextEngine {}

#[pymethods]
impl PyContextEngine {
    #[new]
    pub fn new(_swap_file_path: String, _total_disk_pages: usize, _max_ram_pages: usize) -> PyResult<Self> {
        Ok(PyContextEngine {})
    }

    pub fn insert_page(&mut self, page_id: u64, tokens: Vec<u32>) -> PyResult<()> {
        if tokens.len() > TOKENS_PER_PAGE {
            return Err(pyo3::exceptions::PyValueError::new_err("Token slice exceeds TOKENS_PER_PAGE"));
        }
        unsafe {
            insert_page_to_ram(page_id, tokens.as_ptr(), tokens.len());
        }
        Ok(())
    }

    pub fn prefetch_page(&mut self, page_id: u64, attention_score: f32) -> PyResult<()> {
        unsafe {
            prefetch_page(page_id, attention_score);
        }
        Ok(())
    }
}

#[pymodule]
fn infinite_context_engine(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyContextEngine>()?;
    Ok(())
}