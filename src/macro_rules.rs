macro_rules! hashmap {
    ($( $key: expr => $val: expr ), *) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )*
        map
}} }

macro_rules! echo {
    () => (
        println!("echo nothing");
    );
    ($exp: expr) => (
        println!("echo single expression: {}", stringify!($exp));
    );
    ($($exp: expr), +) => (
        $(
            println!("echo multi expressions: {}", stringify!($exp));
        )+
    );
}
