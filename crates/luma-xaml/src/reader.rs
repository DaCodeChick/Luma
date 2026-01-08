//! XML reader wrapper for XAML parsing.

use quick_xml::events::Event;
use quick_xml::Reader;
use crate::error::{Result, XamlError, ErrorLocation};
use std::io::BufRead;

/// XAML reader that wraps the quick-xml parser.
pub struct XamlReader<R: BufRead> {
    reader: Reader<R>,
    position: ErrorLocation,
}

impl<R: BufRead> XamlReader<R> {
    /// Create a new XAML reader from a BufRead source.
    pub fn new(reader: Reader<R>) -> Self {
        Self {
            reader,
            position: ErrorLocation::new(1, 0),
        }
    }

    /// Get the current position in the document.
    pub fn position(&self) -> ErrorLocation {
        self.position
    }

    /// Read the next event from the XML stream.
    pub fn read_event(&mut self) -> Result<XamlEvent> {
        let mut buf = Vec::new();
        
        match self.reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = std::str::from_utf8(e.name().as_ref())
                    .map_err(|e| XamlError::Utf8(e))?
                    .to_string();
                
                let mut attributes = Vec::new();
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| XamlError::QuickXml(e.into()))?;
                    let key = std::str::from_utf8(attr.key.as_ref())
                        .map_err(|e| XamlError::Utf8(e))?
                        .to_string();
                    let value = attr.unescape_value()
                        .map_err(|e| XamlError::QuickXml(e))?
                        .to_string();
                    attributes.push((key, value));
                }
                
                Ok(XamlEvent::StartElement {
                    name,
                    attributes,
                    is_empty: false,
                })
            }
            
            Ok(Event::Empty(e)) => {
                let name = std::str::from_utf8(e.name().as_ref())
                    .map_err(|e| XamlError::Utf8(e))?
                    .to_string();
                
                let mut attributes = Vec::new();
                for attr in e.attributes() {
                    let attr = attr.map_err(|e| XamlError::QuickXml(e.into()))?;
                    let key = std::str::from_utf8(attr.key.as_ref())
                        .map_err(|e| XamlError::Utf8(e))?
                        .to_string();
                    let value = attr.unescape_value()
                        .map_err(|e| XamlError::QuickXml(e))?
                        .to_string();
                    attributes.push((key, value));
                }
                
                Ok(XamlEvent::StartElement {
                    name,
                    attributes,
                    is_empty: true,
                })
            }
            
            Ok(Event::End(e)) => {
                let name = std::str::from_utf8(e.name().as_ref())
                    .map_err(|e| XamlError::Utf8(e))?
                    .to_string();
                Ok(XamlEvent::EndElement { name })
            }
            
            Ok(Event::Text(e)) => {
                let text = e.unescape()
                    .map_err(|e| XamlError::QuickXml(e))?
                    .to_string();
                Ok(XamlEvent::Text(text))
            }
            
            Ok(Event::CData(e)) => {
                let text = std::str::from_utf8(&e)
                    .map_err(|e| XamlError::Utf8(e))?
                    .to_string();
                Ok(XamlEvent::Text(text))
            }
            
            Ok(Event::Comment(_)) => {
                // Skip comments
                self.read_event()
            }
            
            Ok(Event::Decl(_)) => {
                // Skip XML declaration
                self.read_event()
            }
            
            Ok(Event::PI(_)) => {
                // Skip processing instructions
                self.read_event()
            }
            
            Ok(Event::DocType(_)) => {
                // Skip doctype
                self.read_event()
            }
            
            Ok(Event::Eof) => Ok(XamlEvent::Eof),
            
            Err(e) => Err(XamlError::QuickXml(e)),
        }
    }

    /// Peek at the next event without consuming it.
    pub fn peek_event(&mut self) -> Result<XamlEvent> {
        // This is a simplified peek - for a full implementation,
        // we'd need to buffer events
        self.read_event()
    }

    /// Skip whitespace-only text nodes.
    pub fn skip_whitespace(&mut self) -> Result<()> {
        loop {
            match self.peek_event()? {
                XamlEvent::Text(ref text) if text.trim().is_empty() => {
                    self.read_event()?;
                }
                _ => break,
            }
        }
        Ok(())
    }
}

impl<'a> XamlReader<&'a [u8]> {
    /// Create a new XAML reader from a string slice.
    pub fn from_str(xaml: &'a str) -> Self {
        let reader = Reader::from_str(xaml);
        Self::new(reader)
    }

    /// Create a new XAML reader from bytes.
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let reader = Reader::from_reader(bytes);
        Self::new(reader)
    }
}

/// Events emitted by the XAML reader.
#[derive(Debug, Clone, PartialEq)]
pub enum XamlEvent {
    /// Start of an element.
    StartElement {
        /// Element name (may include namespace prefix).
        name: String,
        /// Attributes as (name, value) pairs.
        attributes: Vec<(String, String)>,
        /// Whether this is a self-closing element.
        is_empty: bool,
    },
    
    /// End of an element.
    EndElement {
        /// Element name.
        name: String,
    },
    
    /// Text content.
    Text(String),
    
    /// End of file.
    Eof,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_element() {
        let xaml = r#"<Button Content="Click Me"/>"#;
        let mut reader = XamlReader::from_str(xaml);
        
        let event = reader.read_event().unwrap();
        match event {
            XamlEvent::StartElement { name, attributes, is_empty } => {
                assert_eq!(name, "Button");
                assert_eq!(attributes.len(), 1);
                assert_eq!(attributes[0].0, "Content");
                assert_eq!(attributes[0].1, "Click Me");
                assert!(is_empty);
            }
            _ => panic!("Expected StartElement"),
        }
        
        let event = reader.read_event().unwrap();
        assert_eq!(event, XamlEvent::Eof);
    }

    #[test]
    fn test_nested_elements() {
        let xaml = r#"<Window><Button/></Window>"#;
        let mut reader = XamlReader::from_str(xaml);
        
        // Window start
        match reader.read_event().unwrap() {
            XamlEvent::StartElement { name, is_empty, .. } => {
                assert_eq!(name, "Window");
                assert!(!is_empty);
            }
            _ => panic!("Expected Window start"),
        }
        
        // Button (empty)
        match reader.read_event().unwrap() {
            XamlEvent::StartElement { name, is_empty, .. } => {
                assert_eq!(name, "Button");
                assert!(is_empty);
            }
            _ => panic!("Expected Button"),
        }
        
        // Window end
        match reader.read_event().unwrap() {
            XamlEvent::EndElement { name } => {
                assert_eq!(name, "Window");
            }
            _ => panic!("Expected Window end"),
        }
    }

    #[test]
    fn test_text_content() {
        let xaml = r#"<TextBlock>Hello World</TextBlock>"#;
        let mut reader = XamlReader::from_str(xaml);
        
        // TextBlock start
        reader.read_event().unwrap();
        
        // Text
        match reader.read_event().unwrap() {
            XamlEvent::Text(text) => {
                assert_eq!(text, "Hello World");
            }
            _ => panic!("Expected Text"),
        }
        
        // TextBlock end
        reader.read_event().unwrap();
    }

    #[test]
    fn test_multiple_attributes() {
        let xaml = r#"<Button Width="100" Height="50" Content="OK"/>"#;
        let mut reader = XamlReader::from_str(xaml);
        
        match reader.read_event().unwrap() {
            XamlEvent::StartElement { name, attributes, .. } => {
                assert_eq!(name, "Button");
                assert_eq!(attributes.len(), 3);
                
                let attr_map: std::collections::HashMap<_, _> = 
                    attributes.into_iter().collect();
                    
                assert_eq!(attr_map.get("Width"), Some(&"100".to_string()));
                assert_eq!(attr_map.get("Height"), Some(&"50".to_string()));
                assert_eq!(attr_map.get("Content"), Some(&"OK".to_string()));
            }
            _ => panic!("Expected StartElement"),
        }
    }
}
