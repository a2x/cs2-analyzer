use cs2_analyzer::{Analyzer, AnalyzerOptions, Result};

use walkdir::WalkDir;

fn main() -> Result<()> {
    let cs2_path = find_cs2_install_path()?;

    let dll_paths: Vec<_> = WalkDir::new(&cs2_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".dll"))
        .map(|e| e.path().to_path_buf())
        .collect();

    let mut analyzer = Analyzer::new_with_opts(AnalyzerOptions {
        buttons: false,
        concommands: false,
        convars: false,
        interfaces: true,
        offsets: false,
        schemas: false,
    });

    analyzer.add_files(&dll_paths);

    // Analyze all added files (This may take a while).
    let result = analyzer.analyze();

    for (file_name, result) in &result {
        for iface in &result.interfaces {
            println!(
                "found interface: {} in {} ({} + {:#X})",
                iface.name, file_name, file_name, iface.rva
            );
        }

        println!(
            "found {} interfaces in {}",
            result.interfaces.len(),
            file_name
        );
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
