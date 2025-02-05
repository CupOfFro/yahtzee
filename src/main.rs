use std::io;

// usefule references
// https://stackoverflow.com/questions/33139248/i-cannot-print-color-escape-codes-to-the-terminal
// https://stackoverflow.com/questions/69597466/move-cursor-escape-sequence
// https://duffney.io/usingansiescapesequencespowershell/


fn main() {
    println!("\x1b[2J"); // clear screen
    println!("\x1b[H"); // Go home
    println!("\x1b[31mHello, world!\x1b[39m"); // Red text
    println!("\x1b[5;5HTesting"); // Go to position 5, 5 (1 based I believe)
    println!("\x1b[31mHello, world!\x1b[39m"); // Red text

    let mut pause = String::new();
    io::stdin()
        .read_line(&mut pause)
        .expect("Failed to read line");
}
