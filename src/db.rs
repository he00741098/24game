use std::env;
use std::process::Command;

pub fn read_all_from_db() {
    let read_all_command = Command::new("curl")
        .arg(format!("{}?prefix=", get_replit_db_url()))
        .output()
        .expect("Failed to read all keys");

    println!(
        "All numbers found in the array so far: \n{}",
        String::from_utf8_lossy(&read_all_command.stdout)
    );
}

pub fn save_to_db(key: &str, value: &i32) {
    let key_value_pair = format!("{}={}", key, value);

    Command::new("curl")
        .arg(get_replit_db_url())
        .arg("-d")
        .arg(&key_value_pair)
        .output()
        .expect("Failed to create a new key-value pair");
}

fn get_replit_db_url() -> String {
    return env::var("REPLIT_DB_URL").unwrap();
}

use std::any::type_name;
pub fn clear(){
    let read_all_command = Command::new("curl")
        .arg(format!("{}?prefix=", get_replit_db_url()))
        .output()
        .expect("Failed to read all keys");
	
	
//delete(&(k as i32));
	println!("{}",String::from_utf8_lossy(&read_all_command.stdout));	
	print_type_of(&String::from_utf8_lossy(&read_all_command.stdout));
	
}

pub fn delete(key:&i32){
	println!("Deleting {}", key);
    Command::new("curl")
        .arg("-XDELETE")
        .arg(get_replit_db_url())
        .arg(key.to_string())
        .output()
        .expect("Failed to delete key");
	
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


pub fn query_key(key:&str) -> String{
let g = Command::new("curl")
.arg(get_replit_db_url())
.arg(key)
.output().expect("Failed to get key");
let s = String::from_utf8_lossy(&g.stdout);
s.to_string()
}