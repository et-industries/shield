use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use shield_circuit::{
    pool::{AnonymityPool, Note},
    Hash,
};
use std::sync::Mutex;
use std::{collections::HashMap, ops::AddAssign};
use tauri::Manager;

lazy_static! {
    static ref NOTES: Mutex<HashMap<Hash, Note>> = Mutex::new(HashMap::new());
    static ref POOL: Mutex<AnonymityPool> = Mutex::new(AnonymityPool::new());
    static ref TOPIC: Mutex<u64> = Mutex::new(0);
}

#[tauri::command]
fn get_default_account() -> String {
    AnonymityPool::account().to_string()
}

#[tauri::command]
fn get_default_amount() -> String {
    AnonymityPool::amount().to_string()
}

#[tauri::command]
fn get_balance(account: u64) -> Result<String, String> {
    let pool = match POOL.lock() {
        Ok(pool) => pool,
        Err(e) => return Err(e.to_string()),
    };
    Ok(pool.get_balance(account).to_string())
}

#[tauri::command]
fn deposit(recipiant: u64) -> Result<String, String> {
    let mut rng = thread_rng();
    let secret = rng.gen::<u64>();
    let mut topic = match TOPIC.lock() {
        Ok(topic) => topic,
        Err(e) => return Err(e.to_string()),
    };
    let mut notes = match NOTES.lock() {
        Ok(notes) => notes,
        Err(e) => return Err(e.to_string()),
    };
    let mut pool = match POOL.lock() {
        Ok(pool) => pool,
        Err(e) => return Err(e.to_string()),
    };
    let note = pool.deposit(AnonymityPool::account(), secret, *topic, recipiant);
    let nullifier = note.nullifier();
    notes.insert(nullifier.clone(), note);
    topic.add_assign(1);

    Ok(nullifier.to_hex())
}

#[tauri::command]
fn get_notes() -> Result<HashMap<Hash, Note>, String> {
    let notes = match NOTES.lock() {
        Ok(pool) => pool,
        Err(e) => return Err(e.to_string()),
    };
    Ok(notes.clone())
}

#[tauri::command]
fn get_nullifiers() -> Result<HashMap<Hash, bool>, String> {
    let pool = match POOL.lock() {
        Ok(pool) => pool,
        Err(e) => return Err(e.to_string()),
    };
    Ok(pool.nullifiers())
}

#[tauri::command]
fn withdraw(nullifier: Hash) -> Result<bool, String> {
    let mut pool = match POOL.lock() {
        Ok(pool) => pool,
        Err(e) => return Err(e.to_string()),
    };
    let notes = match NOTES.lock() {
        Ok(pool) => pool,
        Err(e) => return Err(e.to_string()),
    };
    let res = match notes.get(&nullifier) {
        Some(note) => pool.withdraw(note.clone()),
        None => return Err("Not Found".to_string()),
    };

    Ok(res)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            deposit,
            withdraw,
            get_notes,
            get_balance,
            get_nullifiers,
            get_default_amount,
            get_default_account,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[test]
fn test_multiple_deposit_withdraw() {
    let n1 = deposit(1).unwrap();
    let n2 = deposit(2).unwrap();
    assert_eq!(withdraw(Hash::from_hex(n1)).unwrap(), true);
    assert_eq!(withdraw(Hash::from_hex(n2)).unwrap(), true);
}
