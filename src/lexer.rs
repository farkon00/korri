#[derive(Debug)]
enum LexToken<'a> {
    Keyword(&'a str),
    BinaryOperation(&'a str),
    Number(&'a str),
    Identifier(&'a str),
}

const KEYWORDS: [&str; 1] = [
    "print",
];

fn lex_line(line: &str) -> Vec<LexToken> {
    let words: Vec<&str> = line.split(' ').filter(|&x| !x.is_empty()).collect();
    let mut tokens:Vec<LexToken> = Vec::<LexToken>::new();
    println!("{:?}", words);
    for word in words {
        match word {
            word if KEYWORDS.contains(&word) => tokens.push(LexToken::Keyword(word)),
            "+" => tokens.push(LexToken::BinaryOperation("+")),
            x if numeric(x) => tokens.push(LexToken::Number(x)),
            x => tokens.push(LexToken::Identifier(x)), 
        }
    }

    tokens
}

pub fn numeric(to_check: &str) -> bool {
    let mut characters = to_check.chars().peekable();
    let mut last_char: char = ' ';
    let mut seen_dot: bool = false;
    let mut seen_exp: bool = false;
    while let Some(t) = characters.next() {
        match t {
            t if t.is_digit(10) => last_char = t, // Numbers are normally found in numbers :p
            '.' if seen_dot || seen_exp => return false, // Only one dot per number, and no dot after `e`
            'e' if !seen_exp => {
                let next_char = characters.peek();
                if last_char.is_digit(10) {
                    match next_char {
                        Some(v) if v.is_digit(10) => seen_exp = true,
                        _ => return false
                    }
                } else {
                    return false
                }
            },
            '.' => {
                let next_char = characters.peek();
                match next_char {
                    Some(v) if v.is_digit(10) => seen_dot = true,
                    _ => return false
                }
            }, // Only a number can follow a dot
            _ => return false
        }
    }
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
