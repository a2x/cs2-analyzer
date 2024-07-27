use cs2_analyzer::{Analyzer, AnalyzerOptions, Result};

fn main() -> Result<()> {
    let cs2_path = find_cs2_install_path()?;

    let mut analyzer = Analyzer::new_with_opts(AnalyzerOptions {
        buttons: true,
        concommands: false,
        convars: false,
        interfaces: false,
        offsets: false,
        schemas: false,
    });

    analyzer.add_file(format!(r"{}\game\csgo\bin\win64\client.dll", cs2_path));

    let result = analyzer.analyze_file("client.dll")?;

    for button in &result.buttons {
        println!(
            "found button: {} (client.dll + {:#X})",
            button.name, button.rva
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
