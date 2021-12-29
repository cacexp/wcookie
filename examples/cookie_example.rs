use wcookie::{SetCookie};
use std::str::FromStr;

pub fn main() {

    let mut cookie = SetCookie::from_str("id=a3fWa; Expires=Wed, 26 Jan 2022 07:28:00 GMT; Secure").unwrap();

    cookie.domain = Some(String::from("example.com"));

    assert!(cookie.use_in_request_domain("www.example.com"));

    let request_cookie = cookie.to_cookie();

    println!("Set-Cookie: {}", cookie);

    println!("Cookie: {}", request_cookie);
}
