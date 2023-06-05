

pub mod db{

    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::Read;
    use std::fmt;
    use std::result;
    use std::path;
    use struct_db::*;
    use crate::entities::{agent::*, ship::*};
    use crate::logger;

    // Update the database
    pub fn update<T>(db: &Db, datum: T) -> std::result::Result<(), Box<dyn std::error::Error>>{
        // update calls
            
        Ok(())
    }

    pub fn insert<T: SDBItem>(db: &Db, datum: T) -> std::result::Result<(), Box<dyn std::error::Error>>{
        let mut txn = db.transaction()?;
        {
            let mut tables = txn.tables();
            tables.insert(&txn, datum)?;
        } 
        txn.commit()?;
        Ok(())
    }

    // Searches the database with primary key
    pub fn read<T: SDBItem>(db: &Db, p_key: String) -> T{
        println!("read_transaction");
        let mut txn = db.read_transaction().unwrap();
        println!("get tables");
        let mut tables = txn.tables();
        println!("make call with primary key {}", p_key);
        println!("p_key: {:#?}", p_key.as_bytes());
        // vvvvvvvvvvvvvvvvvv FIXME:
        let data: T = tables.primary_get(&txn, p_key.as_bytes()).unwrap().unwrap();
        // ^^^^^^^^^^^^^^^^^^
        data
    }

    pub fn init_db(p: &str) -> Result<Db>{
        let path = path::Path::new(p);
        
        let mut db = Db::init(&path)?;
        
        logger::log(format!("LATTICE initialized with path \"{}\". Now defining schemas...", p).as_str(), logger::AlertType::DEFAULT, false);
        // Initiate schemas (do this for each struct)
        db.define::<Agents>();
        logger::log("Schemas have been defined.", logger::AlertType::DEFAULT, false);

        Ok(db)
    }

}