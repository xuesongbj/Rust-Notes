mod macros {
    macro_rules! my_macro {
        ()  => {
            println!("Check out my macro!");
        };
    }

    pub(crate) use my_macro;            // New method(since Rust 1.32,2019-01-17)
}

fn main() {
    macros::my_macro!();
}

/*
 * // Old method
 *
 * #[macro_use]
 * mod macros {
 *      macro_rules! my_macro {
 *          () => {
 *              println!("Check out my macro");
 *          };
 *      }
 * }
 *
 * fn main() {
 *      my_macro!();
 * }
 *
 */
