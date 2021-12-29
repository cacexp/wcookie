use chrono::NaiveDate;
use regex::Regex;
use crate::ParseError;
use chrono::Utc;
use chrono::DateTime;

// Regex for dates Sun, 06 Nov 1994 08:49:37 GMT
const DATE_FORMAT_1123: &str= "(Mon|Tue|Wed|Thu|Fri|Sat|Sun), \
(0[1-9]|[123][0-9]) (Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec) ([0-9]{4}) \
([0-1][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]) GMT";

/// Parses RFC 1123 dates, as defined in [RFC2616 Section 3.3.1](https://datatracker.ietf.org/doc/html/rfc2616#section-3.3.1).
/// 
/// For example,  `Sun, 06 Nov 1994 08:49:37 GMT` date.
pub fn parse_rfc_1123_date(date: &str) -> Result<DateTime<Utc>, ParseError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(DATE_FORMAT_1123).unwrap();
    }

    
    if let Some(captures) = RE.captures(date) {
        // Capture 0 is the full match and  1 is the day of the week name
        let day : u32 = captures.get(2).unwrap().as_str().parse().unwrap();
        let month = match captures.get(3).unwrap().as_str() {
            "Jan" => 1,
            "Feb" => 2,
            "Mar" => 3,
            "Apr" => 4,
            "May" => 5,
            "Jun" => 6,
            "Jul" => 7,
            "Aug" => 8,
            "Sep" => 9,
            "Oct" => 10,
            "Nov" => 11,
            "Dec" => 12,
            _ => return Err(ParseError::new("Invalid date"))
        };

        let year: i32 = captures.get(4).unwrap().as_str().parse().unwrap();

        let hour : u32 = captures.get(5).unwrap().as_str().parse().unwrap();
        let min : u32 = captures.get(6).unwrap().as_str().parse().unwrap();
        let secs : u32 = captures.get(7).unwrap().as_str().parse().unwrap();

        let naive = NaiveDate::from_ymd(year, month, day).and_hms(hour,min,secs);
        return Ok(DateTime::<Utc>::from_utc(naive, Utc));
    } else {
        return Err(ParseError::new("Invalid date"));
    }
}