use crate::server::cache::Cache;

pub fn process_command(input: &String, cache: &mut Cache) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let command = parts.get(0);

    match command {
        Some(&"set") => {
            let key = String::from(parts[1]);
            let value = String::from(parts[2]);
            cache.put(&key, &value);
            println!("Called set with: {key} -> {value}");
        }
        Some(&"get") => {
            let key = String::from(parts[1]);
            let value = cache.get(&key);
            match value {
                Some(&ref v) => {
                    println!("Called get with: {key}: got {v}")
                }
                None => {
                    println!("Called get with: {key}. Value not found")
                }
            }
        }
        Some(&"exists") => {
            let key = String::from(parts[1]);
            let does_exist = cache.exists(&key);
            println!("Called exists with: {key} - found? {does_exist}");
        }
        Some(&"exit") => {
            println!("Wrapping up");
            // todo
            return;
        }
        _ => {
            println!("Invalid command.");
        }
    }
}