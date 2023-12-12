use std::fmt::format;
use std::fs;
use crate::maple::Maple;

struct Garden {
    location: String,
}

impl Garden {
    fn write_maple(&self, content: &str) -> Result<String, String> {
        let path = format!("{}/db.maples", self.location);
        let io_result = fs::write(path, content.as_bytes());
        if io_result.is_err() {
            Err("io error".to_string())
        } else {
            Ok("ok".to_string())
        }
    }

    fn read_maple(&self) -> Result<String, String> {
        let path = format!("{}/db.maples", self.location);
        let io_result = fs::read(path);
        if io_result.is_err() {
            return Err("io error".to_string());
        }
        let content = String::from_utf8(io_result.unwrap());
        if content.is_err() {
            return Err("encode error".to_string());
        }
        Ok(content.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::garden::Garden;

    #[test]
    fn write_read_maple() {
        let garden = Garden {
            location: String::from("./")
        };
        let _ = garden.write_maple("Hello World!").unwrap();
        let content = garden.read_maple().unwrap();
        let _ = fs::remove_file("./db.maples").unwrap();
        assert_eq!(content.as_str(), "Hello World!");
    }
}
