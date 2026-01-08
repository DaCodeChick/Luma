//! Markup extension parser - parses {Binding Path=Name} syntax.

use crate::error::{Result, XamlError};
use std::collections::HashMap;

/// A token in markup extension syntax.
#[derive(Debug, Clone, PartialEq)]
pub enum MarkupToken {
    /// Opening brace {
    OpenBrace,
    /// Closing brace }
    CloseBrace,
    /// Identifier (extension name or parameter name)
    Identifier(String),
    /// String literal
    String(String),
    /// Equals sign
    Equals,
    /// Comma
    Comma,
    /// End of input
    Eof,
}

/// Tokenizes markup extension syntax.
pub struct MarkupLexer {
    input: Vec<char>,
    position: usize,
}

impl MarkupLexer {
    /// Create a new lexer from input string.
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Get the next token.
    pub fn next_token(&mut self) -> Result<MarkupToken> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(MarkupToken::Eof);
        }

        let ch = self.current_char();

        match ch {
            '{' => {
                self.position += 1;
                Ok(MarkupToken::OpenBrace)
            }
            '}' => {
                self.position += 1;
                Ok(MarkupToken::CloseBrace)
            }
            '=' => {
                self.position += 1;
                Ok(MarkupToken::Equals)
            }
            ',' => {
                self.position += 1;
                Ok(MarkupToken::Comma)
            }
            '\'' | '"' => self.read_string(ch),
            _ if ch.is_alphabetic() || ch == '_' || ch == ':' => self.read_identifier(),
            _ => Err(XamlError::InvalidMarkupExtension {
                line: 0,
                details: format!("Unexpected character: '{}'", ch),
            }),
        }
    }

    /// Read a string literal.
    fn read_string(&mut self, quote: char) -> Result<MarkupToken> {
        self.position += 1; // Skip opening quote
        let mut value = String::new();

        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch == quote {
                self.position += 1; // Skip closing quote
                return Ok(MarkupToken::String(value));
            }
            
            if ch == '\\' && self.position + 1 < self.input.len() {
                self.position += 1;
                let escaped = self.current_char();
                value.push(match escaped {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '\'' => '\'',
                    '"' => '"',
                    _ => escaped,
                });
                self.position += 1;
            } else {
                value.push(ch);
                self.position += 1;
            }
        }

        Err(XamlError::InvalidMarkupExtension {
            line: 0,
            details: "Unterminated string literal".to_string(),
        })
    }

    /// Read an identifier.
    fn read_identifier(&mut self) -> Result<MarkupToken> {
        let mut value = String::new();

        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch.is_alphanumeric() || ch == '_' || ch == ':' || ch == '.' {
                value.push(ch);
                self.position += 1;
            } else {
                break;
            }
        }

        Ok(MarkupToken::Identifier(value))
    }

    /// Skip whitespace characters.
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.current_char().is_whitespace() {
            self.position += 1;
        }
    }

    /// Get current character.
    fn current_char(&self) -> char {
        self.input[self.position]
    }

    /// Peek at remaining input for debugging.
    #[allow(dead_code)]
    fn remaining(&self) -> String {
        self.input[self.position..].iter().collect()
    }
}

/// Parsed markup extension.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedMarkupExtension {
    /// Extension name (e.g., "StaticResource", "Binding")
    pub name: String,
    
    /// Positional argument (the first argument without a name)
    pub positional_arg: Option<String>,
    
    /// Named arguments
    pub arguments: HashMap<String, String>,
}

/// Parse a markup extension string.
pub fn parse_markup_extension(input: &str) -> Result<ParsedMarkupExtension> {
    let mut lexer = MarkupLexer::new(input);
    
    // Expect opening brace
    match lexer.next_token()? {
        MarkupToken::OpenBrace => {}
        _ => {
            return Err(XamlError::InvalidMarkupExtension {
                line: 0,
                details: "Markup extension must start with '{'".to_string(),
            });
        }
    }
    
    // Read extension name
    let name = match lexer.next_token()? {
        MarkupToken::Identifier(name) => name,
        _ => {
            return Err(XamlError::InvalidMarkupExtension {
                line: 0,
                details: "Expected extension name".to_string(),
            });
        }
    };
    
    let mut positional_arg = None;
    let mut arguments = HashMap::new();
    
    // Parse arguments
    loop {
        let token = lexer.next_token()?;
        
        match token {
            MarkupToken::CloseBrace => break,
            MarkupToken::Eof => {
                return Err(XamlError::InvalidMarkupExtension {
                    line: 0,
                    details: "Unexpected end of markup extension".to_string(),
                });
            }
            MarkupToken::Comma => continue,
            MarkupToken::Identifier(id) => {
                // Peek ahead to check if this is a named argument
                let next = lexer.next_token()?;
                match next {
                    MarkupToken::Equals => {
                        // Named argument
                        let value = match lexer.next_token()? {
                            MarkupToken::String(s) => s,
                            MarkupToken::Identifier(s) => s,
                            _ => {
                                return Err(XamlError::InvalidMarkupExtension {
                                    line: 0,
                                    details: "Expected value after '='".to_string(),
                                });
                            }
                        };
                        arguments.insert(id, value);
                    }
                    MarkupToken::Comma => {
                        // Positional argument followed by comma
                        if positional_arg.is_none() {
                            positional_arg = Some(id);
                        } else {
                            return Err(XamlError::InvalidMarkupExtension {
                                line: 0,
                                details: "Multiple positional arguments not supported".to_string(),
                            });
                        }
                    }
                    MarkupToken::CloseBrace => {
                        // Positional argument at end
                        if positional_arg.is_none() {
                            positional_arg = Some(id);
                        } else {
                            return Err(XamlError::InvalidMarkupExtension {
                                line: 0,
                                details: "Multiple positional arguments not supported".to_string(),
                            });
                        }
                        break;
                    }
                    _ => {
                        return Err(XamlError::InvalidMarkupExtension {
                            line: 0,
                            details: "Unexpected token after identifier".to_string(),
                        });
                    }
                }
            }
            MarkupToken::String(s) => {
                // Positional string argument
                if positional_arg.is_none() {
                    positional_arg = Some(s);
                } else {
                    return Err(XamlError::InvalidMarkupExtension {
                        line: 0,
                        details: "Multiple positional arguments not supported".to_string(),
                    });
                }
            }
            _ => {
                return Err(XamlError::InvalidMarkupExtension {
                    line: 0,
                    details: "Unexpected token in markup extension".to_string(),
                });
            }
        }
    }
    
    Ok(ParsedMarkupExtension {
        name,
        positional_arg,
        arguments,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_extension() {
        let parsed = parse_markup_extension("{StaticResource MyBrush}").unwrap();
        assert_eq!(parsed.name, "StaticResource");
        assert_eq!(parsed.positional_arg, Some("MyBrush".to_string()));
        assert!(parsed.arguments.is_empty());
    }

    #[test]
    fn test_extension_with_named_args() {
        let parsed = parse_markup_extension("{Binding Path=Name, Mode=TwoWay}").unwrap();
        assert_eq!(parsed.name, "Binding");
        assert_eq!(parsed.positional_arg, None);
        assert_eq!(parsed.arguments.get("Path"), Some(&"Name".to_string()));
        assert_eq!(parsed.arguments.get("Mode"), Some(&"TwoWay".to_string()));
    }

    #[test]
    fn test_extension_with_positional_and_named() {
        let parsed = parse_markup_extension("{Binding Name, Mode=TwoWay}").unwrap();
        assert_eq!(parsed.name, "Binding");
        assert_eq!(parsed.positional_arg, Some("Name".to_string()));
        assert_eq!(parsed.arguments.get("Mode"), Some(&"TwoWay".to_string()));
    }

    #[test]
    fn test_extension_with_string_literal() {
        let parsed = parse_markup_extension("{StaticResource 'My Brush'}").unwrap();
        assert_eq!(parsed.name, "StaticResource");
        assert_eq!(parsed.positional_arg, Some("My Brush".to_string()));
    }

    #[test]
    fn test_null_extension() {
        let parsed = parse_markup_extension("{x:Null}").unwrap();
        assert_eq!(parsed.name, "x:Null");
        assert_eq!(parsed.positional_arg, None);
        assert!(parsed.arguments.is_empty());
    }

    #[test]
    fn test_type_extension() {
        let parsed = parse_markup_extension("{x:Type local:MyType}").unwrap();
        assert_eq!(parsed.name, "x:Type");
        assert_eq!(parsed.positional_arg, Some("local:MyType".to_string()));
    }

    #[test]
    fn test_complex_binding() {
        let parsed = parse_markup_extension(
            "{Binding Path=User.Name, Mode=TwoWay, UpdateSourceTrigger=PropertyChanged}"
        ).unwrap();
        assert_eq!(parsed.name, "Binding");
        assert_eq!(parsed.arguments.get("Path"), Some(&"User.Name".to_string()));
        assert_eq!(parsed.arguments.get("Mode"), Some(&"TwoWay".to_string()));
        assert_eq!(parsed.arguments.get("UpdateSourceTrigger"), Some(&"PropertyChanged".to_string()));
    }
}
