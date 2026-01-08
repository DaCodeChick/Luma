//! XAML parser - parses XAML files and strings into object models.

use crate::model::XamlDocument;
use crate::types::TypeRegistry;
use crate::flags::ParserFlags;
use crate::error::{Result, XamlError};
use std::path::Path;

/// Settings for the XAML parser.
#[derive(Debug, Clone)]
pub struct ParserSettings {
    /// Parser behavior flags.
    pub flags: ParserFlags,
}

impl Default for ParserSettings {
    fn default() -> Self {
        Self {
            flags: ParserFlags::DEFAULT,
        }
    }
}

impl ParserSettings {
    /// Create new parser settings with default flags.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create parser settings with custom flags.
    pub fn with_flags(flags: ParserFlags) -> Self {
        Self { flags }
    }

    /// Enable strict mode (unknown types cause errors).
    pub fn strict(mut self) -> Self {
        self.flags.insert(ParserFlags::STRICT_MODE);
        self
    }

    /// Disable strict mode (allow unknown types).
    pub fn lenient(mut self) -> Self {
        self.flags.remove(ParserFlags::STRICT_MODE);
        self.flags.insert(ParserFlags::ALLOW_UNKNOWN_TYPES);
        self
    }

    /// Enable type validation.
    pub fn validate_types(mut self) -> Self {
        self.flags.insert(ParserFlags::VALIDATE_TYPES);
        self
    }

    /// Preserve whitespace in text content.
    pub fn preserve_whitespace(mut self) -> Self {
        self.flags.insert(ParserFlags::PRESERVE_WHITESPACE);
        self
    }

    /// Enable namespace validation.
    pub fn validate_namespaces(mut self) -> Self {
        self.flags.insert(ParserFlags::VALIDATE_NAMESPACES);
        self
    }
}

/// XAML parser that converts XAML text into an object model.
pub struct XamlParser {
    /// Type registry for resolving types.
    registry: TypeRegistry,
    
    /// Parser settings.
    settings: ParserSettings,
}

impl XamlParser {
    /// Create a new XAML parser with the given type registry.
    pub fn new(registry: TypeRegistry) -> Self {
        Self {
            registry,
            settings: ParserSettings::default(),
        }
    }

    /// Set custom parser settings.
    pub fn with_settings(mut self, settings: ParserSettings) -> Self {
        self.settings = settings;
        self
    }

    /// Parse a XAML file.
    pub fn parse_file(&self, path: &Path) -> Result<XamlDocument> {
        let content = std::fs::read_to_string(path)?;
        self.parse_string(&content)
    }

    /// Parse a XAML string.
    pub fn parse_string(&self, xaml: &str) -> Result<XamlDocument> {
        let mut reader = crate::reader::XamlReader::from_str(xaml);
        let mut context = ParseContext::new(&self.registry, &self.settings);
        
        // Skip any leading whitespace or comments
        loop {
            let event = reader.read_event()?;
            match event {
                crate::reader::XamlEvent::Text(ref text) if text.trim().is_empty() => {
                    // Skip whitespace
                    continue;
                }
                crate::reader::XamlEvent::StartElement { name, attributes, is_empty } => {
                    // Found the root element - parse it directly
                    let root = self.parse_root_element(name, attributes, is_empty, &mut reader, &mut context)?;
                    
                    // Create the document
                    let mut doc = XamlDocument::new(root);
                    
                    // Extract resources from context if any
                    for (key, value) in context.resources {
                        doc.add_resource(key, value);
                    }
                    
                    return Ok(doc);
                }
                crate::reader::XamlEvent::Eof => {
                    return Err(XamlError::custom("Empty document - no root element found"));
                }
                _ => {
                    return Err(XamlError::custom("Unexpected content before root element"));
                }
            }
        }
    }
    
    /// Parse the root element with known start event data.
    fn parse_root_element<R: std::io::BufRead>(
        &self,
        element_name: String,
        attributes: Vec<(String, String)>,
        is_empty: bool,
        reader: &mut crate::reader::XamlReader<R>,
        context: &mut ParseContext<'_>,
    ) -> Result<crate::model::XamlElement> {
        use crate::reader::XamlEvent;
        use crate::model::{XamlElement, XamlNode};
        use crate::types::XamlTypeName;
        
        // Parse the element name (handle namespaces)
        let (prefix, local_name) = parse_qualified_name(&element_name);
        
        // Create the type name - initially without namespace resolution
        let type_name = XamlTypeName::new("", local_name);
        
        // Create the element
        let mut element = XamlElement::new(type_name.clone());
        
        // Process attributes FIRST to get namespace declarations
        for (attr_name, attr_value) in attributes {
            self.process_attribute(&mut element, &attr_name, &attr_value, context)?;
        }
        
        // NOW resolve the namespace for this element
        let namespace = if let Some(prefix) = prefix {
            context.resolve_namespace(prefix)?
        } else {
            context.default_namespace.clone()
        };
        
        // Update the type name with resolved namespace
        element.type_name = XamlTypeName::new(namespace, local_name);
        
        // If not empty, parse children
        if !is_empty {
            loop {
                let event = reader.read_event()?;
                
                match event {
                    XamlEvent::EndElement { name } => {
                        if name != element_name {
                            return Err(XamlError::XmlError {
                                line: 0,
                                col: 0,
                                message: format!("Mismatched tags: expected {}, got {}", element_name, name),
                            });
                        }
                        break;
                    }
                    
                    XamlEvent::StartElement { name, attributes, is_empty } => {
                        if name.contains('.') {
                            self.parse_property_element(&mut element, &name, reader, context)?;
                        } else {
                            let child = self.parse_child_element(name, attributes, is_empty, reader, context)?;
                            element.add_child(XamlNode::Element(child));
                        }
                    }
                    
                    XamlEvent::Text(text) => {
                        if self.has_flag(ParserFlags::PRESERVE_WHITESPACE) || !text.trim().is_empty() {
                            element.add_child(XamlNode::Text(text));
                        }
                    }
                    
                    XamlEvent::Eof => {
                        return Err(XamlError::custom(format!("Unexpected EOF while parsing element {}", element_name)));
                    }
                }
            }
        }
        
        Ok(element)
    }
    
    /// Parse a single element from the reader.
    #[allow(dead_code)]
    fn parse_element<R: std::io::BufRead>(
        &self,
        reader: &mut crate::reader::XamlReader<R>,
        context: &mut ParseContext<'_>,
    ) -> Result<crate::model::XamlElement> {
        use crate::reader::XamlEvent;
        use crate::model::{XamlElement, XamlNode};
        use crate::types::XamlTypeName;
        
        // Read the start element event
        let (element_name, attributes, is_empty) = match reader.read_event()? {
            XamlEvent::StartElement { name, attributes, is_empty } => {
                (name, attributes, is_empty)
            }
            XamlEvent::Eof => {
                return Err(XamlError::custom("Unexpected end of file"));
            }
            XamlEvent::EndElement { name } => {
                return Err(XamlError::custom(format!("Unexpected end element: {}", name)));
            }
            XamlEvent::Text(text) => {
                return Err(XamlError::custom(format!("Unexpected text: {}", text)));
            }
        };
        
        // Parse the element name (handle namespaces)
        let (prefix, local_name) = parse_qualified_name(&element_name);
        
        // Resolve namespace if prefix exists
        let namespace = if let Some(prefix) = prefix {
            context.resolve_namespace(prefix)?
        } else {
            context.default_namespace.clone()
        };
        
        // Create the type name
        let type_name = XamlTypeName::new(namespace, local_name);
        
        // Create the element
        let mut element = XamlElement::new(type_name.clone());
        
        // Process attributes
        for (attr_name, attr_value) in attributes {
            self.process_attribute(&mut element, &attr_name, &attr_value, context)?;
        }
        
        // If not empty, parse children
        if !is_empty {
            loop {
                let event = reader.read_event()?;
                
                match event {
                    XamlEvent::EndElement { name } => {
                        // Verify this is the correct end tag
                        if name != element_name {
                            return Err(XamlError::XmlError {
                                line: 0,
                                col: 0,
                                message: format!("Mismatched tags: expected {}, got {}", element_name, name),
                            });
                        }
                        break;
                    }
                    
                    XamlEvent::StartElement { name, .. } => {
                        // Check if this is a property element (e.g., <Button.Content>)
                        if name.contains('.') {
                            self.parse_property_element(&mut element, &name, reader, context)?;
                        } else {
                            // Regular child element - need to "put back" this event
                            // For now, we'll re-read by creating a new reader for this element
                            // This is a simplification; a proper implementation would buffer events
                            let child = self.parse_element_from_event(name, reader, context)?;
                            element.add_child(XamlNode::Element(child));
                        }
                    }
                    
                    XamlEvent::Text(text) => {
                        // Add text content if not just whitespace (unless preserving whitespace)
                        if self.has_flag(ParserFlags::PRESERVE_WHITESPACE) || !text.trim().is_empty() {
                            element.add_child(XamlNode::Text(text));
                        }
                    }
                    
                    XamlEvent::Eof => {
                        return Err(XamlError::custom(format!("Unexpected EOF while parsing element {}", element_name)));
                    }
                }
            }
        }
        
        Ok(element)
    }
    
    /// Parse an element when we already have the start event information.
    #[allow(dead_code)]
    fn parse_element_from_event<R: std::io::BufRead>(
        &self,
        element_name: String,
        reader: &mut crate::reader::XamlReader<R>,
        context: &mut ParseContext<'_>,
    ) -> Result<crate::model::XamlElement> {
        use crate::reader::XamlEvent;
        use crate::model::{XamlElement, XamlNode};
        use crate::types::XamlTypeName;
        
        // We need to peek to get attributes - for now, assume they're already read
        // This is a helper that handles the case where we've already seen the start tag
        
        // Parse the element name (handle namespaces)
        let (prefix, local_name) = parse_qualified_name(&element_name);
        
        // Resolve namespace if prefix exists
        let namespace = if let Some(prefix) = prefix {
            context.resolve_namespace(prefix)?
        } else {
            context.default_namespace.clone()
        };
        
        // Create the type name
        let type_name = XamlTypeName::new(namespace, local_name);
        
        // Create the element
        let mut element = XamlElement::new(type_name.clone());
        
        // Parse children until we hit the end tag
        loop {
            let event = reader.read_event()?;
            
            match event {
                XamlEvent::EndElement { name } => {
                    if name != element_name {
                        return Err(XamlError::XmlError {
                            line: 0,
                            col: 0,
                            message: format!("Mismatched tags: expected {}, got {}", element_name, name),
                        });
                    }
                    break;
                }
                
                XamlEvent::StartElement { name, attributes, is_empty } => {
                    // Check if this is a property element
                    if name.contains('.') {
                        self.parse_property_element(&mut element, &name, reader, context)?;
                    } else {
                        // Create child element with attributes
                        let child = self.parse_child_element(name, attributes, is_empty, reader, context)?;
                        element.add_child(XamlNode::Element(child));
                    }
                }
                
                XamlEvent::Text(text) => {
                    if self.has_flag(ParserFlags::PRESERVE_WHITESPACE) || !text.trim().is_empty() {
                        element.add_child(XamlNode::Text(text));
                    }
                }
                
                XamlEvent::Eof => {
                    return Err(XamlError::custom(format!("Unexpected EOF while parsing element {}", element_name)));
                }
            }
        }
        
        Ok(element)
    }
    
    /// Parse a child element with known attributes.
    fn parse_child_element<R: std::io::BufRead>(
        &self,
        element_name: String,
        attributes: Vec<(String, String)>,
        is_empty: bool,
        reader: &mut crate::reader::XamlReader<R>,
        context: &mut ParseContext<'_>,
    ) -> Result<crate::model::XamlElement> {
        use crate::reader::XamlEvent;
        use crate::model::{XamlElement, XamlNode};
        use crate::types::XamlTypeName;
        
        // Parse the element name (handle namespaces)
        let (prefix, local_name) = parse_qualified_name(&element_name);
        
        // Create the element with temporary type name
        let mut element = XamlElement::new(XamlTypeName::new("", local_name));
        
        // Process attributes FIRST to get any new namespace declarations
        for (attr_name, attr_value) in attributes {
            self.process_attribute(&mut element, &attr_name, &attr_value, context)?;
        }
        
        // NOW resolve the namespace
        let namespace = if let Some(prefix) = prefix {
            context.resolve_namespace(prefix)?
        } else {
            context.default_namespace.clone()
        };
        
        // Update the type name with resolved namespace
        element.type_name = XamlTypeName::new(namespace, local_name);
        
        // If not empty, parse children
        if !is_empty {
            loop {
                let event = reader.read_event()?;
                
                match event {
                    XamlEvent::EndElement { name } => {
                        if name != element_name {
                            return Err(XamlError::XmlError {
                                line: 0,
                                col: 0,
                                message: format!("Mismatched tags: expected {}, got {}", element_name, name),
                            });
                        }
                        break;
                    }
                    
                    XamlEvent::StartElement { name, attributes, is_empty } => {
                        if name.contains('.') {
                            self.parse_property_element(&mut element, &name, reader, context)?;
                        } else {
                            let child = self.parse_child_element(name, attributes, is_empty, reader, context)?;
                            element.add_child(XamlNode::Element(child));
                        }
                    }
                    
                    XamlEvent::Text(text) => {
                        if self.has_flag(ParserFlags::PRESERVE_WHITESPACE) || !text.trim().is_empty() {
                            element.add_child(XamlNode::Text(text));
                        }
                    }
                    
                    XamlEvent::Eof => {
                        return Err(XamlError::custom(format!("Unexpected EOF while parsing element {}", element_name)));
                    }
                }
            }
        }
        
        Ok(element)
    }
    
    /// Process an attribute on an element.
    fn process_attribute(
        &self,
        element: &mut crate::model::XamlElement,
        attr_name: &str,
        attr_value: &str,
        context: &mut ParseContext<'_>,
    ) -> Result<()> {
        // Handle xmlns declarations
        if attr_name == "xmlns" {
            context.default_namespace = attr_value.to_string();
            element.declare_namespace("", attr_value);
            return Ok(());
        }
        
        if attr_name.starts_with("xmlns:") {
            let prefix = &attr_name[6..];
            context.declare_namespace(prefix, attr_value);
            element.declare_namespace(prefix, attr_value);
            return Ok(());
        }
        
        // Handle x:Name
        if attr_name == "x:Name" || attr_name == "Name" {
            element.set_name(attr_value);
            return Ok(());
        }
        
        // Handle x:Key
        if attr_name == "x:Key" {
            element.set_key(attr_value);
            return Ok(());
        }
        
        // Parse the value
        let value = self.parse_attribute_value(attr_value, context)?;
        
        // Set the attribute
        element.set_attribute(attr_name, value);
        
        Ok(())
    }
    
    /// Parse an attribute value (may contain markup extensions).
    fn parse_attribute_value(
        &self,
        value: &str,
        _context: &ParseContext<'_>,
    ) -> Result<crate::model::XamlValue> {
        use crate::model::XamlValue;
        
        // Check if this is a markup extension
        if value.starts_with('{') && value.ends_with('}') {
            // TODO: Parse markup extension properly
            // For now, just store as string
            return Ok(XamlValue::String(value.to_string()));
        }
        
        // Try to parse as various types
        // Boolean
        if value == "true" || value == "True" {
            return Ok(XamlValue::Boolean(true));
        }
        if value == "false" || value == "False" {
            return Ok(XamlValue::Boolean(false));
        }
        
        // Integer
        if let Ok(i) = value.parse::<i64>() {
            return Ok(XamlValue::Integer(i));
        }
        
        // Float
        if let Ok(f) = value.parse::<f64>() {
            return Ok(XamlValue::Float(f));
        }
        
        // Default to string
        Ok(XamlValue::String(value.to_string()))
    }
    
    /// Parse a property element (e.g., <Button.Content>).
    fn parse_property_element<R: std::io::BufRead>(
        &self,
        element: &mut crate::model::XamlElement,
        property_name: &str,
        reader: &mut crate::reader::XamlReader<R>,
        context: &mut ParseContext<'_>,
    ) -> Result<()> {
        use crate::reader::XamlEvent;
        use crate::model::XamlValue;
        
        // Parse the property name (e.g., "Button.Content" -> "Content")
        let parts: Vec<&str> = property_name.split('.').collect();
        if parts.len() != 2 {
            return Err(XamlError::custom(format!("Invalid property element name: {}", property_name)));
        }
        
        let property_local_name = parts[1];
        
        // Read the property content
        let mut property_value: Option<XamlValue> = None;
        let mut text_content = String::new();
        
        loop {
            let event = reader.read_event()?;
            
            match event {
                XamlEvent::EndElement { name } => {
                    if name != property_name {
                        return Err(XamlError::XmlError {
                            line: 0,
                            col: 0,
                            message: format!("Mismatched property element tags: expected {}, got {}", property_name, name),
                        });
                    }
                    break;
                }
                
                XamlEvent::StartElement { name, attributes, is_empty } => {
                    // Parse the child element as the property value
                    let child = self.parse_child_element(name, attributes, is_empty, reader, context)?;
                    property_value = Some(XamlValue::Element(Box::new(child)));
                }
                
                XamlEvent::Text(text) => {
                    text_content.push_str(&text);
                }
                
                XamlEvent::Eof => {
                    return Err(XamlError::custom(format!("Unexpected EOF while parsing property element {}", property_name)));
                }
            }
        }
        
        // Set the property value
        let final_value = if let Some(val) = property_value {
            val
        } else if !text_content.trim().is_empty() {
            XamlValue::String(text_content)
        } else {
            XamlValue::Null
        };
        
        element.set_property(property_local_name, final_value);
        
        Ok(())
    }

    /// Get a reference to the type registry.
    pub fn type_registry(&self) -> &TypeRegistry {
        &self.registry
    }

    /// Get a reference to the parser settings.
    pub fn settings(&self) -> &ParserSettings {
        &self.settings
    }

    /// Check if a parser flag is enabled.
    pub fn has_flag(&self, flag: ParserFlags) -> bool {
        self.settings.flags.contains(flag)
    }
}

/// Context maintained during parsing.
#[allow(dead_code)]
struct ParseContext<'a> {
    /// Type registry reference.
    registry: &'a TypeRegistry,
    
    /// Parser settings reference.
    settings: &'a ParserSettings,
    
    /// Default namespace (from xmlns attribute).
    default_namespace: String,
    
    /// Namespace prefix mappings.
    namespace_map: std::collections::HashMap<String, String>,
    
    /// Resources collected during parsing.
    resources: std::collections::HashMap<String, crate::model::XamlValue>,
}

impl<'a> ParseContext<'a> {
    /// Create a new parse context.
    fn new(registry: &'a TypeRegistry, settings: &'a ParserSettings) -> Self {
        Self {
            registry,
            settings,
            default_namespace: String::new(),
            namespace_map: std::collections::HashMap::new(),
            resources: std::collections::HashMap::new(),
        }
    }
    
    /// Declare a namespace prefix mapping.
    fn declare_namespace(&mut self, prefix: impl Into<String>, uri: impl Into<String>) {
        self.namespace_map.insert(prefix.into(), uri.into());
    }
    
    /// Resolve a namespace prefix to its URI.
    fn resolve_namespace(&self, prefix: &str) -> Result<String> {
        self.namespace_map
            .get(prefix)
            .cloned()
            .ok_or_else(|| XamlError::InvalidNamespace {
                line: 0,
                details: format!("Undefined namespace prefix: {}", prefix),
            })
    }
}

/// Parse a qualified name into (prefix, local_name).
fn parse_qualified_name(name: &str) -> (Option<&str>, &str) {
    if let Some(colon_pos) = name.find(':') {
        let prefix = &name[..colon_pos];
        let local = &name[colon_pos + 1..];
        (Some(prefix), local)
    } else {
        (None, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_settings() {
        let settings = ParserSettings::default();
        assert!(settings.flags.contains(ParserFlags::STRICT_MODE));
        assert!(settings.flags.contains(ParserFlags::VALIDATE_TYPES));
        assert!(settings.flags.contains(ParserFlags::PARSE_MARKUP_EXTENSIONS));
    }

    #[test]
    fn test_builder_pattern() {
        let settings = ParserSettings::new()
            .lenient()
            .preserve_whitespace()
            .validate_namespaces();
        
        assert!(!settings.flags.contains(ParserFlags::STRICT_MODE));
        assert!(settings.flags.contains(ParserFlags::ALLOW_UNKNOWN_TYPES));
        assert!(settings.flags.contains(ParserFlags::PRESERVE_WHITESPACE));
        assert!(settings.flags.contains(ParserFlags::VALIDATE_NAMESPACES));
    }

    #[test]
    fn test_parser_creation() {
        let registry = TypeRegistry::new();
        let parser = XamlParser::new(registry);
        
        assert!(parser.has_flag(ParserFlags::STRICT_MODE));
        assert!(parser.has_flag(ParserFlags::VALIDATE_TYPES));
    }
}
