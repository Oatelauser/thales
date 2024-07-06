use std::time::SystemTime;

#[inline]
pub fn get_timestamp_sec() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
        .expect("Cannot get timestamp from SystemTime").as_secs()
}

#[inline]
pub fn get_timestamp_millis() -> u64 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)
        .expect("Cannot get timestamp from SystemTime").as_millis() as u64
}

#[test]
fn t() {
    println!("{}", get_timestamp_sec());
}