# msgvault-desktop: Frontend Design Specification

**Aesthetic Direction: Dark Foundry**
A professional, high-contrast dark interface inspired by precision instruments and industrial control rooms. Not "One Dark clone" - instead, a unique identity built around the metaphor of a vault: weight, depth, permanence. Every element earns its place.

---

## 1. Design Diagnosis (Current Problems)

### What's Wrong Now

| Problem | Where | Impact |
|---------|-------|--------|
| **Generic One Dark clone** | `colors.rs` | Looks like every Electron app; no identity |
| **System fonts only** | `main.rs`, no font loading | Feels cheap, unfinished |
| **Flat visual hierarchy** | All views | Everything has the same visual weight |
| **No visual rhythm** | `spacing.rs` | Uniform spacing = monotonous |
| **Text-only sidebar** | `sidebar.rs` | No icons, no visual anchors, hard to scan |
| **Bland stats cards** | `stats_card.rs` | Plain boxes, no visual energy |
| **No custom scrollbars** | Global | Default OS scrollbars break the aesthetic |
| **Wizard lacks presence** | `wizard.rs` | Forgettable first impression |
| **No hover/focus feedback** | Buttons, rows | Feels unresponsive |
| **Emoji icons** | `message_detail.rs` | Attachment icons use emoji (unprofessional) |
| **No density options** | Layout | One-size-fits-all spacing |
| **Same border-radius everywhere** | `components.rs` | 4px on everything = no shape language |

---

## 2. Color System Overhaul

### New Palette: "Foundry Dark"

Replace the generic One Dark palette with a unique identity. The key insight: **use warm neutrals for backgrounds and cool accents for information**. This creates depth and prevents the "blue-on-grey" sameness of most dark themes.

```rust
// === Backgrounds (Warm undertone - slight amber cast) ===
BG_DEEP:      #1a1816  // Deepest - app chrome, window frame
BG_BASE:      #211f1c  // Primary background
BG_SURFACE:   #2a2825  // Cards, panels
BG_ELEVATED:  #33302c  // Hover states, raised elements
BG_OVERLAY:   #3d3935  // Modals, dropdowns, tooltips

// === Text (Cool neutral - slight blue cast for contrast) ===
TEXT_PRIMARY:  #e8e4df  // Headings, primary content
TEXT_SECONDARY:#a8a29e  // Body text, descriptions
TEXT_MUTED:    #6b6560  // Timestamps, metadata
TEXT_DISABLED: #4a4541  // Inactive, placeholders

// === Accent: Copper (Primary brand color) ===
ACCENT_PRIMARY:   #d4956a  // Links, primary actions, brand
ACCENT_HOVER:     #e0a87e  // Hover state
ACCENT_PRESSED:   #b87d55  // Pressed state
ACCENT_SUBTLE:    #d4956a @ 15% alpha  // Selection backgrounds

// === Semantic ===
SUCCESS:   #7ab87a  // Confirmations, positive
WARNING:   #d4b86a  // Caution, attention
ERROR:     #c75c5c  // Errors, destructive
INFO:      #6a9fd4  // Informational, threads

// === Borders ===
BORDER_SUBTLE:  rgba(255,255,255, 0.06)  // Dividers
BORDER_VISIBLE: rgba(255,255,255, 0.12)  // Input outlines
BORDER_FOCUS:   #d4956a @ 60% alpha       // Focus rings

// === Selection ===
SELECTION_BG:    #d4956a @ 12% alpha  // Selected row
SELECTION_STRONG:#d4956a @ 20% alpha  // Active selection
```

### Why Copper?

- **Unique**: No major editor uses a copper/amber accent
- **Warm on warm**: Backgrounds and accent share an amber family = cohesive
- **Readable**: Copper on dark brown has excellent contrast (WCAG AA+)
- **Vault metaphor**: Copper = metal, industry, permanence
- **Not aggressive**: Unlike blue or purple, copper feels professional not techy

---

## 3. Typography System

### Font Stack

Bundle two fonts in the binary via `iced::Font::with_name()`:

```
Display/Headings:  "IBM Plex Sans" (Medium, SemiBold)
Body/UI:           "IBM Plex Sans" (Regular, Medium)
Monospace:         "IBM Plex Mono" (data, code, keyboard shortcuts)
```

**Why IBM Plex?**
- Open source (SIL license), can bundle freely
- Has character without being decorative
- Excellent readability at small sizes
- Includes mono variant for consistency
- Humanist sans-serif = warm, matches our palette

### Revised Type Scale

```rust
// Tighter, more intentional scale
SIZE_2XS: f32 = 10.0;   // Keyboard hints, fine metadata
SIZE_XS:  f32 = 11.0;   // Timestamps, badges
SIZE_SM:  f32 = 12.0;   // Secondary text, captions
SIZE_BODY:f32 = 13.0;   // Default text (was 14, slightly tighter)
SIZE_MD:  f32 = 15.0;   // Emphasized text
SIZE_LG:  f32 = 18.0;   // Section titles
SIZE_XL:  f32 = 22.0;   // Page headings
SIZE_2XL: f32 = 28.0;   // Hero text (wizard title)
SIZE_3XL: f32 = 36.0;   // Impact numbers (stats dashboard)
```

### Font Loading in `main.rs`

```rust
use iced::Font;

const PLEX_REGULAR: Font = Font::with_name("IBM Plex Sans");
const PLEX_MEDIUM: Font = Font {
    family: iced::font::Family::Name("IBM Plex Sans"),
    weight: iced::font::Weight::Medium,
    ..Font::DEFAULT
};
const PLEX_MONO: Font = Font::with_name("IBM Plex Mono");

fn main() -> iced::Result {
    iced::application(...)
        .font(include_bytes!("../assets/fonts/IBMPlexSans-Regular.ttf"))
        .font(include_bytes!("../assets/fonts/IBMPlexSans-Medium.ttf"))
        .font(include_bytes!("../assets/fonts/IBMPlexSans-SemiBold.ttf"))
        .font(include_bytes!("../assets/fonts/IBMPlexMono-Regular.ttf"))
        .theme(|_| msgvault_theme())
        // ...
}
```

---

## 4. Spacing & Layout System

### Revised Spacing Scale

```rust
// 4px base, but with more steps for precision
SPACE_1:  u16 = 2;    // Hairline gaps
SPACE_2:  u16 = 4;    // Tight inline
SPACE_3:  u16 = 6;    // Compact list items
SPACE_4:  u16 = 8;    // Default inline gap
SPACE_6:  u16 = 12;   // Form element gaps
SPACE_8:  u16 = 16;   // Section gaps
SPACE_10: u16 = 20;   // Card padding
SPACE_12: u16 = 24;   // Panel padding
SPACE_16: u16 = 32;   // Major section breaks
SPACE_20: u16 = 40;   // Page margins
SPACE_24: u16 = 48;   // Hero spacing
```

### Layout Dimensions

```rust
SIDEBAR_WIDTH: f32 = 240.0;          // Wider for icons + text
SIDEBAR_COLLAPSED_WIDTH: f32 = 56.0; // Icon-only mode (future)
MESSAGE_LIST_WIDTH: f32 = 380.0;     // Fixed, not FillPortion
DETAIL_MIN_WIDTH: f32 = 400.0;       // Minimum for readability
```

### Border Radius Language

Different radii for different element types:

```rust
RADIUS_SM: f32 = 3.0;   // Badges, chips, inline elements
RADIUS_MD: f32 = 6.0;   // Buttons, inputs, list items
RADIUS_LG: f32 = 10.0;  // Cards, panels
RADIUS_XL: f32 = 16.0;  // Modals, wizards
RADIUS_FULL: f32 = 999.0; // Avatars, circular buttons
```

---

## 5. Component Redesign

### 5.1 Sidebar

**Current**: Plain text list, no visual structure
**New**: Structured sections with icons, hover effects, and active indicator

```
┌─────────────────────┐
│  ◆ msgvault         │  <- Logo mark (copper diamond) + wordmark
│                     │
│  NAVIGATE           │  <- Section labels: uppercase, XS, muted, letter-spaced
│  ◉ Dashboard    ⌘D  │  <- Active: copper dot + copper text + bg highlight
│  ○ Search       /   │  <- Inactive: muted dot + secondary text
│  ○ Sync         Y   │
│                     │
│  BROWSE             │
│  ○ Senders          │
│  ○ Domains          │
│  ○ Labels           │
│  ○ Time             │
│                     │
│  ACCOUNTS           │
│  ● ben@gmail.com    │  <- Colored dot from avatar palette
│  ● work@corp.com    │
│                     │
│  ─────────────────  │  <- Subtle divider before bottom
│  ○ Settings     ,   │
│  ○ Help         ?   │
│                     │
│  Connected ●        │  <- Status indicator (green dot)
└─────────────────────┘
```

**Key changes**:
- Section labels: uppercase, letter-spaced, `SIZE_2XS`
- Active item: left border accent bar (2px copper), tinted background
- Shortcut hints: right-aligned, monospace font, muted
- Bottom status bar: connection state + version

### 5.2 Message List

**Current**: Basic rows with avatar + text
**New**: Denser, scannable list with visual hierarchy

```
┌────────────────────────────────────────┐
│  Senders > john@example.com            │  <- Breadcrumb header
│  127 messages              3 selected  │
│────────────────────────────────────────│
│ ┌ JS ┐ John Smith              2:45 PM │  <- Row: avatar, name (bold), time
│ │    │ Re: Q4 Budget Review         📎 │  <- Subject (secondary), attachment
│ └────┘ Draft is ready for review...    │  <- Snippet (muted, 1 line)
│────────────────────────────────────────│
│ ┌ AS ┐ Alice Sanders          Yesterday │  <- Focused row: left accent bar
│▌│    │ Project Kickoff Meeting         │  <- 2px copper bar on left edge
│ └────┘ Hi team, let's plan...          │
│────────────────────────────────────────│
│ ┌ MJ ┐ Mike Johnson           Mar 14   │
│ │    │ Invoice #4521                📎 │
│ └────┘ Please find attached...         │
└────────────────────────────────────────┘
│  Page 1 of 3    j/k navigate · Enter open │
└────────────────────────────────────────────┘
```

**Key changes**:
- Three-line rows: name+time / subject+attachments / snippet
- Focused row: 2px copper left border, slightly elevated background
- Selected row: checkbox filled with copper, subtle tint
- Snippet text: single line, truncated, muted (uses `snippet` from API)
- Denser spacing for scanning efficiency
- Remove explicit checkbox; use row background + left border for selection state

### 5.3 Message Detail

**Current**: Basic header + scrollable body
**New**: Structured header card with action bar

```
┌──────────────────────────────────────────────────────┐
│  ← Back                                    r R f  c  │  <- Action bar
│──────────────────────────────────────────────────────│
│                                                      │
│  Re: Q4 Budget Review                                │  <- Subject (XL, primary)
│                                                      │
│  ┌ JS ┐  John Smith                                  │
│  │    │  john@example.com → me, alice@...            │
│  └────┘  Tue, Feb 15, 2026 at 2:45 PM               │
│                                                      │
│  ─────────────────────────────────────────────────── │
│                                                      │
│  Hi team,                                            │
│                                                      │
│  The draft is ready for your review. I've attached    │
│  the latest version with the Q4 projections...       │
│                                                      │
│  ─────────────────────────────────────────────────── │
│                                                      │
│  ATTACHMENTS (2)                                     │
│  ┌─────────────────────────────────────────────────┐ │
│  │  📄 Q4_Budget_v3.xlsx   2.4 MB    [Download]   │ │
│  │  📄 Notes.pdf           840 KB    [Download]   │ │
│  └─────────────────────────────────────────────────┘ │
│                                                      │
│  Esc: back | ←/→: prev/next | t: thread | r: reply  │
└──────────────────────────────────────────────────────┘
```

**Key changes**:
- Top action bar with icon buttons (Reply, Reply All, Forward, Compose)
- Recipient line: "to me, alice@..." format (Gmail-style)
- Horizontal rule between header and body
- Attachment section: structured table with download buttons
- Text-based file type icons (not emoji) - use monospace glyphs

### 5.4 Dashboard

**Current**: Row of identical stat cards
**New**: Hero stats with visual weight hierarchy

```
┌──────────────────────────────────────────────────────┐
│                                                      │
│            23,847                                    │  <- Hero number (3XL, copper)
│            messages archived                         │  <- Label (SM, muted)
│                                                      │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌────────┐ │
│  │  1,247  │  │    42   │  │   186   │  │  3.2GB │ │  <- Secondary stats
│  │ threads │  │accounts │  │ labels  │  │  size  │ │
│  └─────────┘  └─────────┘  └─────────┘  └────────┘ │
│                                                      │
│  QUICK NAVIGATION                                    │
│  [Senders]  [Domains]  [Labels]  [Time]              │
│                                                      │
│  Tab: cycle views · /: search · ?: help              │
└──────────────────────────────────────────────────────┘
```

**Key changes**:
- Hero stat: total messages gets special treatment (3XL, copper color)
- Secondary stats: smaller cards, horizontal
- Browse buttons: pill-shaped with subtle border
- Hints at bottom

### 5.5 Wizard / Onboarding

**Current**: Plain card with form
**New**: Full-screen experience with atmosphere

```
┌──────────────────────────────────────────────────────┐
│                                                      │
│                                                      │
│                    ◆                                 │  <- Logo mark (large)
│                                                      │
│              msgvault                                │  <- Wordmark (2XL)
│       Your email, your archive                       │  <- Tagline (muted)
│                                                      │
│                                                      │
│       ┌─────────────────────────────────┐            │
│       │                                 │            │
│       │  Looking for your server...     │            │  <- Discovery card
│       │                                 │            │
│       │  ✓  Config file found           │            │
│       │  ✓  Server responding           │            │
│       │  ...  Validating connection     │            │
│       │                                 │            │
│       └─────────────────────────────────┘            │
│                                                      │
│                                                      │
└──────────────────────────────────────────────────────┘
```

**Key changes**:
- Centered vertically and horizontally
- Logo mark prominent
- Card narrower (400px) and more focused
- Step indicators use consistent icons: `✓` `✗` `...`
- "Found server" state: copper highlight on URL
- Subtle radial gradient on background for depth

### 5.6 Compose Modal

**Current**: Basic form overlay
**New**: Full-width bottom panel or centered modal with clear structure

```
┌──────────────────────────────────────────────────────┐
│  New Message                                   ✕     │
│──────────────────────────────────────────────────────│
│  From: ben@gmail.com                          ▼     │
│──────────────────────────────────────────────────────│
│  To:   [john@example.com ✕] [alice@co ✕] [____]    │
│  Cc:   [____]                                       │
│──────────────────────────────────────────────────────│
│  Subject: [Re: Q4 Budget Review________________]    │
│──────────────────────────────────────────────────────│
│                                                      │
│  |                                                   │  <- Body textarea
│                                                      │
│                                                      │
│──────────────────────────────────────────────────────│
│  📎 Add attachment          [Discard]  [Send ▶]     │
└──────────────────────────────────────────────────────┘
```

**Key changes**:
- Clear horizontal rule between sections
- Recipient chips: rounded pill with ✕ remove
- From dropdown: show account selector
- Footer: attachment button left, actions right
- Send button: copper accent, prominent

---

## 6. Icon System

Since Iced doesn't have built-in icon support, use one of these approaches:

### Option A: Unicode/Text Icons (Simplest)
Replace emoji with consistent text symbols:

```rust
mod icons {
    pub const DASHBOARD: &str = "◉";
    pub const SEARCH: &str = "⌕";
    pub const SYNC: &str = "↻";
    pub const SETTINGS: &str = "⚙";
    pub const COMPOSE: &str = "+";
    pub const REPLY: &str = "↩";
    pub const FORWARD: &str = "↪";
    pub const ATTACH: &str = "⊕";
    pub const DELETE: &str = "✕";
    pub const CHECK: &str = "✓";
    pub const ARROW_LEFT: &str = "←";
    pub const ARROW_RIGHT: &str = "→";
    pub const EXPAND: &str = "▸";
    pub const COLLAPSE: &str = "▾";
    pub const DOT_FILLED: &str = "●";
    pub const DOT_EMPTY: &str = "○";
    pub const DIAMOND: &str = "◆";

    // File types (monospace, consistent)
    pub const FILE_PDF: &str = "PDF";
    pub const FILE_DOC: &str = "DOC";
    pub const FILE_XLS: &str = "XLS";
    pub const FILE_IMG: &str = "IMG";
    pub const FILE_ZIP: &str = "ZIP";
    pub const FILE_GENERIC: &str = "FILE";
}
```

### Option B: Icon Font (Better Quality)
Bundle a subset of an icon font (e.g., Lucide, Phosphor) as TTF and render via `text()` with the icon font family. More work to set up but crisper results.

**Recommendation**: Start with Option A for speed, migrate to Option B later.

---

## 7. Interaction Design

### Focus & Selection States

Every interactive element needs 4 visual states:

| State | Visual Treatment |
|-------|-----------------|
| **Default** | Base background, secondary text |
| **Hover** | Elevated background, primary text |
| **Focused** | Copper left border (2px), selection background |
| **Active/Pressed** | Darker background, copper text |
| **Selected** | Subtle copper tint, checkbox filled |

### Keyboard Shortcut Display

Show shortcuts in monospace, inside subtle rounded containers:

```rust
fn keyboard_hint(shortcut: &str) -> Element<'static, Message> {
    container(
        text(shortcut)
            .font(PLEX_MONO)
            .size(SIZE_2XS)
    )
    .padding([1, 4])
    .style(|_| container::Style {
        background: Some(BG_ELEVATED),
        border: Border { radius: 3.0, width: 1.0, color: BORDER_SUBTLE },
    })
}
```

### Loading States

Replace plain "Loading..." text with a pulsing indicator:

```rust
// Animated dot pattern (updates via subscription)
fn loading_dots(frame: usize) -> &'static str {
    match frame % 4 {
        0 => "●○○",
        1 => "○●○",
        2 => "○○●",
        _ => "○●○",
    }
}
```

---

## 8. Implementation Plan

### Phase 1: Color & Typography Foundation (2-3 hours)
1. Download IBM Plex fonts, add to `assets/fonts/`
2. Load fonts in `main.rs` via `include_bytes!`
3. Replace all color constants in `colors.rs` with Foundry Dark palette
4. Update type scale in `typography.rs`
5. Add font constants to typography module
6. Update spacing scale
7. Add border radius constants
8. Update `components.rs` with new styles
9. Verify: `cargo run --release` - all views should render with new palette

### Phase 2: Sidebar Redesign (2-3 hours)
1. Add icons module (`theme/icons.rs`)
2. Rewrite `sidebar.rs`:
   - Add section labels (uppercase, letter-spaced)
   - Add left-border active indicator
   - Add shortcut hints (monospace)
   - Add connection status indicator
   - Logo mark at top
3. Update layout widths for wider sidebar

### Phase 3: Message List Polish (2-3 hours)
1. Switch to 3-line row format in `messages.rs`
2. Add left-border focus indicator
3. Improve selection visual (tinted background, not just checkbox)
4. Show snippet text (already available in `MessageSummary.snippet`)
5. Better date formatting
6. Denser default spacing

### Phase 4: Message Detail & Attachments (2-3 hours)
1. Add action bar to top of `message_detail.rs`
2. Replace emoji file icons with text icons
3. Add horizontal dividers between sections
4. Improve attachment section with download buttons (connect to Phase F work)
5. Better recipient display format

### Phase 5: Dashboard & Stats (1-2 hours)
1. Hero stat treatment for total messages
2. Smaller secondary stat cards
3. Pill-shaped browse buttons
4. Better visual hierarchy

### Phase 6: Wizard & Modals (1-2 hours)
1. Full-screen wizard with centered card
2. Logo mark above card
3. Improved step indicators
4. Better modal styling (wider border radius, stronger shadow)
5. Compose modal refinements

### Phase 7: Cross-Cutting Polish (1-2 hours)
1. Consistent hover/focus states across all components
2. Keyboard hint containers everywhere
3. Loading state improvements
4. Empty state improvements
5. Final spacing and alignment pass

---

## 9. Files to Modify

| File | Changes |
|------|---------|
| `Cargo.toml` | No changes needed (fonts are embedded) |
| `main.rs` | Add font loading, update theme |
| `theme/colors.rs` | Complete palette replacement |
| `theme/typography.rs` | New scale, font constants |
| `theme/spacing.rs` | Revised scale, radius constants |
| `theme/components.rs` | All component styles updated |
| `theme/mod.rs` | Export new `icons` module |
| **NEW** `theme/icons.rs` | Text icon constants |
| `view/sidebar.rs` | Complete rewrite |
| `view/messages.rs` | 3-line rows, focus indicator |
| `view/message_detail.rs` | Action bar, dividers, file icons |
| `view/dashboard.rs` | Hero stat, layout |
| `view/wizard.rs` | Full-screen, logo mark |
| `view/layout.rs` | Updated dimensions |
| `view/compose.rs` | Recipient chips, styling |
| `view/search.rs` | Consistent with new design |
| `view/sync.rs` | Consistent with new design |
| `view/settings.rs` | Consistent with new design |
| `view/accounts.rs` | Consistent with new design |
| `view/mod.rs` | Help modal, delete modal updates |
| `view/widgets/avatar.rs` | New color palette |
| `view/widgets/stats_card.rs` | Hero + secondary variants |
| `view/attachments.rs` | Text icons, better layout |
| `view/thread.rs` | Consistent with new design |

---

## 10. Design Principles (Reference)

1. **Warm, not cold.** Dark themes default to cold blue-grey. We use warm brown-amber to feel approachable.

2. **Hierarchy through weight, not decoration.** Larger = more important. Copper = actionable. Muted = metadata. No unnecessary borders or backgrounds.

3. **Density is a feature.** Email clients show lots of data. Tight spacing with clear hierarchy > spacious spacing with flat hierarchy.

4. **Consistent shape language.** Small radius for inline elements, medium for interactive elements, large for containers. Never mix.

5. **Keyboard-first is visible.** Show shortcut hints everywhere. The app rewards keyboard users.

6. **Copper means "do something."** The accent color is reserved for interactive elements and active states. It's never decorative.

7. **System fonts are fine if intentional.** If bundling fonts is too much overhead, at least specify `.font(Font::with_name("SF Pro"))` on macOS for native quality.
