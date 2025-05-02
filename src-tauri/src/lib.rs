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
    .invoke_handler(tauri::generate_handler![print, printByNumber])
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
    std::process::Command::new("lp")
        .arg(format!("/Users/admin/Documents/nummeriert/quittung_{}.pdf", number))
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

