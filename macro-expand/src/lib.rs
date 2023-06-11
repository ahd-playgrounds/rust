#[allow(unused)]
#[allow(dead_code)]

macro_rules! def_mac {
    ($s:ident, $table:tt, $cols:expr) => {
        pub struct $s {}
    };
}
