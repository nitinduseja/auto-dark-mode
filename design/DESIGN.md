---
name: Fluent Nightfall
colors:
  surface: '#131313'
  surface-dim: '#131313'
  surface-bright: '#393939'
  surface-container-lowest: '#0e0e0e'
  surface-container-low: '#1b1c1c'
  surface-container: '#202020'
  surface-container-high: '#2a2a2a'
  surface-container-highest: '#353535'
  on-surface: '#e5e2e1'
  on-surface-variant: '#bdc8cf'
  inverse-surface: '#e5e2e1'
  inverse-on-surface: '#303030'
  outline: '#889299'
  outline-variant: '#3e484e'
  surface-tint: '#74d1ff'
  primary: '#b4e4ff'
  on-primary: '#003548'
  primary-container: '#60cdff'
  on-primary-container: '#005572'
  inverse-primary: '#006687'
  secondary: '#ffdb9c'
  on-secondary: '#412d00'
  secondary-container: '#fdb700'
  on-secondary-container: '#6a4b00'
  tertiary: '#93f482'
  on-tertiary: '#003a03'
  tertiary-container: '#77d769'
  on-tertiary-container: '#005d07'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#c1e8ff'
  primary-fixed-dim: '#74d1ff'
  on-primary-fixed: '#001e2b'
  on-primary-fixed-variant: '#004d67'
  secondary-fixed: '#ffdea7'
  secondary-fixed-dim: '#ffbb18'
  on-secondary-fixed: '#271900'
  on-secondary-fixed-variant: '#5e4200'
  tertiary-fixed: '#97f987'
  tertiary-fixed-dim: '#7cdc6e'
  on-tertiary-fixed: '#002201'
  on-tertiary-fixed-variant: '#005306'
  background: '#131313'
  on-background: '#e5e2e1'
  surface-variant: '#353535'
typography:
  display:
    fontFamily: Segoe UI Variable
    fontSize: 40px
    fontWeight: '600'
    lineHeight: 52px
  headline-lg:
    fontFamily: Segoe UI Variable
    fontSize: 28px
    fontWeight: '600'
    lineHeight: 36px
  headline-md:
    fontFamily: Segoe UI Variable
    fontSize: 20px
    fontWeight: '600'
    lineHeight: 26px
  title-sm:
    fontFamily: Segoe UI Variable
    fontSize: 16px
    fontWeight: '600'
    lineHeight: 22px
  body-lg:
    fontFamily: Segoe UI Variable
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  body-md:
    fontFamily: Segoe UI Variable
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 18px
  label-md:
    fontFamily: Segoe UI Variable
    fontSize: 12px
    fontWeight: '500'
    lineHeight: 16px
  caption-sm:
    fontFamily: Segoe UI Variable
    fontSize: 11px
    fontWeight: '400'
    lineHeight: 14px
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  gutter: 1rem
  gutter-desktop: 1.5rem
  margin: 1.5rem
  margin-compact: 1rem
  space-xs: 0.25rem
  space-sm: 0.5rem
  space-md: 0.75rem
  space-lg: 1rem
  space-xl: 1.5rem
---

## Brand & Style

This design system delivers a specialized, native Windows 11 Fluent aesthetic crafted for utility software operating in dark mode. The atmosphere balances desktop precision with physical depth, mirroring the translucent layering of Mica material and Acrylic surfaces. 

The aesthetic is functional, modern, and unobtrusive. It embraces:
- **Material Depth**: Hierarchy communicated via surface illumination, subtle translucency, and light-reflecting edges rather than dramatic cast shadows.
- **Precision Alignment**: Tight 8px coordinate alignment suited for complex utility controls, hardware triggers, and timing configurations.
- **Fluent Micro-interactions**: Smooth, responsive states that provide definitive physical feedback without visual clutter.

## Colors

The palette establishes an authentic Windows 11 desktop experience rooted in dark charcoal layers, accompanied by glowing signals for active states and daylight transitions.

### Surface System
- **App Canvas (Mica Neutral)**: `#202020` creates the base canvas backdrop.
- **Sidebar Surface (Sub-base)**: `#181818` recedes behind the canvas to establish structural grouping.
- **Default Card Surface**: `#2B2B2B` forms the resting elevation for setting clusters and configuration blocks.
- **Card Surface Hover / High-Level**: `#323232` provides immediate visual confirmation of interactive bounds.
- **Stroke / Boundary**: `#3A3A3A` paired with `rgba(255, 255, 255, 0.07)` for refined, light-catching borders on surface rims.

### Accent & Semantic Roles
- **Primary Accent (`#60CDFF`)**: Windows 11 high-visibility blue for selected states, active radio nodes, slider tracks, and focused rings. 
- **Sunset Amber (`#FFB900`)**: Indicates dusk triggers, golden hour automations, and evening scheduling modes.
- **Sunrise Yellow (`#FCE100`)**: Highlights solar noon, day triggers, and dawn scheduling offsets.
- **Success / Operational Green (`#6CCB5F`)**: Displays operational status, running background daemons, and synchronized sensors.
- **Text & Foreground**:
  - Primary: `#FFFFFF` (100% opacity)
  - Secondary: `rgba(255, 255, 255, 0.786)`
  - Tertiary / Disabled: `rgba(255, 255, 255, 0.445)`

## Typography

The type scale strictly follows Windows 11 desktop conventions, utilizing variable optical axes for small caption labels versus prominent page headers.

- **Display & Headline-LG**: Reserved for utility page headers (e.g., "Settings", "Time Triggers") with tight vertical rhythm.
- **Headline-MD & Title-SM**: Used for setting group labels, sectional anchors, and modal view titles.
- **Body-LG & Body-MD**: The functional workhorses for setting item names, toggle descriptions, and detailed schedule parameters.
- **Label-MD & Caption-SM**: Assigned to status badges, micro-indicators, coordinate strings, and contextual hints beneath inputs.

## Layout & Spacing

Layout geometry follows an 8px base grid, maintaining desktop ergonomics for mouse and precision input navigation.

- **Window Anatomy**:
  - Left navigation pane: Fixed width of `280px` for top-level application modules. Collapses to an icon-only rail (`48px`) when window width drops below `768px`.
  - Main scrollable area: Adaptive column layout with standard outer padding of `space-xl` (`24px`).
- **Card Containers**: Sub-items and setting rows utilize a standard height of `64px` for expandable cards or `48px` for compact configuration fields.
- **Spacing Rhythm**:
  - Gaps between setting items within the same card: `0` (separated by hairline horizontal dividers of `1px` at `rgba(255, 255, 255, 0.05)`).
  - Vertical gap between discrete functional cards: `space-xs` (4px) to `space-sm` (8px).
  - Internal card padding: `space-lg` (16px) horizontally and `space-md` (12px) vertically.

## Elevation & Depth

Visual hierarchy is constructed via surface brightness, subtle border illumination, and delicate ambient drop shadows rather than heavy blurs.

### Surface Hierarchy
1. **Background / Sub-Surface (`#181818`)**: Receded background for sidebar navigation and system toolbars.
2. **Base Canvas (`#202020`)**: Standard application background layer simulating Mica.
3. **Resting Cards (`#2B2B2B`)**: Primary interaction layer. Surrounded by a 1px border (`#3A3A3A` blended with `rgba(255, 255, 255, 0.07)`). Casts a faint ambient shadow: `0 2px 4px rgba(0, 0, 0, 0.14)`.
4. **Hovered Cards (`#323232`)**: Slightly elevated brightness with top edge highlight (`rgba(255, 255, 255, 0.12)`).
5. **Floating Modals & Menus (`#2C2C2C`)**: Acrylic backdrop blur (`20px`), backed by border stroke `rgba(255, 255, 255, 0.12)` and elevation shadow: `0 8px 16px rgba(0, 0, 0, 0.36)`.

## Shapes

The geometric identity balances the clean precision of desktop tools with the curvature of Windows 11.

- **Setting Cards & Enclosures**: `8px` corner radius for standard grouped containers.
- **Buttons, Segmented Switches & Input Fields**: `4px` to `6px` radius for focused touch and click affordability.
- **Pill Badges & Indicator Dots**: Fully rounded (`9999px`) for status nodes and current theme chips.
- **Floating Flyouts & Menus**: `8px` corner radius with clipping to preserve backdrop filtering.

## Components

### Segmented Theme Switchers
- **Container**: Enclosed in a `#262626` track with `1px solid rgba(255, 255, 255, 0.06)` and `6px` radius. Padding is `2px`.
- **Segment Item**: Displays an icon (Sun, Moon, Auto Clock) and label. In resting state, transparent background with `rgba(255, 255, 255, 0.786)` text.
- **Active Segment**: Backed by `#383838`, elevated with `0 1px 3px rgba(0, 0, 0, 0.35)`, `1px solid rgba(255, 255, 255, 0.08)`, and pure white text.
- **Accent Underline**: A `2px` high pill in `#60CDFF` sits centered at the bottom edge of the selected segment.

### Native Toggle Switches
- **Track**: `40px` wide, `20px` tall pill. 
  - Off state: `#202020` fill, `1px solid rgba(255, 255, 255, 0.54)`.
  - On state: Fill transitions to `#60CDFF`, border switches to transparent.
- **Thumb**: `12px` circle.
  - Off state: `rgba(255, 255, 255, 0.786)`, aligned left.
  - On state: `#000000`, aligned right. Smooth `150ms ease-out` slide.

### Setting Cards & Rows
- **Card**: Rounded rectangle (`8px`) filled with `#2B2B2B`.
- **Row Content**: Left side hosts a 16px glyph, primary title (`13px`, medium weight), and optional descriptive secondary text (`12px`, muted). Right side anchors the interactive element (Toggle, Dropdown, or Link chevron).
- **Separators**: Where multiple sub-settings stack in one card, hairline borders (`rgba(255, 255, 255, 0.05)`) divide rows without breaking outer card corners.

### Buttons & Inputs
- **Primary Button**: Solid `#60CDFF` background, `#000000` text, bold weight (`600`). On hover, lightens to `#78D5FF`.
- **Standard Button**: `#2B2B2B` background, `1px solid rgba(255, 255, 255, 0.08)`, `#FFFFFF` text. On hover, shifts to `#323232`.
- **Time/Coordinate Field**: Inset field with `#1F1F1F` background, bottom-focused border accent (`2px solid #60CDFF` on focus), and mono-spaced numeral alignment for sunset/sunrise coordinates.

### Status Indicators & Chips
- **Daemon Running**: Circular `6px` node filled with `#6CCB5F`, accompanied by secondary label "Service active".
- **Sunset / Sunrise Schedule Badges**: Compact tags with subtle alpha backgrounds (`rgba(255, 185, 0, 0.15)` for dusk, `rgba(252, 225, 0, 0.15)` for dawn) and colored typography indicating offset values.