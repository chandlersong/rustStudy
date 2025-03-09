extern crate rocksdb;
use rocksdb::DB;
use std::time::Instant;
use std::error::Error;
use std::path::Path;

const NUM_WRITES: usize = 100_000_000;

fn create_key(i: u32) -> Vec<u8> {
    let mut key = Vec::with_capacity(8);
    key.extend_from_slice(b"key ");
    key.extend_from_slice(&i.to_be_bytes());
    key
}

fn create_value() -> Vec<u8> {
    vec![b'a'; 100]
}

fn generate_data(n: usize) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut data = Vec::with_capacity(n);
    for i in 0..n {
        let key = create_key(i as u32);
        let value = create_value();
        data.push((key, value));
    }
    data
}

fn test_individual_puts(db: &DB, data: &[(Vec<u8>, Vec<u8>)]) -> std::time::Duration {
    let start = Instant::now();
    for (key, value) in data {
        db.put(key, value).unwrap();
    }
    start.elapsed()
}

fn test_write_batch(db: &DB, data: &[(Vec<u8>, Vec<u8>)]) -> std::time::Duration {
    let start = Instant::now();
    let mut batch = rocksdb::WriteBatch::default();
    for (key, value) in data {
        batch.put(key, value);
    }
    db.write(batch).unwrap();
    start.elapsed()
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./rocksdbTest/db/performance";
    let mut options = rocksdb::Options::default();
    options.create_if_missing(true);

    if Path::new(path).exists() {
        DB::destroy(&options, path).unwrap();
    }

    let db = DB::open(&options, path)?;

    let data = generate_data(NUM_WRITES);

    let time_individual = test_individual_puts(&db, &data);
    println!(
        "Individual put operations: {} milliseconds per operation",
        time_individual.as_millis() as f64 / NUM_WRITES as f64
    );

    let time_batch = test_write_batch(&db, &data);
    println!(
        "WriteBatch operations: {} milliseconds per operation",
        time_batch.as_millis() as f64 / NUM_WRITES as f64
    );

    // DB::destroy(&options, path).expect("TODO: panic message");
    Ok(())
}
