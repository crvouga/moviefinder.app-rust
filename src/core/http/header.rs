pub trait Header {
    fn header(&self, key: &str, value: &str) -> Self;
}
