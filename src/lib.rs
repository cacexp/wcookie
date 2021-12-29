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

//! # HTTP Cookie implementation in Rust
//! 
//! Implementation of the [RFC 6265 Set-Cookie](https://datatracker.ietf.org/doc/html/rfc6265).
//! 
//! After receiving a HTTP response, a server cand send one or more `Set-Cookie` headers. The user agent usually stores these
//! cookies and includes some of them (those which match cookie criteria) inside a `Cookie` header.
//!  
//!  # Tutorial
//!  
//! ## Client-side
//! 
//! ### Receiving `Set-Cookie` headers at HTTP responses
//!  
//!  Servers may incluide one or more `Set-Cookie` headers at HTTP responses, for example:
//! 
//!  `Set-Cookie: id=1213342`
//! 
//!  `Set-Cookie: user=john@smith; Expires=Thu, 31 Oct 2021 07:28:00 GMT; Secure`
//! 
//!  The type [SetCookie](crate::SetCookie) represents a server cookie. To parse a `Set-Cookie` header value it has a `from_string` function:
//! 
//! ```rust
//! use wcookie::SetCookie;
//! use std::str::FromStr;
//! 
//! let cookie_value: &str = "user=john@smith; Expires=Thu, 31 Oct 2021 07:28:00 GMT; Secure";
//! 
//! let cookie = SetCookie::from_str(cookie_value);
//! 
//! assert!(cookie.is_ok());
//! 
//! 
//! ```
//! ### Sending `Cookie` headers in HTTP requests
//! The user agent can check if the cookie can be used in a request with function [SetCookie::use_in_request](crate::SetCookie::use_in_request) passing as params:
//! 
//! * `domain`: request target domain
//! * `path`: request path
//! * `secure`: if HTTPS is used
//! 
//! ```rust
//! use wcookie::SetCookie;
//! use std::str::FromStr;
//! 
//! let cookie = SetCookie::from_str("cookie1=122343; Domain=b.a").unwrap();
//! 
//! assert!(cookie.use_in_request("c.b.a", "/", true));
//! 
//! ```
//! 
//! Note, if the `Set-Cookie` value does not include the `Domain` directive, it should be set the request's host domain:
//!
//! ```rust
//! use wcookie::SetCookie;
//! use std::str::FromStr;
//! 
//! let mut cookie = SetCookie::from_str("cookie1=122343").unwrap();
//!
//! cookie.domain = Some(String::from("b.a"));
//! 
//! assert!(cookie.use_in_request("c.b.a", "/", true));
//! ```
//! 
//! By default, the cookie path, if not set, is `/`.
//! 
//! Note, `use_in_request` makes use of next functions:
//! 
//! * [SetCookie::expired](crate::SetCookie::expired)
//! * [SetCookie::use_in_request_domain](crate::SetCookie::use_in_request_domain)
//! * [SetCookie::use_in_request_path](crate::SetCookie::use_in_request_path)   
//! 
//! A `SetCookie` can be converted into a [Cookie](crate::Cookie) to be incluided in a `Cookie` header:
//! 
//! ```rust
//! use wcookie::SetCookie;
//! use std::str::FromStr;
//! 
//! let set_cookie = SetCookie::from_str("cookie1=122343; Max-Age=12000; Domain=b.a").unwrap();
//! 
//! // Check the cookie can be used in request
//! assert!(set_cookie.use_in_request("c.b.a", "/", true));
//! 
//! let cookie = set_cookie.to_cookie();
//! 
//! assert_eq!(cookie.name.as_str(), "cookie1");
//! assert_eq!(cookie.value.as_str(), "122343");
//! assert_eq!(cookie.to_string(), "cookie1=122343");
//! 
//! ```
//! 
//!  ## Server-side: creating `Set-Cookie` 
//! 
//! At server side, a cookie can be created using the `new` constructor and member values can be set when it is mutable:
//! 
//! ```
//! use wcookie::SetCookie;
//! use chrono::{Utc, TimeZone};
//! 
//! let mut cookie = SetCookie::new("cookie_name", "cookie_value");
//! cookie.domain = Some(String::from("myserver.com"));
//! cookie.expires = Some(Utc.ymd(2014, 7, 8).and_hms(9, 10, 11));
//! 
//! ```
//! 

#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

use std::fmt::Display;
use chrono::Timelike;
use std::fmt;
use chrono::Utc;
use chrono::{DateTime, Datelike};
use std::str::FromStr;
use std::error::Error;
use std::time::{Duration, SystemTime};
use std::ops::Add;
use std::collections::HashMap;
use std::cmp::{PartialEq, Eq};
use std::hash::{Hash, Hasher};


mod rfc_1123;
pub use rfc_1123::parse_rfc_1123_date;


mod rfc_850;
pub use rfc_850::parse_rfc_850_date;

mod asct;
pub use asct::parse_asct_date;


pub(crate) const COOKIE: &str = "cookie";
pub(crate) const COOKIE_EXPIRES: &str = "expires";
pub(crate) const COOKIE_MAX_AGE: &str = "max-age";
pub(crate) const COOKIE_DOMAIN: &str = "domain";
pub(crate) const COOKIE_PATH: &str = "path";
pub(crate) const COOKIE_SAME_SITE: &str = "samesite";
pub(crate) const COOKIE_SAME_SITE_STRICT: &str = "strict";
pub(crate) const COOKIE_SAME_SITE_LAX: &str = "lax";
pub(crate) const COOKIE_SAME_SITE_NONE: &str = "none";
pub(crate) const COOKIE_SECURE: &str = "secure";
pub(crate) const COOKIE_HTTP_ONLY: &str = "httponly";

/// Error type produced while parsing a `Cookie`
#[derive(Debug)]
pub struct ParseError {
    details: String
}

impl ParseError {
    /// Constructor with any type of string
    fn new<S>(msg: S) -> ParseError
    where S: Into<String> {
        ParseError{details: msg.into()}
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Represents a cookie sent at a `Cookie` header at an HTTP Request.
#[derive(Debug, PartialEq)]
pub struct Cookie {
    // Cookie name
    pub name: String,
    // Cookie value as a character string. Binary values can be encoded, for instance, using `Base-64` encoder.
    pub value: String
}

impl Cookie {
    /// `Request Cookie` constructor
    pub fn new<S>(name:S, value: S) -> Cookie 
    where S: Into<String> {
        Cookie {
            name: name.into(),
            value: value.into()
        }
    }
}

impl Display for Cookie {
    /// Writes a cookie with format `name=value`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}={}", &self.name, &self.value)    
    }
}

impl Hash for Cookie {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

/// Enum with `SameSite` possible values for `Set-Cookie` attribute
#[derive(Debug,Copy,Clone,PartialEq)]
pub enum SameSiteValue {Strict, Lax, None}

impl FromStr for SameSiteValue {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            COOKIE_SAME_SITE_STRICT => Ok(SameSiteValue::Strict),
            COOKIE_SAME_SITE_LAX => Ok(SameSiteValue::Lax),
            COOKIE_SAME_SITE_NONE => Ok(SameSiteValue::None),
            _ => Err(
                ParseError::new(format!("Invalid SameSite cookie directive value: {}", s)))
        }

    }
}

/// Represents a cookie created from `Set-Cookie` response header. 
/// 
/// A `SetCookie` can be parsed from the `Set-Cookie` value from an HTTP `Response` using the trait `FromStr`:
///
/// ```no_run
///  use wcookie::SetCookie;
///  use std::str::FromStr;
/// 
///  let cookie = SetCookie::from_str("id=a3fWa; Expires=Wed, 21 Oct 2022 07:28:00 GMT");
/// 
///  ```
/// 
/// or, constructed with the `new` function and setting some of its public members:
/// 
/// ```rust
/// use wcookie::SetCookie;
/// use std::time::Duration;
/// 
/// let mut cookie = SetCookie::new("id", "234r34");
/// cookie.max_age = Some(Duration::from_secs(12000));
/// 
/// ```
/// 
/// See [RFC6265 Set-Cookie](https://datatracker.ietf.org/doc/html/rfc6265#section-4.2) for more information.
/// 

#[derive(Debug,Clone)]
pub struct SetCookie {
    /// Cookie name
    pub name: String,
    /// Cookie value
    pub value: String,
    /// Cookie domain, by default is the originating domain of the request
    pub domain: Option<String>,
    /// Cookie path, by default, it is the request's path
    pub path: Option<String>,
    /// When the Cookie expires, if None, it does not expire.
    /// This value is obtained from Max-Age and Expires attributes (Max-Age has precedence)
    pub expires: Option<DateTime<Utc>>,
    /// Max-age
    pub max_age: Option<Duration>,
    /// Time there the cookie was received/create to calculate later `expire_time`
    pub(crate) created: SystemTime,
    /// Internal expires time from expires and max_age
    /// Cookie same site value (option)
    pub same_site: SameSiteValue,
    /// Cookie requires HTTPS
    pub secure: bool,
    /// Browsers does not allow Javascript access to this cookie
    pub http_only: bool,
    /// Other Set-Cookie extensions
    pub extensions: HashMap<String, Option<String>>    
}


impl SetCookie { 
    /// Constructor with mandatory fiels `name` and `value`
    pub fn new<S>(name: S, value: S) -> SetCookie 
    where S : Into<String> {
        SetCookie {
            name: name.into(),
            value: value.into(),
            domain: None,
            path: None,
            expires: None,
            max_age: None,
            created: SystemTime::now(),            
            same_site: SameSiteValue::Lax,
            secure: false,
            http_only: false,
            extensions: HashMap::new()
        }
    }

    /// Generates a [Cookie](crate::Cookie) to be used in an HTTP Request
    pub fn to_cookie (& self) -> Cookie {
        Cookie {
            name: self.name.clone(),
            value: self.value.clone()
        }
    }

    /// Gets the cookie path or the detault value which is `\"/\"`
    pub fn path_or_default(&self) -> &str {
        self.path.as_deref().unwrap_or("/")
    }

    /// Gets the local `SystemTime` when the cookie expires if any. Returns `None` if the 
    /// cookie never expires.
    /// 
    /// This value is get from `Expires` and `Max-Age`params. When both params are set, 
    /// `Max-Age` has precedence.
    pub fn expire_time(&self) -> Option<SystemTime> {
        if let Some(duration) = self.max_age {
            return Some(self.created.add(duration));
        }        
        if let Some(date) = self.expires {
            let time = date.timestamp();
            if let Ok(utime) = u64::try_from(time) {
                return Some(SystemTime::UNIX_EPOCH.add(Duration::from_secs(utime)));
            } else { // Time before UNIX Epoch, it is expired
                return Some(self.created.clone())
            }
        }
        return None
    }

    /// Checks if the cookie is expired.
    /// 
    /// `Max-Age` time is assumed from the moment the cookie was parsed or created
    pub fn expired(&self) -> bool {
        if let Some(expires) = self.expire_time() {
            let now = SystemTime::now();
            return expires < now;
        }
        return false;
    }
    

    /// Checks if the request path match the cookie path. 
    /// 
    /// Using [RFC6265 Section 5.1.4](https://datatracker.ietf.org/doc/html/rfc6265#section-5.1.4) Algorithm.
    /// 
    /// Note if field `path` is not set, this function  return always `false`
    pub fn use_in_request_path(&self, path: &str) -> bool {
                
        let cookie_path = self.path_or_default();

        let cookie_path_len = cookie_path.len();
        let request_path_len = path.len();
 
       
        if !path.starts_with(cookie_path) {
            // A. cookie path is a prefix of request path
            return false;
        }
    
        return
            // 1. They are identical, or 
            request_path_len == cookie_path_len 
            // 2. A and cookie path ends with an slash
            || cookie_path.chars().nth(cookie_path_len - 1).unwrap() == '/' 
            // 3. A and the first char of request path that is not incled in request path is an slash
            || path.chars().nth(cookie_path_len).unwrap() == '/'; 
    }

    /// Checks if the cookie can be sent to the `request_domain`. 
    /// 
    /// Using [RFC6265 Section 4.1.1.3](https://datatracker.ietf.org/doc/html/rfc6265#section-4.1.2.3).
    /// > For example, if the value of the Domain attribute is "example.com", 
    /// > the user agent will include the cookie in the Cookie header when making HTTP requests 
    /// > to example.com, www.example.com, and www.corp.example.com.
    /// 
    /// Note: if field `domain` is not set, this function return always `false`
    pub fn use_in_request_domain(&self, request_domain: &str) -> bool {
        if self.domain.is_none() {
            return false;
        }

        let cookie_domain = self.domain.as_deref().unwrap();
        if let Some(index) = request_domain.rfind(cookie_domain) {
            if index == 0 { // same domain
                return true;
            }
            // The cookie domain is a subdomain of request domain, acccept
            return request_domain.chars().nth(index-1).unwrap() == '.';
        }
         
        return false;
    }

    /// Checks if the cookie can be used on this request
    pub fn use_in_request(&self, request_domain: &str, request_path: &str, secure: bool) -> bool {

        if self.domain.is_none() {
            return false;
        }

        // Match Secure restrictions 

        if self.secure && !secure {
            return false;
        }
    
        // Strict behaviour: it is only same-site if the domain is the same

        if self.same_site == SameSiteValue::Strict && self.domain.as_deref().unwrap() != request_domain {
            return false;
        }

        // Lax behaviour: allow cross-site from subdomain to father domain
        if self.same_site == SameSiteValue::Lax && !self.use_in_request_domain(request_domain) {
            return false;
        }

        // None: allow all cookies transfer but only it HTTPS is in use
        if self.same_site == SameSiteValue::None && ! self.secure {
            return false;
        }

        // PATH filteringseconds

        return self.use_in_request_path(request_path);      
    } 
}

impl PartialEq for SetCookie {
    fn eq(&self, other: &Self) -> bool {
        return self.name == other.name &&
               self.value == other.value &&
               self.domain == other.domain && 
               self.path == other.path
    }    
}

impl Eq for SetCookie{}

impl FromStr for SetCookie {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(';');
     
            return if let Some(slice) = components.next() {
                let (key, value) = parse_cookie_value(slice)?;
                let mut cookie = SetCookie::new(key, value);
                while let Some(param) = components.next() {
                    let directive = CookieDirective::from_str(param)?;
                    match directive {
                        CookieDirective::Expires(date) =>
                           cookie.expires = Some(date),
                        CookieDirective::MaxAge(seconds) => 
                           cookie.max_age = Some(seconds),
                        CookieDirective::Domain(url) =>  // starting dot is ignored                      
                           cookie.domain = Some(if let Some(stripped) = url.as_str().strip_prefix(".") {
                               String::from(stripped)
                           } else {
                               url
                           }),
                        CookieDirective::Path(path) => cookie.path = Some(path),
                        CookieDirective::SameSite(val) => cookie.same_site = val,
                        CookieDirective::Secure => cookie.secure = true,
                        CookieDirective::HttpOnly => cookie.http_only = true,
                        CookieDirective::Extension(name, value) => {
                            let _res = cookie.extensions.insert(name, value);
                        }
                    }
                }         
                Ok(cookie)
            } else {
                if CookieDirective::from_str(s).is_ok() {
                    return Err(ParseError::new("Cookie has not got name/value"));
                };
    
                let (key, value) = parse_cookie_value(s)?;            
                Ok(SetCookie::new(key, value))
            }
        }
}

impl Hash for SetCookie {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.domain.hash(state);
    }
}



const MONTH_NAME: [&'static str; 12] = ["Jan" , "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

impl fmt::Display for SetCookie {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}={}", self.name, self.value)?;
        if let Some(ref domain) =  self.domain {
            write!(f, ", Domain={}", domain)?;
        }
        if let Some(ref path) =  self.path {
            write!(f, ", Path={}", path)?;
        }

        if let Some(duration) = self.max_age {
            write!(f, ", Max-Age={}", duration.as_secs())?;
        }else if let Some(ref date) = self.expires {
            write!(f, ", Expires={}, {:02}-{}-{} {:02}:{:02}:{:02} GMT",
                   date.weekday(), date.day(), MONTH_NAME[(date.month()-1) as usize], date.year(),
                   date.hour(), date.minute(), date.second())?;
        } 
        match self.same_site {
            SameSiteValue::None => write!(f, ", SameSite=None")?,
            SameSiteValue::Strict => write!(f, ", SameSite=Strict")?,
            _ => {}
        };

        if self.secure {
            write!(f, ", Secure")?;
        }

        if self.http_only {
            write!(f, ", HttpOnly")?;
        }

        for (key, value) in &self.extensions {
            if let Some(val) = value {
                write!(f, ", {}={}", key, val)?;
            } else {
                write!(f, ", {}", key)?;
            }
        }

        return Ok(());
        
    }
}

/// Helper function to parse the `Cookie` name and value
pub(crate) fn parse_cookie_value(cookie: &str) -> Result<(String, String), ParseError>{
    if let Some(index) = cookie.find('=') {
        let key = String::from(cookie[0..index].trim());
        let value = String::from(cookie[index + 1..].trim());
        if value.len() == 0 {
            return Err(ParseError::new("Cookie value must not be empty"));
        }
        return Ok((key, value));
    } else {
        return Err(ParseError::new(format!("Malformed HTTP cookie: {}", cookie)));
    }
}

/// Helper enum to parse directives and set up the `Cookie` values
enum CookieDirective {
    Expires(DateTime<Utc>),
    MaxAge(Duration),
    Domain(String),
    Path(String),
    SameSite(SameSiteValue),
    Secure,
    HttpOnly,
    Extension(String, Option<String>)
}




/// Helper function to parse `CookieDirective`
impl FromStr for CookieDirective {
    
    type Err = ParseError;

    fn from_str(s: &str) -> Result<CookieDirective, ParseError> {
        if let Some(index) = s.find('=') { // Cookie param with value
            let key = s[0..index].trim().to_ascii_lowercase();
            let value = s[index + 1..].trim();
            if value.len() == 0 {
                return Err(ParseError::new(format!("Directive {} value must not be empty", key)));
            }
            return match key.as_str() {
                COOKIE_EXPIRES => {
                    let expires = parse_rfc_1123_date(value)
                        .or_else(|_| parse_rfc_850_date(value))
                        .or_else(|_| parse_asct_date(value))?; 

                    Ok(CookieDirective::Expires(expires))
                },
                COOKIE_MAX_AGE => {  // Max-age value in seconds
                    let digit = u64::from_str(value)
                        .or_else(|_|  {
                            Err(ParseError::new("Cannot parse Max-age"))
                        })?;
                    Ok(CookieDirective::MaxAge(Duration::from_secs(digit)))
                },
                COOKIE_DOMAIN => {
                    Ok(CookieDirective::Domain(String::from(value)))
                },
                COOKIE_PATH => {
                    Ok(CookieDirective::Path(String::from(value)))
                }
                COOKIE_SAME_SITE => {
                    let lower_case = value.to_ascii_lowercase();
                    match SameSiteValue::from_str(lower_case.as_str()) {
                        Ok(site_value) => Ok(CookieDirective::SameSite(site_value)),
                        Err(e) => Err(e)
                    }
                },
                _ => Ok(CookieDirective::Extension(key, Some(value.to_string())))
            }
        } else {
            let directive = s.trim().to_ascii_lowercase();

            match directive.as_str() {
                COOKIE_SECURE => Ok(CookieDirective::Secure),
                COOKIE_HTTP_ONLY => Ok(CookieDirective::HttpOnly),
                COOKIE_DOMAIN | COOKIE_EXPIRES | COOKIE_MAX_AGE | COOKIE_PATH |COOKIE_SAME_SITE => Err(ParseError::new(format!("Directive {} needs a value", directive))),
                _ => Ok(CookieDirective::Extension(directive, None))
            }
        }
    }
}

#[cfg(test)]
mod test;