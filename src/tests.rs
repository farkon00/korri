use super::*;

#[test]
fn numeric_test() {

    // 2, 2.3, .23, 2.3e10

    assert_eq!(lexer::numeric("2"), true); 
    assert_eq!(lexer::numeric("x"), false);
    assert_eq!(lexer::numeric("2.3"), true);
    assert_eq!(lexer::numeric("49."), false);
    assert_eq!(lexer::numeric(".43"), true);
    assert_eq!(lexer::numeric("2e10"), true);
    assert_eq!(lexer::numeric("2e"), false);
    assert_eq!(lexer::numeric("e10"), false);
    assert_eq!(lexer::numeric("2.e10"), false);
    assert_eq!(lexer::numeric("2.1e10"), true);
}
