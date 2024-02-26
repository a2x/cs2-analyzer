use cs2_analyzer::{Analyzer, ParserOptions, Result};

fn main() -> Result<()> {
    let install_path = find_cs2_install_path()?;

    let mut analyzer = Analyzer::new_with_opts(ParserOptions {
        buttons: true,
        concommands: false,
        convars: false,
        interfaces: false,
        offsets: false,
        schemas: false,
    });

    analyzer.add_file(format!(r"{}\game\csgo\bin\win64\client.dll", install_path));

    // Analyze the file.
    let result = analyzer.analyze_file("client.dll")?;

    for button in &result.buttons {
        println!("{:#?}", button);
    }

    Ok(())
}

#[cfg(target_family = "windows")]
fn find_cs2_install_path() -> Result<String> {
    use winreg::enums::HKEY_LOCAL_MACHINE;
    use winreg::RegKey;

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let cs2 = hklm.open_subkey("SOFTWARE\\WOW6432Node\\Valve\\cs2")?;

    let install_path: String = cs2.get_value("installpath")?;

    Ok(install_path)
}

#[cfg(not(target_family = "windows"))]
fn find_cs2_install_path() -> Result<String> {
    unimplemented!("auto-detecting cs2 install path is only supported on windows")
}
