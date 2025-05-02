#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![print, printByNumber, pwd])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn print() -> Result<(), String> {
  println!("print_pdf in tauri");
    std::process::Command::new("lp")
        .arg("/Users/admin/Documents/nummeriert/quittung_1.pdf")
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn printByNumber(number: i32) -> Result<(), String> {
    println!("printByNumber in tauri");
    let home_dir = std::env::var("HOME").map_err(|e| e.to_string())?;
    std::process::Command::new("lp")
        .arg(format!("{}/Documents/nummeriert/quittung_{}.pdf", home_dir, number))
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn pwd() -> Result<String, String> {
    let output = std::process::Command::new("pwd")
        .output()
        .map_err(|e| e.to_string())?;
    println!("Current directory: {}", String::from_utf8_lossy(&output.stdout));
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}


