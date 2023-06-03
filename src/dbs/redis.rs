pub fn connect_redis() -> redis::Connection {
    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let con = client.get_connection().unwrap();

    con
}
