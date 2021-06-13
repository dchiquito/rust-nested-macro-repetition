fn f() {
    println!("f");
}

fn g() {
    println!("g");
}

fn foo(a: u32, b: u32, c: u32) {
    println!("foo {} {} {}", a, b, c);
}

fn bar(a: u32, b: u32, c: u32) {
    println!("bar {} {} {}", a, b, c);
}

macro_rules! call_them_all {
    ($($func:ident),+ : $($arg:expr),*) => {
        $(
            // This nested repetition is not recognized?
            $func($($arg)*);
        )*
    };
}

fn main() {
    call_them_all!(f,g : );
    call_them_all!(foo,bar : 1, 2, 3);
}
