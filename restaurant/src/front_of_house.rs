pub mod hosting;

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}

pub fn say_hello() {
    println!("Hello!");
    super::say_bye();
}