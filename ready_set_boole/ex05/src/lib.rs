

pub fn negation_normal_form(formula: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = negation_normal_form("AB&!");
        assert_eq!(result, "A!B!|");
    }
}
