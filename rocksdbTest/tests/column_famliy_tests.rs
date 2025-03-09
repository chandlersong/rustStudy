#[cfg(test)]
mod tests {
    use std::os::unix::raw::ino_t;
    use rocksdb::{ColumnFamilyDescriptor, Options, DB};
    use std::path::Path;

    const DATA_FOLDER: &str = "./db/cf";

    #[test]
    fn test_column_family() -> Result<(), rocksdb::Error> {
        let mut options = Options::default();
        options.create_if_missing(true);

        if Path::new(DATA_FOLDER).exists() {
            DB::destroy(&options, DATA_FOLDER).unwrap();
        }
        // {
        //     let mut db = DB::open(&options, DATA_FOLDER)?;
        //     db.create_cf("cf1", &options)?;
        //     db.flush()?
        // }
        let cf_descriptors = vec![
            ColumnFamilyDescriptor::new("cf1", Options::default()),
        ];

        let db = DB::open_cf_descriptors(&options, DATA_FOLDER,cf_descriptors)?;

        Ok(())
    }


    ///
    /// 列出所有CF
    #[test]
    fn test_list_column_family() -> Result<(), rocksdb::Error> {
        let mut options = rocksdb::Options::default();
        options.create_if_missing(true);
        let cfs = DB::list_cf(&options, DATA_FOLDER)?;
        for cf in &cfs {
            println!("exist cf {}", cf);
        }


        Ok(())
    }
}
