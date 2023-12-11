struct MapleDB {}

impl MapleDB {
    fn get(&self, _key: &str) -> Result<Option<String>, String> {
        Ok(None)
    }

    fn get_del(&self, _key: &str) -> Result<Option<String>, String> {
        Ok(None)
    }

    fn set(&self, _key: &str, _value: &str) -> Result<Option<String>, String> {
        Ok(None)
    }

    fn str_len(&self, _key: &str) -> Result<Option<u64>, String> {
        Ok(None)
    }
}
