use tuicalc::psi;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_psi() {
        assert_eq!(
            psi(),
            -0.6180339887498948482045868343656381177203091798057628621354486227052604628189024
        );
    }
}
