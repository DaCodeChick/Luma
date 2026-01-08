# Phase 4: WinUI 3 Proof-of-Concept - Implementation Plan

**Status**: 🚧 IN PROGRESS  
**Start Date**: January 8, 2026  
**Critical**: This is a GO/NO-GO decision point for WinUI 3 backend

## Overview

Phase 4 aims to create a minimal but functional WinUI 3 backend with:
- Basic window creation
- 3 widgets: Button, TextBlock, StackPanel
- Event handling (button clicks)
- Stability testing over 100+ hours

**Success Criteria**: ALL must pass or we abandon WinUI 3
- ✅ Window creates without crash
- ✅ Widgets render correctly
- ✅ Events (button click) work
- ⏳ No unexplained crashes in 100 hours (requires user testing)
- ⏳ Works on all test Windows versions (requires user testing)
- ⏳ Performance acceptable (<16ms frame time) (requires user testing)

## Current State

The `luma-winui` crate exists with skeleton structure:
- ✅ Error types defined
- ✅ Runtime module (stubbed)
- ✅ Window module (stubbed)
- ✅ Widgets module (empty)
- ✅ Dependencies configured (windows-rs 0.52)

**What's Missing**: All actual WinUI 3 implementation

## Implementation Strategy

### Approach: Programmatic-First

We will **NOT** use XAML in Phase 4. Instead, we'll create WinUI controls programmatically via windows-rs bindings.

**Rationale**:
1. Simpler - no XAML parsing complexity
2. Faster to test stability
3. XAML loading comes in Phase 5 (after stability is proven)
4. Matches the phase requirements exactly

### Technology Stack

- **windows-rs 0.52**: Rust bindings to Windows APIs
- **Windows App SDK**: WinUI 3 runtime (must be installed on system)
- **COM interop**: Required for WinUI 3
- **Luma backend traits**: From `luma-core`

## Implementation Tasks

### Task 1: Windows App SDK Integration ✅

**Goal**: Get WinUI 3 runtime initialized

**Files to modify**:
- `Cargo.toml` - Add WinUI 3 Windows features
- `src/runtime.rs` - Implement runtime initialization

**Windows APIs needed**:
```rust
// Windows App SDK bootstrap
use windows::Win32::System::WinRT::*;
use windows::ApplicationModel::*;

// WinUI 3 namespaces
use windows::UI::Xaml::*;
use windows::UI::Xaml::Controls::*;
use windows::UI::Xaml::Hosting::*;
```

**Steps**:
1. Research Windows App SDK bootstrap APIs
2. Implement `WinUIRuntime::initialize()`
3. Handle COM initialization
4. Test runtime can start/stop without crash

### Task 2: Basic Window Creation ✅

**Goal**: Create a programmatic WinUI Window that shows on screen

**Files to modify**:
- `src/window.rs` - Implement WinUIWindow
- Implement `WindowBackend` trait

**WinUI APIs**:
```rust
// Window class (WinUI 3)
use windows::UI::Xaml::Window;
use windows::UI::Xaml::DesktopWindowXamlSource; // For desktop app hosting
```

**Steps**:
1. Research how to host WinUI 3 in a desktop app
2. Create HWND for window chrome
3. Create DesktopWindowXamlSource to host XAML islands
4. Set window title, size
5. Show window
6. Handle window close events

**Testing**:
- Window appears on screen
- Window can be moved, resized, closed
- No crashes on create/show/close

### Task 3: WinUIButton Widget ✅

**Goal**: Create a clickable button that fires events

**Files to create**:
- `src/widgets/button.rs`
- `src/widgets/mod.rs` (update exports)

**WinUI APIs**:
```rust
use windows::UI::Xaml::Controls::Button;
use windows::Foundation::TypedEventHandler;
```

**Steps**:
1. Create Button control programmatically
2. Set Content property (button text)
3. Add Click event handler
4. Implement ButtonBackend trait
5. Test button renders and clicks work

**Testing**:
- Button appears in window
- Button text is correct
- Click events fire callback
- No crashes on rapid clicking

### Task 4: WinUITextBlock Widget ✅

**Goal**: Display text in a label

**Files to create**:
- `src/widgets/text_block.rs`

**WinUI APIs**:
```rust
use windows::UI::Xaml::Controls::TextBlock;
```

**Steps**:
1. Create TextBlock programmatically
2. Set Text property
3. Implement LabelBackend trait
4. Test text displays correctly

**Testing**:
- Text renders correctly
- Text updates when changed
- No crashes

### Task 5: WinUIStackPanel Layout ✅

**Goal**: Stack widgets vertically or horizontally

**Files to create**:
- `src/widgets/stack_panel.rs`

**WinUI APIs**:
```rust
use windows::UI::Xaml::Controls::StackPanel;
use windows::UI::Xaml::Controls::Orientation;
```

**Steps**:
1. Create StackPanel programmatically
2. Set Orientation property
3. Add Children collection support
4. Implement PanelBackend trait
5. Test layout of multiple widgets

**Testing**:
- Multiple widgets stack correctly
- Orientation changes work
- No layout glitches

### Task 6: Proof-of-Concept Example ✅

**Goal**: Working example app demonstrating all features

**Files to create**:
- `examples/poc_window.rs`

**Example app should**:
- Create window
- Add StackPanel
- Add TextBlock with "Hello WinUI"
- Add Button with "Click Me"
- Button click updates TextBlock text
- Button click count displayed

**Testing**:
- Run example, verify all works
- No crashes over 10 minute run
- Document any quirks/issues

### Task 7: Backend Trait Implementation ✅

**Goal**: Implement luma-core backend traits

**Traits to implement**:
- `WindowBackend` for `WinUIWindow`
- `ButtonBackend` for `WinUIButton`
- `LabelBackend` for `WinUITextBlock`
- `PanelBackend` for `WinUIStackPanel`

**Testing**:
- Traits compile
- Can swap Win32 backend with WinUI backend
- Same API, different implementation

### Task 8: Documentation ✅

**Goal**: Document POC results for go/no-go decision

**Documents to create**:
- `WINUI_POC_RESULTS.md` - Detailed findings
- Update `XAML_ARCHITECTURE.md` with Phase 4 status

**Document should include**:
- What works
- What doesn't work
- Crashes encountered
- Performance measurements
- Stability assessment
- Recommendation: GO or NO-GO

## Technical Challenges

### Challenge 1: XAML Islands vs Pure WinUI

**Issue**: Desktop apps use "XAML Islands" to host WinUI 3 content.

**Options**:
1. Use DesktopWindowXamlSource (XAML Islands approach)
2. Use pure packaged app model (requires MSIX packaging)

**Decision**: Start with XAML Islands for flexibility. Document any issues.

### Challenge 2: COM Threading Model

**Issue**: WinUI 3 requires STA (Single-Threaded Apartment) COM threading.

**Solution**: 
- Initialize COM as STA in runtime
- Ensure all WinUI calls on same thread
- Use message loop properly

### Challenge 3: Event Handling

**Issue**: WinUI events use TypedEventHandler which requires closures.

**Solution**:
- Box closures for event handlers
- Careful lifetime management
- Test memory leaks

### Challenge 4: windows-rs API Gaps

**Issue**: windows-rs may not have all WinUI 3 APIs yet.

**Mitigation**:
- Use windows-bindgen if needed
- Document missing APIs
- Consider contributing to windows-rs

## Testing Plan

### Phase 4 Testing (During Development)

1. **Unit Tests**: Basic functionality
2. **Integration Tests**: Window + widgets together
3. **Example Apps**: Visual verification
4. **Manual Testing**: User interaction

### Phase 4 Stability Testing (After Implementation)

**100+ Hour Test Plan** (User responsibility):

1. **Automated Stress Test** (72 hours)
   - Create/destroy windows in loop
   - Click buttons rapidly
   - Update text frequently
   - Monitor for crashes, leaks

2. **Manual Testing** (28+ hours)
   - Use example apps daily
   - Different Windows versions
   - Different screen configurations
   - Document all issues

3. **Performance Testing**
   - Frame time measurements
   - CPU/memory usage
   - Startup time
   - Shutdown time

**Go Criteria**: Zero unexplained crashes, <16ms frame time

**No-Go Criteria**: Random crashes, memory leaks, >16ms frame time

## Dependencies

### Required Software
- Windows 10 version 1809+ or Windows 11
- Windows App SDK runtime installed
- Visual C++ Redistributable
- Rust 1.75+

### Cargo Dependencies
```toml
windows = { version = "0.52", features = [
    "Win32_Foundation",
    "Win32_System_WinRT",
    "Win32_System_Com",
    "Win32_UI_WindowsAndMessaging",
    "Foundation",
    "ApplicationModel",
    "UI_Xaml",
    "UI_Xaml_Controls",
    "UI_Xaml_Hosting",
    "UI_Xaml_Markup",
] }
```

## Risk Assessment

### High Risk
- ❌ **WinUI 3 stability**: This is the critical unknown
- ❌ **Undocumented behavior**: May hit edge cases
- ❌ **Windows version differences**: 10 vs 11 may differ

### Medium Risk
- ⚠️ **COM interop complexity**: Rust + COM can be tricky
- ⚠️ **Event handler lifetimes**: Memory management
- ⚠️ **Performance**: Unknown if <16ms achievable

### Low Risk
- ✅ **Basic window creation**: Well-documented
- ✅ **Simple widgets**: Button, TextBlock are stable APIs
- ✅ **Layout**: StackPanel is straightforward

## Timeline

**Development** (This session): ~4-6 hours
- Task 1: Runtime - 30 min
- Task 2: Window - 1 hour
- Task 3: Button - 1 hour
- Task 4: TextBlock - 30 min
- Task 5: StackPanel - 1 hour
- Task 6: Example - 30 min
- Task 7: Traits - 1 hour
- Task 8: Documentation - 30 min

**Stability Testing** (User): 100+ hours over weeks/months
- Can be done in background
- Automated tests run unattended
- Manual tests during normal usage

## Success Metrics

### Phase 4 POC Success
- ✅ Code compiles without warnings
- ✅ Window creates and shows
- ✅ All 3 widgets render
- ✅ Button click works
- ✅ Example app runs
- ⏳ No crashes in 100 hours (user testing)
- ⏳ <16ms frame time (user testing)

### Phase 4 POC Failure (Any of these)
- ❌ Random crashes
- ❌ Memory leaks
- ❌ Consistent >16ms frame time
- ❌ Rendering glitches
- ❌ Events don't fire reliably

## Next Steps After Phase 4

### If GO (Success)
- Proceed to Phase 5: XAML Loading
- Implement XamlLoader to parse XAML → WinUI objects
- Expand widget library
- Continue stability monitoring

### If NO-GO (Failure)
- Document all findings in detail
- Archive luma-winui crate
- Focus 100% on Win32 backend (luma-windows)
- Consider contributing findings to community

## Notes

- This is a **passion project** - quality over speed
- Take time to test thoroughly
- Document everything
- Be honest about stability issues
- The Win32 fallback is always available

---

**Ready to Begin**: ✅ Yes
**Next Step**: Implement Task 1 (Runtime initialization)
