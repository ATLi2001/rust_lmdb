use lmdb;
use lmdb::{Transaction, WriteFlags};
use time::precise_time_ns;

fn main() {
  let name = "lmdb_storage";
  if !std::path::Path::new(&name.clone()).exists() {
    let _ = std::fs::create_dir(name.clone()).unwrap();
  }
  let dbenv = lmdb::Environment::new()
            .set_map_size(100 * 1024 * 1024 * 1024)
            .open(std::path::Path::new(&name.clone()))
            .unwrap();
  let default_db = dbenv.open_db(None).unwrap();

  for i in 0..100 {
    println!("{}", precise_time_ns());

    let key = Vec::from(i.to_string());
    let value = Vec::from(i.to_string());

    let mut txn = dbenv.begin_rw_txn().unwrap();
    txn.put(default_db, &key, &value, WriteFlags::empty()).expect("put failed");
    let _ = txn.commit();
  }
  println!("{}", precise_time_ns());
}