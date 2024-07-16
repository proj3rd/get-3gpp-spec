#[derive(PartialEq, Debug)]
pub struct Version {
    major: u8,
    minor: u8,
    editorial: u8,
}

pub fn parse_version(version_string: &str) -> Result<Version, String> {
    const VERSION_CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyz";
    let err_msg = Err(format!("'{version_string}' should be a 3-letter alphanumeric string or 6-digit non-negative integer"));
    match version_string.len() {
        3 => {
            let version_string_lower = version_string.to_lowercase();
            let mut version_chars = version_string_lower.chars();
            let mut current = version_chars.next().unwrap();
            let major = VERSION_CHARS.chars().position(|c| c == current);
            current = version_chars.next().unwrap();
            let minor = VERSION_CHARS.chars().position(|c| c == current);
            current = version_chars.next().unwrap();
            let editorial = VERSION_CHARS.chars().position(|c| c == current);
            if major == None || minor == None || editorial == None {
                return err_msg;
            }
            Ok(Version {
                major: u8::try_from(major.unwrap()).unwrap(),
                minor: u8::try_from(minor.unwrap()).unwrap(),
                editorial: u8::try_from(editorial.unwrap()).unwrap(),
            })
        }
        6 => {
            let major = version_string[0..2].parse::<u8>();
            let minor = version_string[2..4].parse::<u8>();
            let editorial = version_string[4..6].parse::<u8>();
            if major.is_err() || minor.is_err() || editorial.is_err() {
                return err_msg;
            }
            Ok(Version {
                major: major.unwrap(),
                minor: minor.unwrap(),
                editorial: editorial.unwrap(),
            })
        }
        _ => err_msg,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_version() {
        assert_eq!(
            parse_version("09i").unwrap(),
            Version {
                major: 0,
                minor: 9,
                editorial: 18
            }
        );
        assert_eq!(
            parse_version("012345").unwrap(),
            Version {
                major: 1,
                minor: 23,
                editorial: 45
            }
        );
    }
}
