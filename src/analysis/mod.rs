pub use client::Button;
pub use concommands::ConCommand;
pub use convars::{ConVar, ConVarFlags};
pub use globals::Global;
pub use interfaces::Interface;
pub use schemas::{Class, ClassField, Enum, EnumMember};

use std::collections::BTreeMap;

use pelite::pattern;
use pelite::pe64::{Pe, PeFile, Rva};

use crate::error::{Error, Result};

pub mod client;
pub mod concommands;
pub mod convars;
pub mod engine2;
pub mod globals;
pub mod input_system;
pub mod interfaces;
pub mod matchmaking;
pub mod schemas;

/// Represents the result of an analysis.
#[derive(Clone, Debug, Default)]
pub struct AnalysisResult<'a> {
    /// A list of buttons found during the analysis.
    pub buttons: Vec<Button<'a>>,

    /// A list of concommands found during the analysis.
    pub concommands: Vec<ConCommand<'a>>,

    /// A list of convars found during the analysis.
    pub convars: Vec<ConVar<'a>>,

    /// A list of interfaces found during the analysis.
    pub interfaces: Vec<Interface<'a>>,

    /// A map of offsets found during the analysis, with the offset name as the key and the
    /// relative virtual address (RVA) as the value.
    pub offsets: BTreeMap<&'a str, Rva>,

    /// A list of classes found during the analysis.
    pub classes: Vec<Class<'a>>,

    /// A list of enums found during the analysis.
    pub enums: Vec<Enum<'a>>,
}

/// Represents the options for the parser.
#[derive(Clone, Copy, Debug)]
pub struct ParserOptions {
    /// Whether to parse key buttons.
    pub buttons: bool,

    /// Whether to parse concommands.
    pub concommands: bool,

    /// Whether to parse convars.
    pub convars: bool,

    /// Whether to parse interfaces.
    pub interfaces: bool,

    /// Whether to parse offsets.
    pub offsets: bool,

    /// Whether to parse schema classes/enums.
    pub schemas: bool,
}

impl Default for ParserOptions {
    /// Set all parsing options to true by default.
    fn default() -> Self {
        Self {
            buttons: true,
            concommands: true,
            convars: true,
            interfaces: true,
            offsets: true,
            schemas: true,
        }
    }
}

/// Analyzes the given PE file with the default parser options.
pub fn analyze(file: PeFile<'_>) -> Result<AnalysisResult<'_>> {
    analyze_with_opts(file, &ParserOptions::default())
}

/// Analyzes the given PE file with the given parser options.
pub fn analyze_with_opts<'a>(file: PeFile<'a>, opts: &ParserOptions) -> Result<AnalysisResult<'a>> {
    let module_name = read_module_name(file)?;

    let mut result = AnalysisResult::default();

    if opts.buttons {
        result.buttons = match module_name {
            "client.dll" => client::buttons(file),
            _ => vec![],
        };
    }

    if opts.concommands {
        result.concommands = concommands::concommands(file);
    }

    if opts.convars {
        result.convars = convars::convars(file);
    }

    if opts.interfaces {
        result.interfaces = interfaces::interfaces(file);
    }

    if opts.offsets {
        result.offsets = match module_name {
            "client.dll" => client::offsets(file),
            "engine2.dll" => engine2::offsets(file),
            "inputsystem.dll" => input_system::offsets(file),
            "matchmaking.dll" => matchmaking::offsets(file),
            _ => BTreeMap::new(),
        };
    }

    if opts.schemas {
        let (classes, enums) = schemas::schemas(file);

        result.classes = classes;
        result.enums = enums;
    }

    Ok(result)
}

/// Reads the module name from the PE file.
fn read_module_name(file: PeFile<'_>) -> Result<&str> {
    let mut save = [0; 2];

    if !file
        .scanner()
        .finds_code(pattern!("e8${488d05${'}} 488bd0498bcf"), &mut save)
    {
        return Err(Error::Other("Unable to read module name"));
    }

    let name = file.derva_c_str(save[1])?.to_str()?;

    Ok(name)
}
