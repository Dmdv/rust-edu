fn is_ipv4(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();

    if parts.len() != 4 {
        return false;
    }

    parts.into_iter().all(|part| {
        match part.parse::<i32>() {
            Ok(num) => num >= 0 && num <= 255 && (!part.starts_with('0') || part.len() == 1),
            Err(_) => false,
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_ipv4() {
        assert_eq!(is_ipv4("192.168.0.1"), true);
        assert_eq!(is_ipv4("255.255.255.255"), true);
        assert_eq!(is_ipv4("0.0.0.0"), true);
        assert_eq!(is_ipv4("256.1.2.3"), false);
        assert_eq!(is_ipv4("1.2.3"), false);
        assert_eq!(is_ipv4("1.2.3.4.5"), false);
        assert_eq!(is_ipv4("abc.def.ghi.jkl"), false);
        assert_eq!(is_ipv4("01.1.1.1"), false);
        assert_eq!(is_ipv4("129.123.132.123"), true);
    }
}
