use std::cell::RefCell;

use futures::{future, prelude::*};
use redis::{aio::MultiplexedConnection, RedisResult};

async fn test_cmd(mut con: &RefCell<MultiplexedConnection>, i: i32) -> RedisResult<()> {
    let key = format!("key{}", i);
    let key2 = format!("key{}_2", i);
    let value = format!("foo{}", i);

    redis::cmd("SET")
        .arg(&key[..])
        .arg(&value)
        .query_async(*con.get_mut())
        .await?;

    // if let Some(c) = Arc::get_mut(&mut con) {
    //     redis::cmd("SET")
    //     .arg(&[&key2, "bar"])
    //     .query_async(c)
    //     .await?;
    //     println!("##########> {}", Arc::strong_count(&con));
    // }

    // if let Some(c) = Arc::get_mut(&mut con) {
    //     redis::cmd("MGET")
    //     .arg(&[&key, &key2])
    //     .query_async(c)
    //     .await?;
    // }

    Ok(())
}

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();

    let mut con = RefCell::new(client.get_multiplexed_tokio_connection().await.unwrap());

    let cmds = (0..3).map(|i| test_cmd(&con, i));

    let result = future::try_join_all(cmds).await.unwrap();

    assert_eq!(3, result.len());

    // let s = r#"
    // local index = KEYS[1]
    // local size = KEYS[2]

    // local roomids = redis.call("LRANGE", "roomid_list", index, size)
    // local res = redis.call("HMGET", "room_dict", unpack(roomids))
    // return {roomids, res}
    // "#;

    // let script = redis::Script::new(s);
    // println!("{:?}", script.get_hash());
    // let result: Vec<Vec<String>> = script
    //     .key(0)
    //     .key(-1)
    //     .invoke_async(con.get_mut())
    //     .await.unwrap();
    // println!("{:?}", result);
    // assert_eq!(result, 3);

    // let script1 = redis::Script::new(
    //     "return redis.call('SET', KEYS[1], ARGV[1])",
    // );
    // let script2 = redis::Script::new("return redis.call('GET', KEYS[1])");
    // let v1: String = script1
    //     .key("key1")
    //     .arg("foo")
    //     .invoke_async(con.get_mut())
    //     .await.unwrap();

    // let v2: String = script2
    //     .key("key1")
    //     .invoke_async(con.get_mut())
    //     .await.unwrap();

    // println!("{:?} {:?}", v1, v2);
}
