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

#[derive(Clone, Debug, Default)]
pub struct AnalysisResult<'a> {
    pub buttons: Vec<Button<'a>>,
    pub concommands: Vec<ConCommand<'a>>,
    pub convars: Vec<ConVar<'a>>,
    pub interfaces: Vec<Interface<'a>>,
    pub offsets: BTreeMap<&'a str, Rva>,
    pub classes: Vec<Class<'a>>,
    pub enums: Vec<Enum<'a>>,
}

#[derive(Clone, Copy, Debug)]
pub struct ParserOptions {
    /// Whether to parse buttons.
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

pub fn analyze(file: PeFile<'_>) -> Result<AnalysisResult<'_>> {
    analyze_with_opts(file, &ParserOptions::default())
}

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
