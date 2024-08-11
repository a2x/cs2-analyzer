pub use analysis::{AnalysisResult, AnalyzerOptions};
pub use error::{Error, Result};

use std::collections::HashMap;
use std::path::Path;

use pelite::pe64::PeFile;

#[cfg(not(target_arch = "wasm32"))]
use pelite::FileMap;

pub mod analysis;
pub mod error;

mod source2;

#[derive(Clone, Debug, PartialEq)]
pub struct Analyzer {
    files: HashMap<String, Vec<u8>>,
    options: AnalyzerOptions,
}

impl Analyzer {
    /// Creates a new [`Analyzer`] instance with the default options.
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
            options: AnalyzerOptions::default(),
        }
    }

    /// Creates a new [`Analyzer`] instance with the specified options.
    pub fn new_with_opts(options: AnalyzerOptions) -> Self {
        Self {
            files: HashMap::new(),
            options,
        }
    }

    /// Adds a file to the analyzer.
    #[cfg(target_arch = "wasm32")]
    pub fn add_file<P: AsRef<Path>>(&mut self, _path: P) -> Result<()> {
        Err(Error::Other(
            "Analyzer::add_file is not supported in the WebAssembly target",
        ))
    }

    /// Adds a file to the analyzer.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) {
        let path = path.as_ref();

        if let Some(file_name) = path.file_name().and_then(|f| f.to_str()) {
            if let Ok(map) = FileMap::open(path) {
                let data = map.as_ref().to_vec();

                self.files.insert(file_name.to_string(), data);
            }
        }
    }

    /// Adds multiple files to the analyzer.
    #[cfg(target_arch = "wasm32")]
    pub fn add_files<P: AsRef<Path>>(&mut self, _paths: &[P]) -> Result<()> {
        Err(Error::Other(
            "Analyzer::add_files is not supported in the WebAssembly target",
        ))
    }

    /// Adds multiple files to the analyzer.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn add_files<P: AsRef<Path>>(&mut self, paths: &[P]) {
        for path in paths {
            self.add_file(path);
        }
    }

    /// Analyzes all added files.
    #[cfg(target_arch = "wasm32")]
    pub fn analyze(&self) -> Result<HashMap<String, AnalysisResult<'_>>> {
        Err(Error::Other(
            "Analyzer::analyze is not supported in the WebAssembly target",
        ))
    }

    /// Analyzes all added files.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn analyze(&self) -> HashMap<String, AnalysisResult<'_>> {
        let mut results = HashMap::new();

        for (file_name, data) in &self.files {
            if let Ok(result) = self.analyze_from_bytes(data) {
                results.insert(file_name.clone(), result);
            }
        }

        results
    }

    /// Analyzes a file by name.
    #[cfg(target_arch = "wasm32")]
    pub fn analyze_file(&self, _file_name: &str) -> Result<AnalysisResult<'_>> {
        Err(Error::Other(
            "Analyzer::analyze_file is not supported in the WebAssembly target",
        ))
    }

    /// Analyzes a file by name.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn analyze_file(&self, file_name: &str) -> Result<AnalysisResult<'_>> {
        if let Some(data) = self.files.get(file_name) {
            self.analyze_from_bytes(data)
        } else {
            Err(Error::Other("file not found"))
        }
    }

    /// Analyzes a file from a byte slice.
    pub fn analyze_from_bytes<'a>(&self, bytes: &'a [u8]) -> Result<AnalysisResult<'a>> {
        let file = PeFile::from_bytes(bytes)?;

        analysis::analyze_with_opts(file, &self.options)
    }
}
