const DEFAULT_ACCOUNT: u64 = 123;

lazy_static! {
    static ref NOTES: Mutex<HashMap<Hash, Note>> = Mutex::new(HashMap::new());
    static ref POOL: Mutex<AnonymityPool> = Mutex::new(AnonymityPool::new(DEFAULT_ACCOUNT));
    static ref TOPIC: Mutex<u64> = Mutex::new(0);
}

#[tauri::command]
fn get_default_account() -> String {
    DEFAULT_ACCOUNT.to_string()
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
    let note = pool.deposit(DEFAULT_ACCOUNT, secret, topic.clone(), recipiant);
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
