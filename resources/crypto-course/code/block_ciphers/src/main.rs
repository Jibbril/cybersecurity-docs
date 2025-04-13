mod counter_mode;
mod electronic_code_book_mode;
mod cipher_block_chaining_mode;

fn main() {
    electronic_code_book_mode::run_example();
    println!("");
    cipher_block_chaining_mode::run_example();
    println!("");
    counter_mode::run_example();
    println!("");
}
