use cs2_analyzer::{Analyzer, AnalyzerOptions, Result};

fn main() -> Result<()> {
    let install_path = find_cs2_install_path()?;

    let dll_paths = &[
        format!(r"{}\game\bin\win64\engine2.dll", install_path),
        format!(r"{}\game\bin\win64\inputsystem.dll", install_path),
        format!(r"{}\game\csgo\bin\win64\client.dll", install_path),
        format!(r"{}\game\csgo\bin\win64\matchmaking.dll", install_path),
    ];

    let mut analyzer = Analyzer::new_with_opts(AnalyzerOptions {
        buttons: false,
        concommands: false,
        convars: false,
        interfaces: false,
        offsets: true,
        schemas: false,
    });

    analyzer.add_files(dll_paths);

    // Analyze all the files (This may take a while).
    let result = analyzer.analyze();

    for (file_name, result) in &result {
        for (name, value) in &result.offsets {
            println!("[{}] {} @ {:#X}", file_name, name, value);
        }
    }

    Ok(())
}

#[cfg(target_family = "windows")]
fn find_cs2_install_path() -> Result<String> {
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let cs2 = hklm.open_subkey(r"SOFTWARE\WOW6432Node\Valve\cs2")?;

    let install_path: String = cs2.get_value("installpath")?;

    Ok(install_path)
}

#[cfg(not(target_family = "windows"))]
fn find_cs2_install_path() -> Result<String> {
    unimplemented!("auto-detecting cs2 install path is only supported on windows")
}
