#[derive(Debug)]
enum LexToken<'a> {
    Keyword(&'a str),
    BinaryOperation(&'a str),
    Number(&'a str),
}

fn lex_line(line: &str) -> Vec<LexToken> {
    let words: Vec<&str> = line.split(' ').filter(|&x| !x.is_empty()).collect();
    let mut tokens:Vec<LexToken> = Vec::<LexToken>::new();
    println!("{:?}", words);
    for word in words {
        match word {
            "print" => tokens.push(LexToken::Keyword("print")),
            "+" => tokens.push(LexToken::BinaryOperation("+")),
            x if numeric(x) => tokens.push(LexToken::Number(x)),
            x => panic!("[ERROR] Unknown token: {}", x), 
        }
    }

    tokens
}

fn numeric(to_check: &str) -> bool {
    true
}

pub fn lex_file(file_contents: &str) {
    let lines: Vec<&str> = file_contents
        .split('\n')
        .filter(|&x| !x.is_empty())
        .collect();
    for line in &lines {
        println!("{:?}", lex_line(line));
    }
}
