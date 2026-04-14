fn main() {
    let args = std::env::args().skip(1).collect::<Vec<String>>();
    let mut text = Vec::<String>::new();
    let mut trailing_newline = true;

    for arg in args {
        if arg == "-n" {
            trailing_newline = false;
        } else {
            text.push(arg);
        }
    }

    print!(
        "{}{}",
        text.join(" "),
        if trailing_newline { "\n" } else { "" }
    );
}
