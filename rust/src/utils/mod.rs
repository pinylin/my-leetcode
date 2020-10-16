pub mod sort;

#[macro_export]
macro_rules! vec2d {
        [$(
            [$(
                $i:expr
            ),*]
        ),*] => {
            vec![$(
                vec![$($i),*]
            ),*]
        }
    }

#[macro_export]
macro_rules! assert_eq_float {
    ($left:expr, $right:expr) => {{
        let left = $left;
        let right = $right;
        if (left - right).abs() > 0.0001 {
            panic!(format!(
                "assertion failed: \n  left: `{}`\n right: `{}`",
                left, right
            ));
        }
    }};
}
