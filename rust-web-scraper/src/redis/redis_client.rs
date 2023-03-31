extern crate redis;
use redis::{Client, Commands};

pub fn try_redis() -> redis::RedisResult<i32> {
    let client = Client::open("redis://127.0.0.1")?;
    let mut con = client.get_connection()?;
    let _: () = con.set("my_key", 42)?;

    let res: i32 = con.get("my_key")?;
    println!("Redis test: {}", res);
    Ok(res)
}
