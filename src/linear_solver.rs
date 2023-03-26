fn solver_one_unkonw_linear_equation(
    equation: &str,
    mut variable_to_solve_for: Option<&str>,
) -> i128 {
    let mut parts = equation.split('=');
    let lhs = parts.next().unwrap().trim();
    let rhs = parts.next().unwrap().trim();
    let lhs_parts: Vec<&str> = lhs.split(' ').collect();
    let rhs_parts: Vec<&str> = rhs.split(' ').collect();
    let lhs_coef = lhs_parts[0].parse::<i128>().unwrap();
    let lhs_var = lhs_parts[1];
    let rhs_const = rhs_parts[0].parse::<i128>().unwrap();

    if variable_to_solve_for == None {
        variable_to_solve_for = Some("x")
    }

    // Solve for the unknown variable
    match lhs_var {
        "x" => (rhs_const - lhs_coef) / lhs_coef,
        _ => panic!("Equation must be linear with one unknown (e.g. '3x + 2 = 8')"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_linear_equation() {
        assert_eq!(
            solver_one_unkonw_linear_equation("3x + 2 = 8", Some("x")),
            2
        );
        assert_eq!(
            solver_one_unkonw_linear_equation("-2x - 5 = 11", Some("x")),
            -8
        );
        assert_eq!(solver_one_unkonw_linear_equation("7x = 21", Some("x")), 3);
        assert_eq!(
            solver_one_unkonw_linear_equation("x - 4 = -7", Some("x")),
            -3
        );
    }

    #[test]
    #[should_panic]
    fn test_invalid_equation() {
        assert_ne!(
            solver_one_unkonw_linear_equation("2x + y = 5"),
            "Invalid Equation form"
        );
    }
}
