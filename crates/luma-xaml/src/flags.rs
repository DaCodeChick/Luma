//! Bitflags for XAML parser configuration and element attributes.

use bitflags::bitflags;

bitflags! {
    /// Parser behavior flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ParserFlags: u32 {
        /// Strict mode: unknown types cause errors.
        const STRICT_MODE = 0b00000001;
        
        /// Validate that property types match expected types.
        const VALIDATE_TYPES = 0b00000010;
        
        /// Allow unknown types (create placeholder types).
        const ALLOW_UNKNOWN_TYPES = 0b00000100;
        
        /// Preserve whitespace in text content.
        const PRESERVE_WHITESPACE = 0b00001000;
        
        /// Validate namespace URIs against known schemas.
        const VALIDATE_NAMESPACES = 0b00010000;
        
        /// Enable markup extension parsing.
        const PARSE_MARKUP_EXTENSIONS = 0b00100000;
        
        /// Parse and resolve resource references.
        const RESOLVE_RESOURCES = 0b01000000;
        
        /// Default parser flags (strict, validate types, parse extensions, resolve resources).
        const DEFAULT = Self::STRICT_MODE.bits()
            | Self::VALIDATE_TYPES.bits()
            | Self::PARSE_MARKUP_EXTENSIONS.bits()
            | Self::RESOLVE_RESOURCES.bits();
    }
}

bitflags! {
    /// Element-level flags for tracking element state and attributes.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ElementFlags: u32 {
        /// Element has x:Name attribute.
        const HAS_NAME = 0b00000001;
        
        /// Element has x:Key attribute (resource dictionary item).
        const HAS_KEY = 0b00000010;
        
        /// Element is part of a resource dictionary.
        const IS_RESOURCE = 0b00000100;
        
        /// Element has namespace declarations.
        const HAS_NAMESPACES = 0b00001000;
        
        /// Element has child elements.
        const HAS_CHILDREN = 0b00010000;
        
        /// Element has text content.
        const HAS_TEXT_CONTENT = 0b00100000;
        
        /// Element is a collection type.
        const IS_COLLECTION = 0b01000000;
        
        /// Element uses content property syntax (implicit property).
        const USES_CONTENT_PROPERTY = 0b10000000;
    }
}

bitflags! {
    /// Property-level flags for tracking property characteristics.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct PropertyFlags: u32 {
        /// Property is an attached property (e.g., Grid.Row).
        const ATTACHED = 0b00000001;
        
        /// Property is read-only.
        const READONLY = 0b00000010;
        
        /// Property is a dependency property (WPF/WinUI).
        const DEPENDENCY_PROPERTY = 0b00000100;
        
        /// Property is a collection property.
        const COLLECTION = 0b00001000;
        
        /// Property is the content property for its parent type.
        const CONTENT_PROPERTY = 0b00010000;
        
        /// Property value is set via markup extension.
        const FROM_MARKUP_EXTENSION = 0b00100000;
        
        /// Property value is a resource reference.
        const FROM_RESOURCE = 0b01000000;
        
        /// Property value is data-bound.
        const DATA_BOUND = 0b10000000;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_flags() {
        let flags = ParserFlags::DEFAULT;
        assert!(flags.contains(ParserFlags::STRICT_MODE));
        assert!(flags.contains(ParserFlags::VALIDATE_TYPES));
        assert!(flags.contains(ParserFlags::PARSE_MARKUP_EXTENSIONS));
        assert!(!flags.contains(ParserFlags::ALLOW_UNKNOWN_TYPES));
    }

    #[test]
    fn test_parser_flags_combination() {
        let mut flags = ParserFlags::empty();
        flags.insert(ParserFlags::STRICT_MODE);
        flags.insert(ParserFlags::VALIDATE_NAMESPACES);
        
        assert!(flags.contains(ParserFlags::STRICT_MODE));
        assert!(flags.contains(ParserFlags::VALIDATE_NAMESPACES));
        assert!(!flags.contains(ParserFlags::VALIDATE_TYPES));
    }

    #[test]
    fn test_element_flags() {
        let mut flags = ElementFlags::empty();
        flags.insert(ElementFlags::HAS_NAME);
        flags.insert(ElementFlags::HAS_CHILDREN);
        
        assert!(flags.contains(ElementFlags::HAS_NAME));
        assert!(flags.contains(ElementFlags::HAS_CHILDREN));
        assert!(!flags.contains(ElementFlags::HAS_KEY));
    }

    #[test]
    fn test_property_flags() {
        let flags = PropertyFlags::ATTACHED | PropertyFlags::READONLY;
        
        assert!(flags.contains(PropertyFlags::ATTACHED));
        assert!(flags.contains(PropertyFlags::READONLY));
        assert!(!flags.contains(PropertyFlags::COLLECTION));
    }
}
