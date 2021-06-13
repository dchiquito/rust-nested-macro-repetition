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
    ($func:ident, $($funcs:ident),* : $($arg:expr),*) => {
        call_them_all!($func: $($arg),*);
        call_them_all!($($funcs),*: $($arg),*);
    };
    ($func:ident : $($arg:expr),*) => {
        $func($($arg),*);
    }
}

fn main() {
    call_them_all!(f,g : );
    call_them_all!(foo,bar : 1, 2, 3);
}
