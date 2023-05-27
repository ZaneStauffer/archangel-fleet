

pub mod db{

    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::Read;
    use std::fmt;
    use std::result;
    use struct_db::*;
    use crate::schemas::Agents;

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
        let mut txn = db.read_transaction().unwrap();
        let mut tables = txn.tables();
        let data: T = tables.primary_get(&txn, p_key.as_bytes()).unwrap().unwrap();
        
        data
    }

    pub fn init_db(name: &str) -> Result<Db>{
        println!("> Configuring LATTICE storage network...");
        println!(">> LATTICE INITIALIZED <<");
        let mut db = Db::init_tmp(name).unwrap();
        // Initiate schemas (do this for each struct)
        db.define::<Agents>();

        Ok(db)
    }

}