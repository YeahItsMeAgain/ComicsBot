use once_cell::sync::OnceCell;
use persistent_kv::PersistentKeyValueStore;

pub static STORE: OnceCell<PersistentKeyValueStore<String, String>> = OnceCell::new();

pub fn get_store() -> &'static PersistentKeyValueStore<String, String> {
    STORE.get().unwrap()
}
