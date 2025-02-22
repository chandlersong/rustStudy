use prost::Message;
use anyhow::Result;

// 包含编译生成的 Protobuf 代码
mod tutorial {
    include!(concat!(env!("OUT_DIR"), "/tutorial.rs"));
}


fn main() -> Result<()> {
    // 创建 Person 实例
    let person = tutorial::Person {
        name: "Bob".to_string(),
        age: 30,
        email: "bob@example.com".to_string(),
    };

    // 序列化为二进制
    let mut buf = Vec::new();
    person.encode(&mut buf)?;
    println!("Serialized bytes: {:?}", buf);

    // 反序列化
    let decoded = tutorial::Person::decode(&buf[..])?;
    println!(
        "Deserialized: name = {}, age = {}, email = {}",
        decoded.name, decoded.age, decoded.email
    );

    Ok(())
}