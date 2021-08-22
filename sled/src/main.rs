use sled;

fn main() {
    let db: sled::Db = sled::open("db").unwrap();
    assert_eq!(db.insert(&[1,2,3], vec![0]), Ok(None));
    assert_eq!(db.insert(&[1,2,3], vec![1]), Ok(Some(sled::IVec::from(&[0]))));
}
