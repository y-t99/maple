use std::fs;
use std::fs::File;
use std::io::Write;

use crate::maple::Maple;

struct Garden {
    location: String,
}

impl Garden {
    fn write_maple(&self, maple: &Maple) -> Result<String, String> {
        let key_len = maple.key_len.to_be_bytes();
        let value_len = maple.value_len.to_be_bytes();
        let key = maple.key.as_bytes();
        let value = maple.value.as_bytes();
        let path = format!("{}db.maples", self.location);
        let io_result = File::create(&path);
        if io_result.is_err() {
            return Err("io error".to_string());
        }
        let mut file = io_result.unwrap();
        file.write_all(&key_len).unwrap();
        file.write_all(&value_len).unwrap();
        file.write_all(key).unwrap();
        file.write_all(value).unwrap();
        Ok("ok".to_string())
    }

    fn read_maple(&self) -> Result<Maple, String> {
        let path = format!("{}/db.maples", self.location);
        let io_result = fs::read(path);
        if io_result.is_err() {
            return Err("io error".to_string());
        }
        let bytes = io_result.unwrap();
        let vernier = bytes.as_slice();
        let mut key_len_bytes = [0; 8];
        key_len_bytes.clone_from_slice(&vernier[0..8]);
        let key_len = u64::from_be_bytes(key_len_bytes) as usize;
        let mut value_len_bytes = [0; 8];
        value_len_bytes.clone_from_slice(&vernier[8..16]);
        let value_len = u64::from_be_bytes(value_len_bytes) as usize;
        let l_key: usize = 16;
        let r_key: usize = 16 + key_len;
        let key_bytes = vernier[l_key..r_key].to_vec();
        let key = String::from_utf8(key_bytes);
        let l_value = r_key;
        let r_value = r_key + value_len;
        let value_bytes = vernier[l_value..r_value].to_vec();
        let value = String::from_utf8(value_bytes);
        if key.is_err() || value.is_err() {
            return Err("fmt error".to_string());
        }
        Ok(Maple {
            key_len: key_len as u64,
            value_len: value_len as u64,
            key: key.unwrap(),
            value: value.unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::garden::Garden;
    use crate::maple::Maple;

    #[test]
    fn write_read_maple() {
        let garden = Garden {
            location: String::from("./"),
        };
        let _ = garden
            .write_maple(&Maple {
                key_len: "key".as_bytes().len() as u64,
                value_len: "value".as_bytes().len() as u64,
                key: "key".to_string(),
                value: "value".to_string(),
            })
            .unwrap();
        let maple = garden.read_maple().unwrap();
        let _ = fs::remove_file("./db.maples").unwrap();
        assert_eq!(maple.key_len, 3);
        assert_eq!(maple.value_len, 5);
        assert_eq!(maple.key.as_str(), "key");
        assert_eq!(maple.value.as_str(), "value");
    }
}
