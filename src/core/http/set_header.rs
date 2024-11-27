pub trait SetHeader {
    fn set_header(&mut self, key: &str, value: &str) -> &Self;
}
