use std::collections::HashMap;

use miden_hir::{
    FunctionExportName, FunctionInvocationMethod, InterfaceFunctionIdent, MastRootHash,
};

/// A Miden VM codegen metadata for the function import.
/// This struct will have more fields in the future e.g. where the function
/// for this MAST hash is located (to be loaded by the VM)
#[derive(Debug, Clone)]
pub struct ImportMetadata {
    /// The MAST hash of the function to be used in codegen
    pub function_mast_root_hash: MastRootHash,
    /// The method of calling the function
    pub invoke_method: FunctionInvocationMethod,
}

/// A function export metadata
#[derive(Debug, Clone)]
pub struct ExportMetadata {
    /// The method of calling the function
    pub invoke_method: FunctionInvocationMethod,
}

/// Configuration for the WASM translation.
#[derive(Debug)]
pub struct WasmTranslationConfig {
    /// The moduie name to use if Wasm module doesn't have one.
    // TODO: Should we ditch it altogether? In CM there are multiple modules without names.
    pub module_name_fallback: String,

    /// Whether or not to generate native DWARF debug information.
    pub generate_native_debuginfo: bool,

    /// Whether or not to retain DWARF sections in compiled modules.
    pub parse_wasm_debuginfo: bool,

    /// Import metadata for MAST hashes, calling convention, of
    /// each imported function. Having it here might be a temporary solution,
    /// later we might want to move it to Wasm custom section.
    pub import_metadata: HashMap<InterfaceFunctionIdent, ImportMetadata>,

    /// Export metadata for calling convention, etc.
    pub export_metadata: HashMap<FunctionExportName, ExportMetadata>,
}

impl Default for WasmTranslationConfig {
    fn default() -> Self {
        Self {
            module_name_fallback: "noname".to_string(),
            generate_native_debuginfo: false,
            parse_wasm_debuginfo: false,
            import_metadata: Default::default(),
            export_metadata: Default::default(),
        }
    }
}
