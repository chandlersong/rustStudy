use rocksdb::{DB};
use serde::{Serialize, Deserialize};
use serde_json;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    age: u8,
    email: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 打开数据库
    let db = DB::open_default("test_db")?;

    // 创建一个用户对象
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    // 序列化为 JSON 字符串
    let user_json = serde_json::to_string(&user)?;

    // 保存到 RocksDB
    let key = format!("user:{}", user.id);
    db.put(key.as_bytes(), user_json.as_bytes())?;
    println!("User saved to RocksDB: {:?}", user);

    // 从 RocksDB 读取
    if let Some(value) = db.get(key.as_bytes())? {
        let user_json = str::from_utf8(&value)?;
        let user: User = serde_json::from_str(user_json)?;
        println!("User loaded from RocksDB: {:?}", user);
    } else {
        println!("User not found in RocksDB");
    }

    Ok(())
}