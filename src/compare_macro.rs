/// This macro allows you to compare multiple values without the && operator avoiding
/// duplication and improving readability.
#[macro_export]
macro_rules! c{
    ( $a:tt $op:tt $b:tt) => {
        $a $op $b
    };
    ( $a:tt $op:tt $b:tt $( $op_rest:tt $c:tt )+ ) => {
        $a $op $b && c!($b $( $op_rest $c )+)
    };
}
