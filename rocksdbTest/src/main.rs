use rocksdb::{Options, DB};

fn main() {
    let path = "./rocksdbTest/db/helloworld";
    let mut options = Options::default();
    options.create_if_missing(true);

    // 打开数据库
    let db = DB::open(&options, path).unwrap();

    // 插入哈希表字段
    db.put(b"user:1:name", b"Alice").unwrap();
    db.put(b"user:1:age", b"30").unwrap();
    db.put(b"user:1:email", b"alice@example.com").unwrap();

    // 读取哈希表字段
    let name = db.get(b"user:1:name").unwrap().unwrap();
    let age = db.get(b"user:1:age").unwrap().unwrap();
    let email = db.get(b"user:1:email").unwrap().unwrap();

    println!(
        "Name: {}, Age: {}, Email: {}",
        std::str::from_utf8(&name).unwrap(),
        std::str::from_utf8(&age).unwrap(),
        std::str::from_utf8(&email).unwrap()
    );

    // 删除数据库
    drop(db);
}
