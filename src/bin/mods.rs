#![allow(unused)]

mod foo {
    pub const TEMP_FOO: &str = "TEMP_FOO";

    pub fn print() {
        println!("Iam module foo")
    }
}
mod my_module_a {
    use super::foo;
    pub const TEMP_A: &str = "TEMP_A";

    pub fn print() {
        println!("Iam module A")
    }

    pub fn print_foo() {
        print!("Calling from module A. ");
        foo::print();
    }

    pub mod my_module_b {
        use super::super::foo;
        pub const TEMP_B: &str = "TEMP_B";

        pub fn print() {
            println!("Iam module B")
        }

        pub fn print_foo() {
            print!("Calling from module B. ");
            foo::print();
        }
    }
}

fn main() {
    let module_a_state = my_module_a::TEMP_A;
    my_module_a::print();
    my_module_a::print_foo();
    println!("{module_a_state}");

    let module_b_state = my_module_a::my_module_b::TEMP_B;
    my_module_a::my_module_b::print();
    my_module_a::my_module_b::print_foo();
    println!("{module_b_state}");
}
