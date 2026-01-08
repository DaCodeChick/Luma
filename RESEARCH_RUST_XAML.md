# Research: Feasibility of Rust XAML Crate

**Date:** January 8, 2026  
**Question:** Should we build a Rust XAML crate to enable WinUI 3 support?

---

## Cost-Benefit Analysis

### Costs

#### Development Time
- **XAML Parser**: 2-3 months
- **Type Metadata System**: 2-3 months  
- **Dependency Property System**: 1-2 months
- **Build-Time Code Generation**: 1-2 months
- **Resource Management**: 1 month
- **Testing & Debugging**: 2-3 months
- **Maintenance (per year)**: 2-3 months (tracking WinUI changes)

**Total Initial Effort:** 9-14 months full-time work  
**Ongoing Maintenance:** 2-3 months/year (forever)

#### Technical Risks

1. **Undocumented Behavior** (CRITICAL)
   - WinUI makes hidden assumptions about host process
   - These change between releases without documentation
   - No source code access = must reverse engineer
   - Microsoft themselves called this "unsustainable"

2. **Incomplete Fusion Manifests**
   - Windows App Runtime requires specific manifests
   - Cargo doesn't generate these automatically
   - Deployment complexity for end users

3. **Breaking Changes**
   - WinUI 3 is still evolving
   - Each release can break existing code
   - No API stability guarantees yet

4. **Limited Community Support**
   - Zero Rust developers using WinUI 3 currently
   - You'd be the first = all bugs are yours to solve
   - Microsoft won't provide support for Rust

#### Competitive Disadvantage

Other frameworks ship TODAY while you're building XAML support for 9-14 months:
- **egui**: Already cross-platform, immediate mode, mature
- **iced**: Already cross-platform, declarative, growing fast
- **Slint**: Already cross-platform, declarative UI, commercial support
- **Tauri**: Webview approach, massive ecosystem, production-ready

### Benefits

#### What You Get

1. **True Native Windows 11 UI**
   - Fluent Design System out of the box
   - Full dark mode support
   - Modern WinUI 3 controls (NavigationView, InfoBar, etc.)
   - Acrylic, Mica materials

2. **Windows-Only Advantage**
   - Best possible Windows experience
   - Latest UI paradigms
   - Windows 11 integration

#### What You DON'T Get

- ❌ Cross-platform support (Windows 10+ only)
- ❌ Stability (WinUI 3 still evolving)
- ❌ Community support (you'd be first)
- ❌ Microsoft support (Rust not officially supported)
- ❌ Source code access (can't debug deep issues)

---

## Alternative Approaches

### 1. **Win32 + Custom Rendering** (Recommended for Luma)

**Effort:** 3-4 months (Phase 4+)  
**Risk:** Low  
**Result:** Full dark mode, custom styling, stable

```rust
// Owner-drawn controls
impl Button {
    fn paint(&self, hdc: HDC) {
        // Custom rendering with dark theme
        draw_button_background(hdc, self.is_dark_mode);
        draw_button_text(hdc, &self.label, self.text_color);
    }
}
```

**Pros:**
- Complete control over appearance
- Works on Windows 7+ (wider compatibility)
- Stable Win32 API
- Can match Windows 11 design language

**Cons:**
- More manual work
- Need to implement each control's rendering
- No automatic Windows updates

### 2. **DirectComposition API** (Microsoft's Suggestion)

**Effort:** 6-9 months  
**Risk:** Medium  
**Result:** Hardware-accelerated, modern, stable

Microsoft's own recommendation:
> "I think the sustainable way forward is to throw away XAML and build on top of DirectComposition."

DirectComposition = Windows' compositing engine (used by DWM, WinUI itself)

**Pros:**
- Hardware-accelerated
- Modern API (Windows 8+)
- Stable, documented
- Full control over rendering
- Can achieve WinUI-like appearance

**Cons:**
- Lower-level API (more work)
- Still Windows-only
- Need to build UI framework from scratch

### 3. **WebView2 Backend** (Electron-like)

**Effort:** 2-3 months  
**Risk:** Low  
**Result:** Full styling control, web technologies

```rust
// Use Chromium for rendering
use webview2::prelude::*;

let webview = WebView2::builder()
    .html(include_str!("ui.html"))
    .build()?;
```

**Pros:**
- Full CSS styling (including dark mode)
- Mature web technologies
- Cross-platform possible (with platform-specific webviews)
- Large ecosystem (HTML/CSS/JS)

**Cons:**
- Larger memory footprint
- Not "truly native" feel
- Requires WebView2 runtime

### 4. **Hybrid: Win32 + WinUI 3 Controls** (Experimental)

**Effort:** 4-6 months  
**Risk:** High  
**Result:** Mix of stable Win32 + select WinUI controls

Use Win32 for main window/layout, embed specific WinUI 3 controls:

```rust
// Win32 window
let window = Win32Window::new()?;

// Embed WinUI control for specific features
let color_picker = WinUI3::ColorPicker::new()?;
window.embed_control(color_picker, x, y)?;
```

**Pros:**
- Get specific modern controls
- Win32 stability for core framework
- Gradual adoption

**Cons:**
- Still faces WinUI 3 challenges
- Integration complexity
- Mixing two frameworks

---

## Recommendation

### For Luma Specifically

**DO NOT build a Rust XAML crate.** Here's why:

#### 1. Wrong Problem to Solve

Luma's goal is:
> "Native GUI framework for Rust with layout-based design and visual editor"

Not:
> "Enable XAML in Rust"

The value proposition is **layout system + editor**, not **WinUI 3 access**.

#### 2. Massive Opportunity Cost

9-14 months building XAML support means:
- **No** layout system development
- **No** visual editor
- **No** cross-platform foundation
- **No** actual framework for users

Meanwhile, competitors ship and gain users.

#### 3. Microsoft's Warning

When Microsoft themselves say:
> "trial and error was wasting everyone's time"

...you should listen. They have:
- Source code access (you don't)
- Direct contact with WinUI team (you don't)  
- Multiple full-time engineers (you don't)

And even THEY gave up on non-C# XAML support.

#### 4. Better Path Forward

**Phase 1-3 (Now - 3 months):**
- Complete widget set with Win32
- Layout system working
- JSON serialization
- Ship MVP with dark title bars (limitation documented)

**Phase 4 (4-6 months):**
- Custom control rendering
- Full dark mode via owner-drawn controls
- Match Windows 11 Fluent Design aesthetics
- Still using stable Win32 foundation

**Phase 5+ (Optional):**
- DirectComposition rendering layer
- GPU acceleration
- OR: WebView2 backend as alternative
- OR: Cross-platform expansion (Linux/macOS)

**Result:** Production-ready framework in 6-12 months vs. unstable XAML experiment in 9-14 months

---

## What If You REALLY Want WinUI 3?

If after all this you still want to pursue it, here's the realistic path:

### Minimum Viable XAML Support

**Scope:** Programmatic-only (no .xaml files), limited to basic controls

**Phase 1: Proof of Concept (2-3 months)**
1. Fork `windows-app` crate
2. Get basic Window + Button working
3. Implement `IXamlMetadataProvider` properly
4. Integrate `XamlControlsXamlMetaDataProvider`
5. Test on Windows 10 21H2, 22H2, Windows 11 21H2, 22H2

**Decision Point:** If crashes/quirks are unmanageable, STOP HERE.

**Phase 2: Basic Widget Set (3-4 months)**
1. Implement 10-15 core controls programmatically
2. Build builder pattern API (matching Luma's design)
3. Document all quirks/workarounds discovered
4. Test across Windows versions

**Phase 3: Layout Integration (2-3 months)**
1. Map Luma's layout system to XAML's layout containers
2. BoxLayout → StackPanel
3. GridLayout → Grid
4. Test layout recalculation

**Phase 4: Maintenance (forever)**
1. Track every WinUI 3 release
2. Update for breaking changes
3. Work around new quirks
4. Support users hitting edge cases

**Total:** 9-14 months + ongoing maintenance

### Success Criteria

Before committing, define HARD exit criteria:

- [ ] Can create Window + 3 controls without crashes (1 month checkpoint)
- [ ] No unexplained crashes after 100 hours of testing (2 month checkpoint)
- [ ] All controls tested work on Win10 + Win11 (3 month checkpoint)
- [ ] Performance acceptable (< 16ms frame time) (3 month checkpoint)

If ANY criterion fails, **abandon the effort** and return to Win32.

---

## Conclusion

**For Luma:** Stick with Win32, invest in custom rendering for Phase 4+.

**For the Rust Ecosystem:** A Rust XAML crate *could* be valuable, but:
1. Requires Microsoft's explicit support (source code access)
2. Needs dedicated maintainer(s) for years
3. Should be separate project, not blocking Luma
4. Community should fund it (significant effort)

The path to modern Windows UI in Rust is **DirectComposition**, not XAML. Microsoft told us this directly. Let's listen.

---

## Resources

- [Microsoft on XAML removal](https://github.com/microsoft/windows-rs/pull/1836)
- [WinUI 3 support discussion](https://github.com/microsoft/windows-rs/issues/2153)
- [DirectComposition documentation](https://docs.microsoft.com/en-us/windows/win32/directcomp/directcomposition-portal)
- [Rivera's comments on sustainability](https://github.com/microsoft/windows-rs/pull/1836#issuecomment-1163708941)
