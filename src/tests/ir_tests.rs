use super::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_process_ir() {
        let mut ir = HashMap::new();
        ir.insert("key1", "value1");
        ir.insert("key2", "value2");
        assert!(process_ir(&ir).is_ok());
    }
}
