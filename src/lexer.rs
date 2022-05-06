
// Allowing dead code as this is currently unimplemented
#[allow(dead_code)]
enum LexToken<'a> {
    Keyword(&'a str),
    BinaryOperation(&'a str),
}

fn lex_line(line: &str) {
    println!("{}", line);
}



pub fn lex_file(file_contents: &str) {
    let lines: Vec<&str> = file_contents.split('\n').filter(|&x| !x.is_empty()).collect();
    for line in &lines {
        lex_line(line);
    }
}
