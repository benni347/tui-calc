use tuicalc::phi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phi() {
        assert_eq!(phi(), 1.6180339887498949);
    }
}
