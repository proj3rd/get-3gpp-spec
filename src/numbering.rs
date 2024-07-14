pub fn get_series(spec: &str) -> Result<&str, String> {
    let err_msg = Err(format!("'{}' should starts with a 2-digit non-negative integer followed by a dot", spec));
    if spec.len() < 3 {
        return err_msg;
    }
    let mut chars = spec.chars();
    let maybe_dot = chars.nth(2);
    if maybe_dot != Some('.') {
        return err_msg;
    }
    let substring = &spec[..2];
    if substring.parse::<u8>().is_err() {
        return err_msg;
    }
    Ok(substring)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_series() {
        assert_eq!(get_series("36.331").unwrap(), "36");
    }
}