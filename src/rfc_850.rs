use chrono::NaiveDate;
use regex::Regex;
use crate::ParseError;
use chrono::Utc;
use chrono::DateTime;

/// Regex for dates such as Monday, 12-Feb-2022 10:20:00 GMT
const DATE_FORMAT_850: &str= "(Monday|Tuesday|Wednesday|Thursday|Friday|Saturday|Sunday|Mon|Tue|Wed|Thu|Fri|Sat|Sun), \
(0[1-9]|[123][0-9])-(Jan|Feb|Mar|Apr|May|Jun|Jul|Aug|Sep|Oct|Nov|Dec)-([0-9]{4}|[0-9]{2}) \
([0-1][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9]) GMT";

/// Parses RFC 850 dates, with extension, as defined in [RFC2616 Section 3.3.1](https://datatracker.ietf.org/doc/html/rfc2616#section-3.3.1).
/// 
/// For example,  `Wed, 15-Nov-23 09:13:29 GMT` or `Sunday, 06-Nov-94 08:49:2037 GMT` dates.
/// 
pub fn parse_rfc_850_date(date: &str) -> Result<DateTime<Utc>, ParseError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(DATE_FORMAT_850).unwrap();
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

        let mut year: i32 = captures.get(4).unwrap().as_str().parse().unwrap();
        // Fix millenium, for 2 digit year
        year+= if year < 70 {2000} else if year < 100 {1900} else {0};

        let hour : u32 = captures.get(5).unwrap().as_str().parse().unwrap();
        let min : u32 = captures.get(6).unwrap().as_str().parse().unwrap();
        let secs : u32 = captures.get(7).unwrap().as_str().parse().unwrap();

        let naive = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or(ParseError::new("Invalid date"))?
            .and_hms_opt(hour,min,secs)
            .ok_or(ParseError::new("Invalid date"))?;

        return Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc));
    } else {
        return Err(ParseError::new("Invalid date"));
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;
    use chrono::{Utc, DateTime};
    use crate::rfc_850;

    lazy_static! {
        static ref RIGHT_DATE1: DateTime<Utc> = {
            let naive =
            NaiveDate::from_ymd_opt(2023,11,15).unwrap().and_hms_opt(9,13,29).unwrap();
            DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)
        };
        static ref RIGHT_DATE2: DateTime<Utc> = {
            let naive =
            NaiveDate::from_ymd_opt(2023,11,8).unwrap()
            .and_hms_opt(9,13,29).unwrap();
            DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc)
        };
    }

    #[test]
    fn test_date1() {
        let str_date = "Wednesday, 15-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(*RIGHT_DATE1, date);
    }
   
    #[test]
    fn test_date2() {
        let str_date = "Wednesday, 08-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(*RIGHT_DATE2, date);
    }

    #[test]
    fn test_date3() {
        let str_date = "Wednesday, 15-Nov-2023 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(*RIGHT_DATE1, date);
    }

    #[test]
    fn test_date4() {
        let str_date = "Wednesday, 08-Nov-2023 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(*RIGHT_DATE2, date);
    }

    #[test]
    fn test_date5() {
        let str_date = "Wed, 15-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(*RIGHT_DATE1, date);
    }

    #[test]
    fn test_date6() {
        let str_date = "Wed, 08-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(*RIGHT_DATE2, date);
    }

    #[test]
    fn test_date7() {
        let str_date = "Wed, 15-Nov-2023 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(*RIGHT_DATE1, date);
    }

    #[test]
    fn test_date8() {
        let str_date = "Wed, 08-Nov-2023 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(*RIGHT_DATE2, date);
    }

    #[test]
    fn test_error_date1() {
        let str_date = "Wednesday, 15-Nov-23 29:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_err());

    }

    #[test]
    fn test_error_date2() {
        let str_date = "Wednesday, 15-Nov-23 09:73:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_err());

    }

    #[test]
    fn test_error_date3() {
        let str_date = "Wednesday, 15-Nov-23 09:13:99 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_err());

    }

    #[test]
    fn test_error_date4() {
        let str_date = "Wednesday, 15-Nov-23 09:13:29 GNT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_err());

    }

    #[test]
    fn test_day1() {
        let naive =
        NaiveDate::from_ymd_opt(2023,11,13).unwrap()
        .and_hms_opt(9,13,29).unwrap();

        let date_right = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

        let str_date = "Mon, 13-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(date_right, date);

    }

    #[test]
    fn test_day2() {
        let naive =
        NaiveDate::from_ymd_opt(2023,11,14).unwrap().and_hms_opt(9,13,29).unwrap();

        let date_right = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

        let str_date = "Tue, 14-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(date_right, date);

    }
    #[test]
    fn test_day3() {
        let naive =
        NaiveDate::from_ymd_opt(2023,11,15).unwrap().and_hms_opt(9,13,29).unwrap();

        let date_right = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

        let str_date = "Wed, 15-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(date_right, date);

    }
    #[test]
    fn test_day4() {
        let naive =
        NaiveDate::from_ymd_opt(2023,11,16).unwrap().and_hms_opt(9,13,29).unwrap();

        let date_right = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

        let str_date = "Thu, 16-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(date_right, date);

    }
    #[test]
    fn test_day5() {
        let naive =
        NaiveDate::from_ymd_opt(2023,11,17).unwrap().and_hms_opt(9,13,29).unwrap();

        let date_right = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

        let str_date = "Fri, 17-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(date_right, date);

    }
    #[test]
    fn test_day6() {
        let naive =
        NaiveDate::from_ymd_opt(2023,11,18).unwrap().and_hms_opt(9,13,29).unwrap();

        let date_right = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

        let str_date = "Sat, 18-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(date_right, date);

    }
    #[test]
    fn test_day7() {
        let naive =
        NaiveDate::from_ymd_opt(2023,11,13).unwrap().and_hms_opt(9,13,29).unwrap();

        let date_right = DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc);

        let str_date = "Sun, 13-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_ok());

        let date = result.unwrap();

        assert_eq!(date_right, date);
    }

    #[test]
    fn test_wrong_day1() {
        // Dates are case sensitive
        let str_date = "mon, 13-Nov-23 09:13:29 GMT";

        let result = rfc_850::parse_rfc_850_date(str_date);

        assert!(result.is_err());

    }
    
}