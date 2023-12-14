use std::fmt::Display;

pub struct Maple {
    pub key_len: u64,
    pub value_len: u64,
    pub key: String,
    pub value: String,
}

impl Display for Maple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::maple::Maple;

    #[test]
    fn display_maple() {
        let maple = Maple {
            key_len: "key".as_bytes().len() as u64,
            value_len: "value".as_bytes().len() as u64,
            key: "key".to_string(),
            value: "value".to_string(),
        };
        assert_eq!(maple.to_string().as_str(), "key:value");
    }
}
