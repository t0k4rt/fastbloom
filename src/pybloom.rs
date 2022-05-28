use pyo3::prelude::*;
use pyo3::types::PyBytes;

use fastbloom_rs::{BloomFilter, FilterBuilder};

#[pyclass]
pub struct PyFilterBuilder {
    filter_builder: FilterBuilder,
}

#[pymethods]
impl PyFilterBuilder {
    #[new]
    pub fn __init__(expected_elements: u64, false_positive_probability: f64) -> PyResult<Self> {
        Ok(
            PyFilterBuilder {
                filter_builder: FilterBuilder::new(expected_elements, false_positive_probability)
            }
        )
    }

    pub fn build_bloom_filter(&mut self) -> PyResult<PyBloomFilter> {
        let filter = self.filter_builder.build_bloom_filter();
        Ok(PyBloomFilter { bloomfilter: filter })
    }

    pub fn expected_elements(&self) -> u64 {
        self.filter_builder.expected_elements
    }

    pub fn false_positive_probability(&self) -> f64 {
        self.filter_builder.false_positive_probability
    }

    pub fn size(&self) -> u64 {
        self.filter_builder.size
    }

    pub fn hashes(&self) -> u32 {
        self.filter_builder.hashes
    }
}


#[pyclass]
pub struct PyBloomFilter {
    bloomfilter: BloomFilter,
}

#[pymethods]
impl PyBloomFilter {
    pub fn add_int(&mut self, element: i64) {
        self.bloomfilter.add(&i64::to_le_bytes(element));
    }

    pub fn add_int_batch(&mut self, array: Vec<i64>) {
        for x in array {
            self.add_int(x)
        };
    }

    pub fn add_str(&mut self, element: &str) {
        self.bloomfilter.add(element.as_bytes());
    }

    pub fn add_str_batch(&mut self, array: Vec<&str>) {
        for x in array {
            self.bloomfilter.add(x.as_bytes())
        }
    }

    pub fn add_bytes(&mut self, bts: &PyBytes) {
        self.bloomfilter.add(bts.as_bytes());
    }

    pub fn contains_int(&mut self, element: i64) -> bool {
        self.bloomfilter.contains(&i64::to_le_bytes(element))
    }

    pub fn contains_str(&mut self, element: &str) -> bool {
        self.bloomfilter.contains(element.as_bytes())
    }

    pub fn contains_bytes(&self, bts: &PyBytes) -> bool {
        self.bloomfilter.contains(bts.as_bytes())
    }

    pub fn config(&self) -> PyResult<PyFilterBuilder> {
        Ok(PyFilterBuilder { filter_builder: self.bloomfilter.config() })
    }

    pub fn hashes(&self) -> PyResult<u32> {
        Ok(self.bloomfilter.hashes())
    }

    pub fn get_bytes(&self) -> PyResult<&[u8]> {
        Ok(self.bloomfilter.get_u8_array())
    }

    pub fn get_int_array(&self) -> PyResult<Vec<u32>> {
        Ok(Vec::from(self.bloomfilter.get_u32_array()))
    }

    pub fn clear(&mut self) {
        self.bloomfilter.clear()
    }

    pub fn is_empty(&self) -> PyResult<bool> {
        Ok(self.bloomfilter.is_empty())
    }

    pub fn union(&mut self, other: &PyBloomFilter) -> PyResult<bool> {
        Ok(self.bloomfilter.union(&other.bloomfilter))
    }

    pub fn intersect(&mut self, other: &PyBloomFilter) -> PyResult<bool> {
        Ok(self.bloomfilter.intersect(&other.bloomfilter))
    }


    #[staticmethod]
    pub fn from_bytes(array: &[u8], hashes: u32) -> PyResult<Self> {
        Ok(PyBloomFilter { bloomfilter: BloomFilter::from_u8_array(array, hashes) })
    }

    #[staticmethod]
    pub fn from_int_array(array: Vec<u32>, hashes: u32) -> PyResult<Self> {
        Ok(PyBloomFilter { bloomfilter: BloomFilter::from_u32_array(array.as_slice(), hashes) })
    }
}