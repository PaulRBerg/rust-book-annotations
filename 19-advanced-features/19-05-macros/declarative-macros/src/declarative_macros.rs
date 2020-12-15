// The "#[macro_export]" annotation indicates that this macro should be made
// available whenever the crate in which the macro is defined is brought
// into scope.
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

pub fn run() {}
