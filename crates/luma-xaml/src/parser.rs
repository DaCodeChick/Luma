//! XAML parser - parses XAML files and strings into object models.

use crate::model::XamlDocument;
use crate::types::TypeRegistry;
use crate::flags::ParserFlags;
use crate::error::Result;
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
    pub fn parse_file(&self, _path: &Path) -> Result<XamlDocument> {
        // TODO: Implement
        todo!("XAML file parsing not yet implemented")
    }

    /// Parse a XAML string.
    pub fn parse_string(&self, _xaml: &str) -> Result<XamlDocument> {
        // TODO: Implement
        todo!("XAML string parsing not yet implemented")
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
