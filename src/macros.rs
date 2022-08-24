// libera::macros
//!
//

/// Ternary `if` operator.
#[macro_export]
macro_rules! if3 {
    ($if_condition: expr => $if_true: expr , $if_false: expr) => {
        if $if_condition {
            $if_true
        } else {
            $if_false
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::if3;

    #[test]
    fn if3() {
        assert_eq!(if3!(true => 0, 1), 0);
        assert_eq!(if3!(false => 0, 1), 1);
    }
}
