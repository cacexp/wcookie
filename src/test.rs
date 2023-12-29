// Copyright 2021 Juan A. Cáceres (cacexp@gmail.com)
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::*;
use chrono::{DateTime, Utc, NaiveDate};
use std::time::Duration;

macro_rules! assert_invalid_data {
    ($a: expr) => {
        assert!($a.is_err());
    };
    ($a: expr, $b: expr) => {
        assert!($a.is_err());

        let error = $a.err().unwrap();
        assert_eq!(error.to_string(), $b);
    };
}

#[test]
fn test_parse_cookie_value_right1() {
    let right = "name=value";
    let result = parse_cookie_value(right);

    assert!(result.is_ok());

    let (key, value) = result.unwrap();

    assert_eq!(key.as_str(), "name");
    assert_eq!(value.as_str(), "value");
}

#[test]
fn test_parse_cookie_value_right2() {
    let right = "  name=value";
    let result = parse_cookie_value(right);

    assert!(result.is_ok());

    let (key, value) = result.unwrap();

    assert_eq!(key.as_str(), "name");
    assert_eq!(value.as_str(), "value");
}

#[test]
fn test_parse_cookie_value_right3() {
    let right = "  name=value ";
    let result = parse_cookie_value(right);

    assert!(result.is_ok());

    let (key, value) = result.unwrap();

    assert_eq!(key.as_str(), "name");
    assert_eq!(value.as_str(), "value");
}

#[test]
fn test_parse_cookie_value_right4() {
    let right = "  name=value ";
    let result = parse_cookie_value(right);

    assert!(result.is_ok());

    let (key, value) = result.unwrap();

    assert_eq!(key.as_str(), "name");
    assert_eq!(value.as_str(), "value");
}

#[test]
fn test_parse_cookie_value_wrong1() {
    let wrong = "name:value";
    let wrong_message = format!("Malformed HTTP cookie: {}", wrong);

    let result = parse_cookie_value(wrong);

    assert_invalid_data!(result, wrong_message);
}

#[test]
fn test_parse_cookie_value_wrong2() {
    let wrong = "name";
    let wrong_message = format!("Malformed HTTP cookie: {}", wrong);

    let result = parse_cookie_value(wrong);

    assert_invalid_data!(result, wrong_message);
}

#[test]
fn test_parse_cookie_samesite_right1() {
    let right = "SameSite=Strict";
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());

    let directive = result.unwrap();

    if let CookieDirective::SameSite(value) = directive {
        assert_eq!(value, SameSiteValue::Strict);
    } else {
        assert!(false, "Expected CookieDirective::SameSite");
    }
}

#[test]
fn test_parse_cookie_samesite_right2() {
    let right = "SameSite=Lax";
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());

    let directive = result.unwrap();

    if let CookieDirective::SameSite(value) = directive {
        assert_eq!(value, SameSiteValue::Lax);
    } else {
        assert!(false, "Expected CookieDirective::SameSite");
    }
}

#[test]
fn test_parse_cookie_samesite_right3() {
    let right = "SameSite=None";
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());

    let directive = result.unwrap();

    if let CookieDirective::SameSite(value) = directive {
        assert_eq!(value, SameSiteValue::None);
    } else {
        assert!(false, "Expected CookieDirective::SameSite");
    }
}

#[test]
fn test_parse_cookie_samesite_right4() {
    let right = "SameSite=lax";
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());

    let directive = result.unwrap();

    if let CookieDirective::SameSite(value) = directive {
        assert_eq!(value, SameSiteValue::Lax);
    } else {
        assert!(false, "Expected CookieDirective::SameSite");
    }
}

#[test]
fn test_parse_cookie_samesite_right5() {
    let right = "sameSite=Lax";
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());

    let directive = result.unwrap();

    if let CookieDirective::SameSite(value) = directive {
        assert_eq!(value, SameSiteValue::Lax);
    } else {
        assert!(false, "Expected CookieDirective::SameSite");
    }
}

#[test]
fn test_parse_cookie_samesite_wrong1() {
    let right = "SameSite=Void";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_samesite_wrong2() {
    let right = "SameSite";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_samesite_wrong3() {
    let right = "SameSite=";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_expires_right1() {
    let right = "Expires=Sun, 06 Nov 1994 08:49:37 GMT";
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());

    if let CookieDirective::Expires(date) = result.unwrap() {
        let naive =
            NaiveDate::from_ymd_opt(1994,11,6).unwrap().and_hms_opt(8,49,37).unwrap();
        let time = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
       
        assert_eq!(date, time);
    } else {
        assert!(false, "Expected CookieDirective::Expires");
    }
}

#[test]
fn test_parse_cookie_expires_right2() {
    let right = "Expires=Sunday, 06-Nov-1994 08:49:37 GMT";
    let result = CookieDirective::from_str(right);
   
    assert!(result.is_ok());

    if let CookieDirective::Expires(date) = result.unwrap() {
        let naive =
            NaiveDate::from_ymd_opt(1994,11,6).unwrap()
            .and_hms_opt(8,49,37).unwrap();
        let time = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
        
        assert_eq!(date, time);
    } else {
        assert!(false, "Expected CookieDirective::Expires")
    }
}

#[test]
fn test_parse_cookie_expires_right3() {
    let right = "Expires=Sun Nov 6 08:49:37 1994";
    let result = CookieDirective::from_str(right);

    if result.is_err() {
        println!("{}", result.as_ref().err().unwrap());
    }
    assert!(result.is_ok());

    if let CookieDirective::Expires(date) = result.unwrap() {
        let naive =
            NaiveDate::from_ymd_opt(1994,11,06).unwrap().and_hms_opt(8,49,37).unwrap();
        let time = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
       
        assert_eq!(date, time);
    } else {
        assert!(false, "Expected CookieDirective::Expires");
    }
}

#[test]
fn test_parse_cookie_expires_right4() {
    let right = "Expires=Wed, 15-Nov-2023 09:13:29 GMT";
    let result = CookieDirective::from_str(right);

    if result.is_err() {
        println!("{}", result.as_ref().err().unwrap());
    }
    assert!(result.is_ok());

    if let CookieDirective::Expires(date) = result.unwrap() {
        let naive =
            NaiveDate::from_ymd_opt(2023,11,15).unwrap().and_hms_opt(9,13,29).unwrap();
        let time = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);
       
        assert_eq!(date,time);
    } else {
        assert!(false, "Expected CookieDirective::Expires");
    }
}

#[test]
fn test_parse_cookie_expires_wrong1() {
    let right = "Expires=21 Octubre 2015 07:28:00 UTC";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_expires_wrong2() {
    let right = "Expires=21 October 2015 07:28:00 +0200";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_expires_wrong3() {
    let right = "Expires=Sunday, 06-Nov-94 08:49:37 UTC";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}
#[test]
fn test_parse_cookie_expires_wrong4() {
    let right = "Expires=Sunday, 06-Nov-94 08:49:37 +0200";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_max_age_right1() {
    let right = "Max-Age=3600";
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());

    if let CookieDirective::MaxAge(seconds) = result.unwrap() {
        assert_eq!(seconds, Duration::from_secs(3600));
    } else {
        panic!()
    }
}

#[test]
fn test_parse_cookie_max_age_right2() {
    let right = "Max-Age=0";
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());

    if let CookieDirective::MaxAge(seconds) = result.unwrap() {
        assert_eq!(seconds, Duration::from_secs(0));
    } else {
        panic!()
    }
}

#[test]
fn test_parse_cookie_max_age_right3() {
    let right = "max-Age=1200";  // attributes are case-insensitive
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());

    if let CookieDirective::MaxAge(seconds) = result.unwrap() {
        assert_eq!(seconds, Duration::from_secs(1200));
    } else {
        panic!()
    }
}


#[test]
fn test_parse_cookie_max_age_wrong1() {
    let right = "Max-Age=21 Octubre 2015 07:28:00 UTC";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_max_age_wrong2() {
    let right = "Max-Age=-1200";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_max_age_wrong3() {
    let right = "Max-Age";
    let result = CookieDirective::from_str(right);

    assert!(result.is_err());
}

#[test]
fn test_parse_cookie_max_age_wrong5() {
    let right = "Max-Age=1A200";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_domain_right1() {
    let right = "Domain=example.com";  // attributes are case-insentitive
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());
}

#[test]
fn test_parse_cookie_domain_right2() {
    let right = "domain=example.com";  // attributes are case-insentitive
    let result = CookieDirective::from_str(right);

    assert!(result.is_ok());
}

#[test]
fn test_parse_cookie_domain_wrong1() {
    let right = "Domain";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_parse_cookie_domain_wrong2() {
    let right = "Domain=";
    let result = CookieDirective::from_str(right);

    assert_invalid_data!(result);
}

#[test]
fn test_cookie_match1() {
    let cookie1 = SetCookie::from_str("cookie1=122343; Domain=b.a");
    assert!(cookie1.is_ok());
    assert!(cookie1.unwrap().use_in_request_domain("b.a"));
}

#[test]
fn test_cookie_match2() {
    let cookie1 = SetCookie::from_str("cookie1=122343; Domain=b.a");
    assert!(cookie1.is_ok());
    assert!(cookie1.unwrap().use_in_request_domain("c.b.a"));
}

#[test]
fn test_cookie_match3() {
    let cookie1 = SetCookie::from_str("cookie1=122343; Domain=b.a");
    assert!(cookie1.is_ok());
    assert!(cookie1.unwrap().use_in_request_domain("d.c.b.a"));
}

#[test]
fn test_cookie_match4() {
    let cookie1 = SetCookie::from_str("cookie1=122343; Domain=b.a");
    assert!(cookie1.is_ok());
    assert!(!cookie1.unwrap().use_in_request_domain("xb.a"));
}
#[test]
fn test_cookie_match5() {
    let cookie1 = SetCookie::from_str("cookie1=122343; Domain=b.a");
    assert!(cookie1.is_ok());
    assert!(!cookie1.unwrap().use_in_request_domain("x.a"));
}

#[test]
fn test_cookie_match6() {
    let cookie1 = SetCookie::from_str("cookie1=122343; Domain=c.b.a");
    assert!(cookie1.is_ok());
    assert!(!cookie1.unwrap().use_in_request_domain("b.a"));
}

#[test]
fn test_cookie_new1() {
    let mut cookie1 = SetCookie::new("cookie1", "1222343");
    cookie1.domain = Some(String::from("b.a"));

    let cookie2 = SetCookie::from_str("cookie1=1222343; domain=b.a").unwrap();

    assert_eq!(&cookie1, &cookie2);
}

#[test]
fn test_cookie_eq1() {
    let cookie1 = Cookie::new("cookie1", "1222343");
    let cookie2 = Cookie::from_str("cookie1=1222343").unwrap();
    assert_eq!(&cookie1, &cookie2);
}

#[test]
fn test_cookie_eq2() {
    let cookie1 = Cookie::new("cookie1", "1222343");
    let cookie2 = Cookie::from_str("cookie=1222343").unwrap();
    assert_ne!(&cookie1, &cookie2);
}

#[test]
fn test_cookie_eq3() {
    let cookie1 = Cookie::new("Cookie1", "1222343");
    let cookie2 = Cookie::from_str("cookie1=1222343").unwrap();
    assert_ne!(&cookie1, &cookie2);
}

#[test]
fn test_cookie_eq4() {
    let cookie1 = Cookie::new("cookie1", "122234");
    let cookie2 = Cookie::from_str("cookie1=1222343").unwrap();
    assert_ne!(&cookie1, &cookie2);
}