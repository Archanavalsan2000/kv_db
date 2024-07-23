struct KeyValue {
    key: String,
    value: String,
}
struct KvDb {
    store: Vec<KeyValue>,
}

impl KvDb {
    fn new() -> KvDb {
        KvDb { store: Vec::new() }
    }

    fn insert(&mut self, key: String, value: String) {
        self.store.push(KeyValue { key, value });
    }

    fn get(&self, key: &str) -> Option<&String> {
        for kv in &self.store {
            if kv.key == key {
                return Some(&kv.value);
            }
        }
        None
    }

    fn delete(&mut self, key: &str) {
        if let Some(postn) = self.store.iter().position(|kv| kv.key == key) {
            self.store.remove(postn);
        }
    }
    
    
}

fn main() {
    let mut db = KvDb::new();

    db.insert("Ram".to_string(), "12".to_string());
    db.insert("Adil".to_string(), "28".to_string());
    db.insert("Ziya".to_string(), "16".to_string());

    match db.get("Ziya") {
        Some(value) => println!("Ziya: {}", value),
        None => println!("Ziya not found"),
    }

    db.delete("Adil");
    match db.get("Adil") {
        Some(value) => println!("Adil: {}", value),
        None => println!("Adil not found"),
    }
}

