pub fn xor(text: &[u8], key: &[u8]) -> Vec<u8> {
    text.iter()
        .zip(key.iter().cycle())
        .map(|(t, k)| t ^ k)
        .collect()
}

pub fn xor_to_ascii(text: &[u8], key: &[u8]) -> String {
    xor(text, key).iter().map(|b| char::from(*b)).collect()
}

pub fn xor_to_fmt_hex(text: &[u8], key: &[u8]) -> String {
    format!("{:x?}", xor(text, key)).replace(
        |c| match c {
            '[' | ']' | ',' => true,
            _ => false,
        },
        "",
    )
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    const K1: &str = "A";
    const K2: &str = "FISH";

    const M1: &str = "hello";
    const M2: &str = ")$--.";
    const M3: &str = " this is a test";
    const M4: &str = "f=;!5i:;f(s<#:'";

    #[rstest(
        key,
        text,
        expected,
        case(K1, M1, ")$--."),
        case(K1, M2, "hello"),
        case(K2, M3, "f=;!5i:;f(s<#:'"),
        case(K2, M4, " this is a test")
    )]
    fn ascii_output(key: &str, text: &str, expected: &str) {
        assert_eq!(expected, xor_to_ascii(text.as_bytes(), key.as_bytes()));
    }

    #[rstest(
        key,
        text,
        expected,
        case(K1, M1, "29 24 2d 2d 2e"),
        case(K1, M2, "68 65 6c 6c 6f"),
        case(K2, M3, "66 3d 3b 21 35 69 3a 3b 66 28 73 3c 23 3a 27"),
        case(K2, M4, "20 74 68 69 73 20 69 73 20 61 20 74 65 73 74")
    )]
    fn hex_output(key: &str, text: &str, expected: &str) {
        assert_eq!(expected, xor_to_fmt_hex(text.as_bytes(), key.as_bytes()));
    }
}
