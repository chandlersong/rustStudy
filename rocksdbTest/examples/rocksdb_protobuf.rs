use prost::Message;
use anyhow::Result;
use rocksdb::DB;
// 包含编译生成的 Protobuf 代码
mod tutorial {
    include!(concat!(env!("OUT_DIR"), "/tutorial.rs"));
}


fn main() -> Result<()> {
    // 创建 Person 实例
    // 打开 RocksDB 数据库
    let db = DB::open_default("rocksdb_storage")?;

    // 创建 Protobuf 消息实例
    let person = tutorial::Person {
        name: "Alice".to_string(),
        age: 25,
        email: "bbb".to_string(),
    };

    // 序列化为二进制
    let mut buf = Vec::new();
    person.encode(&mut buf)?;
    println!("Serialized bytes: {:?}", buf);

    // 存入 RocksDB
    let key = b"person_key";
    db.put(key, &buf)?;
    println!("Stored in RocksDB with key: {:?}", key);

    // 从 RocksDB 中取出
    match db.get(key)? {
        Some(value) => {
            // 反序列化为 Protobuf 消息
            let decoded = tutorial::Person::decode(&value[..])?;
            println!(
                "Retrieved from RocksDB: name = {}, age = {}",
                decoded.name, decoded.age
            );
        }
        None => println!("Key not found"),
    }

    Ok(())
}