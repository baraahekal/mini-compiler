use crate::Scanner;
use std::collections::HashSet;

#[test]
fn scanner_initializes_with_empty_tokens() {
    let scanner = Scanner::new("".to_string());
    assert!(scanner.tokens.identifiers.is_empty());
    assert!(scanner.tokens.symbols.is_empty());
    assert!(scanner.tokens.reserved_words.is_empty());
    assert!(scanner.tokens.variables.is_empty());
    assert!(scanner.tokens.lists.is_empty());
    assert!(scanner.tokens.comments.is_empty());
    assert!(scanner.tokens.literals.is_empty());
}

#[test]
fn scanner_processes_comments_correctly() {
    let mut scanner = Scanner::new("/* This is a comment */\nint main() { return 0; }".to_string());
    scanner.process_comments();
    assert_eq!(scanner.tokens.comments, vec!["/* This is a comment */".to_string()]);
}

#[test]
fn scanner_processes_literals_correctly() {
    let mut scanner = Scanner::new("int main() { return 0; }".to_string());
    scanner.process_literals();
    assert_eq!(scanner.tokens.literals, vec![("0".to_string(), "Numeric".to_string())].into_iter().collect());
}

#[test]
fn scanner_processes_symbols_correctly() {
    let mut scanner = Scanner::new("int main() { return 0; }".to_string());
    scanner.process_symbols();
    assert_eq!(scanner.tokens.symbols, vec!["(", ")", "{", "}", ";"].into_iter().map(String::from).collect::<HashSet<_>>());
}

#[test]
fn scanner_processes_identifiers_and_reserved_words_correctly() {
    let mut scanner = Scanner::new("int main() { return 0; }".to_string());
    scanner.process_identifiers_and_reserved_words();
    assert_eq!(scanner.tokens.identifiers, vec!["int"].into_iter().map(String::from).collect::<HashSet<_>>());
    assert_eq!(scanner.tokens.reserved_words, vec!["return"].into_iter().map(String::from).collect::<HashSet<_>>());
}

#[test]
fn scanner_processes_variables_correctly() {
    let mut scanner = Scanner::new("int x = 10; int _x = 2; string x6 = 8;".to_string());
    scanner.process_variables();
    assert_eq!(scanner.tokens.variables, vec!["x", "_x", "x6"].into_iter().map(String::from).collect::<HashSet<_>>());
}

#[test]
fn scanner_processes_lists_correctly() {
    let mut scanner = Scanner::new("int a[3] {1, 2, 3};".to_string());
    scanner.process_lists();
    assert_eq!(scanner.tokens.lists, vec![("int a[3]", "{1, 2, 3} (length: 3)")].into_iter().map(|(k, v)| (k.to_string(), v.to_string())).collect());
}
