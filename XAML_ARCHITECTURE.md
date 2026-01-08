# Luma XAML Architecture Design

**Version**: 0.1.0  
**Date**: January 8, 2026  
**Status**: Design Phase  
**Philosophy**: Quality over speed - passion project with no deadline

---

## Executive Summary

This document outlines the architecture for two new Luma crates:

1. **luma-xaml**: A pure, generic XAML parser and manipulation library
2. **luma-winui**: A WinUI 3 backend for Luma that consumes luma-xaml

These crates will coexist with the existing Win32 backend (`luma-windows`), providing an alternative path to modern Windows 11 UI.

---

## Table of Contents

1. [Project Goals](#project-goals)
2. [Architecture Overview](#architecture-overview)
3. [luma-xaml: XAML Parser](#luma-xaml-xaml-parser)
4. [luma-winui: WinUI 3 Backend](#luma-winui-winui-3-backend)
5. [Integration Strategy](#integration-strategy)
6. [Implementation Phases](#implementation-phases)
7. [Risk Mitigation](#risk-mitigation)
8. [Dependencies](#dependencies)
9. [Success Criteria](#success-criteria)

---

## Project Goals

### Primary Goals

1. **Create a high-quality XAML parser for Rust**
   - Support multiple XAML dialects (WinUI 3, WPF, generic)
   - Clean, idiomatic Rust API
   - Comprehensive error handling
   - Well-documented and tested

2. **Enable WinUI 3 support in Luma**
   - Modern Windows 11 UI with Fluent Design
   - Native dark mode support
   - Alternative backend alongside Win32
   - Integration with Luma's layout system

3. **Contribute to Rust Ecosystem**
   - Standalone luma-xaml usable by other projects
   - Showcase Rust's capability for Windows UI development
   - Document challenges and solutions for future developers

### Non-Goals (For Initial Release)

- XAML authoring/writing (only parsing/reading)
- Visual XAML designer (use luma-editor with JSON instead)
- Cross-platform XAML (Windows-only for WinUI/WPF)
- Complete XAML 2009 specification (implement subset)
- .xaml.cs code-behind files (use Rust callbacks instead)

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                        Luma Application                      │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                         luma-gui                             │
│              (Platform-agnostic public API)                  │
└─────────────────────────────────────────────────────────────┘
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
┌──────────────────────────┐  ┌──────────────────────────┐
│     luma-windows         │  │      luma-winui          │
│   (Win32 Backend)        │  │  (WinUI 3 Backend)       │
│                          │  │                          │
│  • Stable, battle-tested │  │  • Modern UI             │
│  • Windows 7+            │  │  • Windows 10+           │
│  • Light theme           │  │  • Dark mode support     │
└──────────────────────────┘  └──────────────────────────┘
                                           │
                                           ▼
                              ┌──────────────────────────┐
                              │      luma-xaml           │
                              │   (XAML Parser)          │
                              │                          │
                              │  • Generic XAML parsing  │
                              │  • WinUI 3 dialect       │
                              │  • WPF dialect           │
                              │  • Type system           │
                              │  • Markup extensions     │
                              └──────────────────────────┘
                                           │
                                           ▼
                              ┌──────────────────────────┐
                              │    XML Parser (quick-xml)│
                              └──────────────────────────┘
```

### Key Architectural Decisions

1. **Separation of Concerns**
   - `luma-xaml`: Pure parsing, no UI rendering
   - `luma-winui`: UI rendering, no parsing logic
   - Clean boundaries enable reuse and testing

2. **Backend Coexistence**
   - Win32 and WinUI backends coexist
   - Users choose at compile time or runtime
   - No breaking changes to existing Win32 code

3. **XAML Dialect Support**
   - Core parser is dialect-agnostic
   - Dialect-specific extensions as modules
   - Start with WinUI 3, add WPF and generic later

4. **Quality Focus**
   - Comprehensive error messages
   - Full test coverage
   - Documentation with examples
   - Performance profiling

---

## luma-xaml: XAML Parser

### Overview

`luma-xaml` is a pure Rust library for parsing and manipulating XAML (eXtensible Application Markup Language). It provides a generic foundation that supports multiple XAML dialects.

### Design Principles

1. **Dialect Agnostic Core**: Parser handles common XAML features
2. **Extensible**: Support for custom types and markup extensions
3. **Zero Unsafe**: Pure safe Rust (no FFI in this crate)
4. **Error Recovery**: Helpful error messages with context
5. **Performance**: Lazy evaluation where possible
6. **Standards Compliant**: Follow XAML 2009 spec where practical

### Core Components

#### 1. XML Foundation

```rust
// Low-level XML parsing
use quick_xml::Reader;

pub struct XamlReader {
    reader: Reader<BufReader<File>>,
    // Namespace tracking
    // Position tracking for errors
}
```

#### 2. XAML Object Model

```rust
/// Represents a XAML element (e.g., <Button>)
pub struct XamlElement {
    pub type_name: XamlTypeName,
    pub attributes: HashMap<String, XamlValue>,
    pub properties: HashMap<String, XamlValue>,
    pub children: Vec<XamlNode>,
    pub namespaces: HashMap<String, String>,
}

/// Represents a XAML type (e.g., Button, String, Int32)
pub struct XamlTypeName {
    pub namespace: String,  // e.g., "Microsoft.UI.Xaml.Controls"
    pub name: String,       // e.g., "Button"
    pub type_args: Vec<XamlTypeName>, // For generics
}

/// A node in the XAML tree
pub enum XamlNode {
    Element(XamlElement),
    Text(String),
    MarkupExtension(Box<dyn MarkupExtension>),
}

/// A value in XAML (property value, attribute, etc.)
pub enum XamlValue {
    String(String),
    Element(XamlElement),
    MarkupExtension(Box<dyn MarkupExtension>),
    Collection(Vec<XamlValue>),
}
```

#### 3. Type System

```rust
/// XAML type metadata
pub trait XamlType {
    fn name(&self) -> &XamlTypeName;
    fn base_type(&self) -> Option<&XamlTypeName>;
    fn properties(&self) -> &[XamlProperty];
    fn is_collection(&self) -> bool;
    fn content_property(&self) -> Option<&str>;
}

/// XAML property metadata
pub struct XamlProperty {
    pub name: String,
    pub type_name: XamlTypeName,
    pub is_attached: bool,
    pub is_readonly: bool,
}

/// Type registry for dialect-specific types
pub struct TypeRegistry {
    types: HashMap<XamlTypeName, Box<dyn XamlType>>,
    namespaces: HashMap<String, String>, // prefix -> URI
}

impl TypeRegistry {
    pub fn register_type(&mut self, xaml_type: Box<dyn XamlType>);
    pub fn lookup_type(&self, name: &XamlTypeName) -> Option<&dyn XamlType>;
    pub fn resolve_namespace(&self, prefix: &str) -> Option<&str>;
}
```

#### 4. Markup Extensions

```rust
/// Base trait for markup extensions (e.g., {Binding}, {StaticResource})
pub trait MarkupExtension: Debug {
    fn extension_name(&self) -> &str;
    fn provide_value(&self, context: &ServiceProvider) -> Result<XamlValue>;
}

/// Common markup extensions
pub struct StaticResourceExtension {
    pub key: String,
}

pub struct BindingExtension {
    pub path: String,
    pub mode: BindingMode,
    pub source: Option<String>,
}

pub struct TemplateBindingExtension {
    pub property: String,
}

pub struct NullExtension;

pub struct TypeExtension {
    pub type_name: XamlTypeName,
}
```

#### 5. Parser API

```rust
/// Main XAML parser
pub struct XamlParser {
    registry: TypeRegistry,
    settings: ParserSettings,
}

pub struct ParserSettings {
    pub strict_mode: bool,
    pub validate_types: bool,
    pub allow_unknown_types: bool,
}

impl XamlParser {
    pub fn new(registry: TypeRegistry) -> Self;
    
    pub fn parse_file(&self, path: &Path) -> Result<XamlDocument>;
    pub fn parse_string(&self, xaml: &str) -> Result<XamlDocument>;
    
    pub fn with_settings(mut self, settings: ParserSettings) -> Self;
}

/// Parsed XAML document
pub struct XamlDocument {
    pub root: XamlElement,
    pub resources: HashMap<String, XamlValue>,
}
```

### XAML Dialect Support

#### WinUI 3 Dialect

```rust
pub mod dialects {
    pub mod winui3 {
        use super::*;
        
        /// Create a type registry pre-populated with WinUI 3 types
        pub fn create_type_registry() -> TypeRegistry {
            let mut registry = TypeRegistry::new();
            
            // Register core types
            registry.register_type(Box::new(ButtonType));
            registry.register_type(Box::new(TextBlockType));
            registry.register_type(Box::new(StackPanelType));
            // ... more types
            
            // Register namespaces
            registry.register_namespace(
                "http://schemas.microsoft.com/winfx/2006/xaml/presentation",
                "default"
            );
            
            registry
        }
    }
    
    pub mod wpf {
        // Similar for WPF dialect
    }
    
    pub mod generic {
        // Minimal type registry for generic XAML
    }
}
```

### Error Handling

```rust
#[derive(Error, Debug)]
pub enum XamlError {
    #[error("XML parsing error at line {line}, column {col}: {message}")]
    XmlError {
        line: usize,
        col: usize,
        message: String,
    },
    
    #[error("Unknown type '{type_name}' at line {line}")]
    UnknownType {
        type_name: String,
        line: usize,
    },
    
    #[error("Unknown property '{property}' on type '{type_name}' at line {line}")]
    UnknownProperty {
        type_name: String,
        property: String,
        line: usize,
    },
    
    #[error("Invalid markup extension syntax at line {line}: {details}")]
    InvalidMarkupExtension {
        line: usize,
        details: String,
    },
    
    #[error("Type mismatch: expected {expected}, got {actual} at line {line}")]
    TypeMismatch {
        expected: String,
        actual: String,
        line: usize,
    },
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, XamlError>;
```

### File Structure

```
luma-xaml/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Public API exports
│   ├── parser.rs           # XamlParser implementation
│   ├── reader.rs           # XamlReader (XML wrapper)
│   ├── model/              # Object model
│   │   ├── mod.rs
│   │   ├── element.rs      # XamlElement
│   │   ├── node.rs         # XamlNode
│   │   ├── value.rs        # XamlValue
│   │   └── document.rs     # XamlDocument
│   ├── types/              # Type system
│   │   ├── mod.rs
│   │   ├── xaml_type.rs    # XamlType trait
│   │   ├── type_name.rs    # XamlTypeName
│   │   ├── property.rs     # XamlProperty
│   │   └── registry.rs     # TypeRegistry
│   ├── markup/             # Markup extensions
│   │   ├── mod.rs
│   │   ├── extension.rs    # MarkupExtension trait
│   │   ├── static_resource.rs
│   │   ├── binding.rs
│   │   ├── template_binding.rs
│   │   └── builtin.rs      # Null, Type, etc.
│   ├── dialects/           # Dialect-specific support
│   │   ├── mod.rs
│   │   ├── winui3/
│   │   │   ├── mod.rs
│   │   │   ├── types.rs    # WinUI 3 type definitions
│   │   │   └── registry.rs # Pre-built registry
│   │   ├── wpf/
│   │   │   ├── mod.rs
│   │   │   ├── types.rs
│   │   │   └── registry.rs
│   │   └── generic/
│   │       ├── mod.rs
│   │       └── registry.rs
│   ├── error.rs            # Error types
│   └── context.rs          # ServiceProvider for extensions
├── tests/
│   ├── winui3_parsing.rs
│   ├── wpf_parsing.rs
│   ├── markup_extensions.rs
│   └── error_handling.rs
└── examples/
    ├── parse_winui3.rs
    ├── parse_wpf.rs
    └── custom_types.rs
```

---

## luma-winui: WinUI 3 Backend

### Overview

`luma-winui` is a WinUI 3 backend for Luma that implements the framework's trait interfaces using Windows App SDK. It consumes `luma-xaml` for declarative UI support.

### Design Principles

1. **Programmatic First**: Core API doesn't require XAML
2. **XAML Optional**: Can load XAML via luma-xaml
3. **Safe Wrappers**: Wrap unsafe WinRT calls in safe Rust
4. **Luma Integration**: Implement all Luma backend traits
5. **Proof-of-Concept**: Start small, expand incrementally

### Core Components

#### 1. WinUI 3 Initialization

```rust
use windows::Win32::System::WinRT::*;
use windows::ApplicationModel::*;

pub struct WinUIRuntime {
    // Windows App SDK initialization
}

impl WinUIRuntime {
    pub fn initialize() -> Result<Self> {
        // Initialize Windows App SDK
        // Set up IXamlMetadataProvider
        // Register application
    }
}
```

#### 2. Window Implementation

```rust
use windows::UI::Xaml::*;
use luma_core::traits::WindowBackend;

pub struct WinUIWindow {
    window: Window,
    content: Option<UIElement>,
}

impl WindowBackend for WinUIWindow {
    fn new(title: &str, width: u32, height: u32) -> Result<Self>;
    fn show(&self) -> Result<()>;
    fn hide(&self) -> Result<()>;
    fn set_title(&mut self, title: &str) -> Result<()>;
    // ... other trait methods
}
```

#### 3. Widget Implementations

```rust
pub struct WinUIButton {
    button: Button,
    click_handler: Option<Box<dyn FnMut()>>,
}

pub struct WinUITextBlock {
    text_block: TextBlock,
}

pub struct WinUIStackPanel {
    stack_panel: StackPanel,
    children: Vec<Box<dyn Widget>>,
}

// ... more widgets
```

#### 4. XAML Integration

```rust
use luma_xaml::{XamlParser, dialects::winui3};

pub struct XamlLoader {
    parser: XamlParser,
    runtime: WinUIRuntime,
}

impl XamlLoader {
    pub fn new(runtime: WinUIRuntime) -> Self {
        let registry = winui3::create_type_registry();
        let parser = XamlParser::new(registry);
        Self { parser, runtime }
    }
    
    pub fn load_window(&self, xaml_path: &Path) -> Result<WinUIWindow> {
        let doc = self.parser.parse_file(xaml_path)?;
        self.instantiate_window(&doc)
    }
    
    fn instantiate_window(&self, doc: &XamlDocument) -> Result<WinUIWindow> {
        // Convert XamlDocument to actual WinUI objects
    }
}
```

### File Structure

```
luma-winui/
├── Cargo.toml
├── README.md
├── src/
│   ├── lib.rs              # Public API
│   ├── runtime.rs          # WinUIRuntime initialization
│   ├── window.rs           # WinUIWindow
│   ├── application.rs      # WinUIApplication
│   ├── widgets/            # Widget implementations
│   │   ├── mod.rs
│   │   ├── button.rs
│   │   ├── text_block.rs
│   │   ├── text_box.rs
│   │   ├── checkbox.rs
│   │   ├── stack_panel.rs
│   │   └── grid.rs
│   ├── xaml/               # XAML integration
│   │   ├── mod.rs
│   │   ├── loader.rs       # XamlLoader
│   │   └── instantiator.rs # Convert XamlDocument to WinUI
│   ├── layout.rs           # Layout integration with Luma
│   ├── error.rs            # Error types
│   └── utils.rs            # Helper functions
├── tests/
│   ├── window_test.rs
│   ├── widgets_test.rs
│   └── xaml_loading_test.rs
└── examples/
    ├── hello_window.rs     # Programmatic API
    ├── button_demo.rs
    └── xaml_window.rs      # XAML loading
```

---

## Integration Strategy

### Backend Selection

Users can choose backend at compile-time or runtime:

#### Compile-Time Selection

```rust
// luma-gui/src/lib.rs
cfg_if! {
    if #[cfg(feature = "winui")] {
        pub(crate) use luma_winui::*;
    } else if #[cfg(windows)] {
        pub(crate) use luma_windows::*;
    } else {
        compile_error!("No backend selected");
    }
}
```

#### Runtime Selection

```rust
use luma_gui::Application;

fn main() -> Result<()> {
    // Explicit backend choice
    let app = Application::with_backend(Backend::WinUI)?;
    // Or auto-detect
    let app = Application::new()?; // Defaults to Win32
    
    // Rest of app...
}
```

### Layout System Integration

WinUI 3 has its own layout system (StackPanel, Grid), but we'll map Luma's layout system to it:

```rust
impl Container for WinUIStackPanel {
    fn add(&mut self, widget: Box<dyn Widget>, constraints: LayoutConstraints) {
        // Map Luma constraints to WinUI properties
        let element = widget.as_ui_element();
        
        if constraints.expand_horizontal {
            element.set_horizontal_alignment(HorizontalAlignment::Stretch);
        }
        
        if let Some(width) = constraints.preferred_width {
            element.set_width(width as f64);
        }
        
        self.stack_panel.children().append(element)?;
    }
}
```

---

## Implementation Phases

### Phase 1: Foundation (Month 1-2)

**Goal**: Create project structure and basic parsing

#### Tasks

1. **Project Setup**
   - Create `luma-xaml` crate structure
   - Create `luma-winui` crate structure
   - Update workspace `Cargo.toml`
   - Set up dependencies

2. **luma-xaml: Core Parser**
   - XML reader wrapper (quick-xml)
   - Basic element parsing
   - Attribute parsing
   - Namespace handling
   - Error types with context

3. **luma-xaml: Object Model**
   - XamlElement, XamlNode, XamlValue
   - XamlDocument structure
   - Tree traversal utilities

4. **Testing**
   - Unit tests for parser
   - Sample XAML files (simple)
   - Error message verification

#### Success Criteria

- [ ] Can parse simple XAML (elements, attributes, text)
- [ ] Proper error messages with line numbers
- [ ] All tests pass
- [ ] Documentation complete

---

### Phase 2: Type System (Month 3-4)

**Goal**: Implement XAML type system and WinUI 3 dialect

#### Tasks

1. **Type System Core**
   - XamlType trait
   - XamlTypeName with generics
   - XamlProperty metadata
   - TypeRegistry

2. **WinUI 3 Type Definitions**
   - Core WinUI 3 types (Button, TextBlock, etc.)
   - Layout containers (StackPanel, Grid)
   - Property metadata
   - Content properties

3. **Type Validation**
   - Property type checking
   - Unknown type detection
   - Helpful error messages

4. **Testing**
   - Type resolution tests
   - WinUI 3 specific parsing
   - Invalid type error handling

#### Success Criteria

- [ ] Type registry working
- [ ] 10+ WinUI 3 types defined
- [ ] Type validation functional
- [ ] Comprehensive tests

---

### Phase 3: Markup Extensions (Month 5-6)

**Goal**: Support common XAML markup extensions

#### Tasks

1. **Extension Infrastructure**
   - MarkupExtension trait
   - ServiceProvider context
   - Extension parsing from attributes

2. **Core Extensions**
   - StaticResource
   - Binding
   - TemplateBinding
   - Null
   - Type

3. **Resource System**
   - Resource dictionary parsing
   - Resource lookup
   - Merged dictionaries

4. **Testing**
   - Each extension type
   - Complex binding scenarios
   - Resource resolution

#### Success Criteria

- [ ] All core extensions implemented
- [ ] Resource system working
- [ ] Binding syntax parsed correctly
- [ ] Tests pass

---

### Phase 4: WinUI 3 Proof-of-Concept (Month 7-9)

**Goal**: Get basic WinUI 3 backend working

**CRITICAL DECISION POINT**: If this phase fails (crashes, instability), we pivot back to Win32.

#### Tasks

1. **Windows App SDK Setup**
   - Initialize Windows App SDK
   - IXamlMetadataProvider implementation
   - Application lifecycle

2. **Window + 3 Widgets**
   - WinUIWindow (programmatic)
   - WinUIButton
   - WinUITextBlock
   - WinUIStackPanel

3. **Stability Testing**
   - 100+ hours of testing
   - Windows 10 21H2, 22H2
   - Windows 11 21H2, 22H2
   - Document all quirks/crashes

4. **Luma Integration**
   - Implement backend traits
   - Create example app
   - Compare with Win32 version

#### Success Criteria (ALL MUST PASS)

- [ ] Window creates without crash
- [ ] Widgets render correctly
- [ ] Events (button click) work
- [ ] No unexplained crashes in 100 hours
- [ ] Works on all test Windows versions
- [ ] Performance acceptable (<16ms frame time)

**If ANY criterion fails**: Document findings, stop WinUI development, return to Win32.

---

### Phase 5: XAML Loading (Month 10-11)

**Prerequisites**: Phase 4 POC succeeded

#### Tasks

1. **XamlLoader Implementation**
   - Convert XamlDocument to WinUI objects
   - Property setting from XAML
   - Children/content handling

2. **Full Widget Set**
   - TextBox, CheckBox, ListBox
   - Grid, Border, ScrollViewer
   - Complete WinUI 3 basics

3. **Example Applications**
   - Load window from XAML file
   - Resource dictionaries
   - Complex layouts

4. **Testing**
   - Round-trip: XAML → objects → verify
   - Error handling in XAML files
   - Integration tests

#### Success Criteria

- [ ] Can load window from XAML
- [ ] All basic widgets supported
- [ ] Resources work
- [ ] Example apps functional

---

### Phase 6: WPF Dialect (Month 12-14)

**Goal**: Add WPF XAML support

#### Tasks

1. **WPF Type Registry**
   - WPF-specific types
   - WPF namespaces
   - Dialect differences

2. **WPF-Specific Features**
   - DependencyProperty handling
   - Triggers (if applicable)
   - Styles

3. **Testing**
   - Parse WPF XAML files
   - Validate against WinUI 3 dialect
   - Cross-dialect tests

#### Success Criteria

- [ ] WPF XAML parses correctly
- [ ] Dialect selection works
- [ ] No WinUI/WPF confusion
- [ ] Documentation clear

---

### Phase 7: Generic XAML (Month 15-16)

**Goal**: Support generic XAML (custom applications)

#### Tasks

1. **Minimal Generic Registry**
   - Basic types (string, int, etc.)
   - Object type
   - Collection types

2. **Custom Type Registration API**
   - User-defined types
   - Custom properties
   - Custom markup extensions

3. **Documentation & Examples**
   - How to define custom types
   - Example custom XAML app
   - Best practices

#### Success Criteria

- [ ] Generic XAML support
- [ ] Custom types API documented
- [ ] Example demonstrates usage

---

### Phase 8: Polish & Release (Month 17-18)

**Goal**: Production-ready 0.1.0 release

#### Tasks

1. **Documentation**
   - API documentation (rustdoc)
   - User guides for both crates
   - Architecture documentation
   - Migration guide from Win32

2. **Performance**
   - Profile XAML parsing
   - Optimize hot paths
   - Benchmark against goals

3. **Testing**
   - Fuzzing XAML parser
   - Property-based tests
   - Integration test suite

4. **Examples Gallery**
   - Showcase each feature
   - Side-by-side Win32/WinUI
   - Best practices demos

5. **Release Preparation**
   - Version to 0.1.0
   - Release notes
   - Publish to crates.io

#### Success Criteria

- [ ] All documentation complete
- [ ] Performance goals met
- [ ] No known critical bugs
- [ ] Ready for community use

---

## Risk Mitigation

### Known Risks from Research

1. **Undocumented WinUI Behavior** (CRITICAL)
   - **Mitigation**: Phase 4 is explicit POC with exit criteria
   - **Fallback**: Keep Win32 backend fully functional
   - **Timeline**: If POC fails, ~9 months saved by stopping

2. **Breaking Changes in WinUI 3**
   - **Mitigation**: Pin to specific Windows App SDK version
   - **Strategy**: Version luma-winui alongside WinUI releases
   - **Documentation**: Clearly state version requirements

3. **Limited Community Support**
   - **Mitigation**: Comprehensive documentation
   - **Strategy**: Be first, set standards for ecosystem
   - **Community**: Share findings, help others

4. **Deployment Complexity**
   - **Mitigation**: Document deployment thoroughly
   - **Provide**: Example installer/manifest templates
   - **Automation**: Scripts for common scenarios

### Additional Risks

5. **Parser Complexity**
   - **Mitigation**: Start simple, add features incrementally
   - **Testing**: Extensive test suite with edge cases
   - **Reference**: Compare with C# XAML parser behavior

6. **Performance**
   - **Mitigation**: Profile early and often
   - **Target**: Parse typical XAML in <100ms
   - **Optimize**: Lazy evaluation, caching

7. **Scope Creep**
   - **Mitigation**: Clear phase boundaries
   - **Discipline**: Defer features to later versions
   - **Focus**: Core functionality first

---

## Dependencies

### luma-xaml Dependencies

```toml
[dependencies]
# XML parsing
quick-xml = "0.31"

# Error handling
thiserror = "1.0"

# Utilities
once_cell = "1.19"

[dev-dependencies]
# Testing
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### luma-winui Dependencies

```toml
[dependencies]
# Core Luma
luma-core = { path = "../luma-core", version = "0.0.1" }
luma-xaml = { path = "../luma-xaml", version = "0.0.1" }

# Windows App SDK
windows = { version = "0.52", features = [
    "Win32_Foundation",
    "Win32_System_WinRT",
    "ApplicationModel",
    "UI_Xaml",
    "UI_Xaml_Controls",
    "UI_Xaml_Markup",
    # ... more as needed
] }

# Error handling
thiserror = "1.0"

# Utilities
once_cell = "1.19"
```

---

## Success Criteria

### luma-xaml Success Criteria

- [ ] Parses valid XAML correctly (WinUI 3, WPF, generic)
- [ ] Clear, actionable error messages
- [ ] Performance: Parse 10KB XAML in <50ms
- [ ] Test coverage >80%
- [ ] Documentation complete with examples
- [ ] Zero unsafe code
- [ ] No panics on invalid input

### luma-winui Success Criteria

- [ ] Implements all Luma backend traits
- [ ] Window + 10 widgets working
- [ ] XAML loading functional
- [ ] Stable (no crashes in 100+ hours testing)
- [ ] Performance: <16ms frame time
- [ ] Works on Windows 10/11
- [ ] Integration with Luma's layout system
- [ ] Example apps demonstrate features

### Overall Project Success

- [ ] Both crates published to crates.io
- [ ] luma-xaml reusable by other projects
- [ ] luma-winui viable alternative to Win32
- [ ] Comprehensive documentation
- [ ] Positive community feedback
- [ ] Contributes to Rust GUI ecosystem

---

## Next Steps

1. **Review this document**: Ensure alignment with vision
2. **Create initial crate structure**: Empty crates with Cargo.toml
3. **Begin Phase 1**: Start with XML reader wrapper
4. **Iterate**: Build incrementally, test continuously

---

## Appendix: XAML Example

### Target WinUI 3 XAML

```xml
<Window
    x:Class="MyApp.MainWindow"
    xmlns="http://schemas.microsoft.com/winfx/2006/xaml/presentation"
    xmlns:x="http://schemas.microsoft.com/winfx/2006/xaml"
    Title="Hello Luma">
    
    <Window.Resources>
        <SolidColorBrush x:Key="AccentBrush" Color="#0078D4"/>
    </Window.Resources>
    
    <StackPanel Orientation="Vertical" Spacing="10">
        <TextBlock Text="Welcome to Luma!" FontSize="24"/>
        
        <Button Content="Click Me" 
                Background="{StaticResource AccentBrush}"
                HorizontalAlignment="Center"/>
        
        <Grid ColumnDefinitions="Auto,*">
            <TextBlock Text="Name:" Grid.Column="0"/>
            <TextBox Grid.Column="1" PlaceholderText="Enter name"/>
        </Grid>
    </StackPanel>
</Window>
```

### Parsed Result (Conceptual)

```rust
XamlDocument {
    root: XamlElement {
        type_name: XamlTypeName {
            namespace: "Microsoft.UI.Xaml",
            name: "Window",
        },
        attributes: {
            "Title": XamlValue::String("Hello Luma"),
        },
        properties: {
            "Resources": XamlValue::Collection([
                XamlElement {
                    type_name: XamlTypeName { ... "SolidColorBrush" },
                    attributes: {
                        "x:Key": "AccentBrush",
                        "Color": "#0078D4",
                    },
                },
            ]),
        },
        children: [
            XamlNode::Element(XamlElement {
                type_name: XamlTypeName { ... "StackPanel" },
                children: [
                    XamlNode::Element(/* TextBlock */),
                    XamlNode::Element(/* Button */),
                    XamlNode::Element(/* Grid */),
                ],
            }),
        ],
    },
    resources: {
        "AccentBrush": /* SolidColorBrush */,
    },
}
```

---

**End of Document**

This living document will be updated as the project evolves. Feedback and revisions are expected and encouraged.
