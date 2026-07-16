# Smith — Design System & Agent Guide

## 1. Product Context (CRITICAL)
Smith is NOT a simple RPA tool. It is a modular agent orchestration platform built in Rust.
The UI must reflect this architectural maturity:

- **Deterministic Layer:** `smith-windows` (UIA tools) + `smith-rpa` (type-safe nodes)
- **Orchestration Layer:** `smith-graph` (FlowGraph engine with routing/error handling)
- **Non-deterministic Layer:** `smith-ai` (Rig-based LLM agent wrapper)
- **Foundation:** `smith-core` (Tool trait, ExecutionContext, scoped variables)

**Visual Implication:** The interface serves complex DAG graphs, agent execution logs, and context inspection. The "Light Engineering Canvas" style exists to manage THIS complexity, not simple automation. Clear visual separation between deterministic (RPA) and non-deterministic (AI) paths is mandatory.

---

## 2. Theme: Light Engineering Canvas
**Visual Metaphor:** Precision drafting table with a teal technical pen.

Smith’s design system is a quiet, near-monochrome productivity canvas where a single confident teal-green does all the talking. The interface behaves like a high-end automation workspace: spacious white surfaces, hairline borders, and soft drop shadows create depth without weight. Inter handles nearly all UI text with measured, slightly tightened tracking for an engineered feel.

Color is rationed — the vivid Sage Teal appears **only** on primary actions and subtle background washes, never as decoration. Component density is comfortable but information-rich: cards feel like technical documents rather than buttons.

The distinctive choice is using **Poppins weight 600** for section display titles against an **Inter** body, creating a typographic contrast where headings shift register from operational UI to editorial voice.

---

## 3. Tokens — Colors

| Name | Value | Tailwind Token | Role |
| :--- | :--- | :--- | :--- |
| **Sage Teal** | `#008F7A` | `sage-teal` | **Primary action.** Filled CTA buttons, active nav indicators, key status badges. Sophisticated teal-green, distinct from pure green. |
| **Mist Wash** | `#00B9A3` | `mist-wash` | Soft surface accent and gradient wash for feature sections. Mid-tone teal that fades into white canvas. |
| **Deep Cyan** | `#00D4C8` | `deep-cyan` | Decorative illustration fill and gradient endpoint. **Never on UI controls.** |
| **Signal Blue** | `#3DA8FF` | `signal-blue` | Secondary semantic color for info states, links, or secondary data-viz. |
| **Obsidian** | `#000000` | `obsidian` | Primary text, heavy borders, dominant hairline dividers. |
| **Graphite** | `#141414` | `graphite` | Headline text and dark surface text. Near-black with slight warmth. |
| **Carbon** | `#313232` | `carbon` | Secondary text, image overlays, dark UI text on light backgrounds. |
| **Slate** | `#545454` | `slate` | Body text helper copy, tertiary borders. |
| **Fog** | `#707070` | `fog` | Muted body text, subtle border tone for de-emphasized content. |
| **Ash** | `#949494` | `ash` | Icon strokes, placeholder text, low-priority text. |
| **Mist** | `#d6d6d6` | `mist` | Button borders, faint dividers. Barely-there structural lines. |
| **Cloud** | `#e6e6e6` | `cloud` | Button background tint, elevated surface base. |
| **Veil** | `#f0f0f0` | `veil` | Hover states, tag backgrounds, section background tint. |
| **Paper** | `#fafafa` | `paper` | Page canvas and card surfaces. Dominant background. |

---

## 4. Tokens — Typography

### Inter Variable — Universal UI Font
Handles all navigation, body, buttons, cards, and heading text (12px–36px). Weight 500 for medium-emphasis labels, 600 for heaviest UI elements. Slightly tightened tracking (-0.02em to -0.03em).
-   **Tailwind Family:** `font-inter`
-   **Weights:** 400, 500, 600

### Poppins — Display Heading Font
Used **exclusively** for major section titles. Weight 600 at 38px creates distinct typographic register shift. Normal letter-spacing.
-   **Tailwind Family:** `font-poppins`
-   **Weights:** 600 only
-   **Sizes:** 38px only

### Type Scale (Tailwind Utilities)

| Role | Size | Line Height | Tracking | Tailwind Class |
| :--- | :--- | :--- | :--- | :--- |
| caption | 12px | 15.6 | -0.36px | `text-caption` |
| body-sm | 14px | 20 | -0.42px | `text-body-sm` |
| body | 16px | 24 | -0.32px | `text-body` |
| subheading | 18px | 27 | -0.36px | `text-subheading` |
| heading-sm | 20px | 30 | -0.4px | `text-heading-sm` |
| heading | 32px | 41.6 | -0.96px | `text-heading` |
| heading-lg | 36px | 46.8 | -1.08px | `text-heading-lg` |
| display | 38px | 47.5 | — | `text-display` |

---

## 5. Tokens — Spacing & Shapes

-   **Base Unit:** 4px
-   **Density:** Comfortable, document-like
-   **Page Max-Width:** 1200px (`max-w-page`)
-   **Section Gap:** 96px (`gap-section`)
-   **Card Padding:** 24px (`p-card`)
-   **Element Gap:** 8px (`gap-element`)

### Border Radius (STRICT)

| Element | Value | Tailwind Token |
| :--- | :--- | :--- |
| nav, tags, cards, buttons | 10px | `rounded-lg` |
| images, screenshots | 14px | `rounded-xl` |
| hero special / micro | 2px | `rounded-sm` |

> ⚠️ **NO OTHER RADII ALLOWED.** Do not use `rounded-md`, `rounded-2xl`, `rounded-full`, or arbitrary values like `rounded-[8px]`.

### Shadows

| Name | Value | Tailwind Token |
| :--- | :--- | :--- |
| sm | `rgba(0,0,0,0.04) 0px 4px 8px 0px` | `shadow-sm` |
| md | `rgba(0,0,0,0.08) 0px 4px 16px 0px` | `shadow-md` |
| inset | `rgba(0,0,0,0.05) 0px 0px 12px 0px inset` | `shadow-inset` |

---

## 6. Components

### Top Navigation Bar
Sticky header. `bg-paper`, `rounded-lg`, h-12. `font-inter text-body-sm font-medium text-graphite`. Right side: ghost "Login" → outlined "Book Demo" (`border-mist`) → **filled `bg-sage-teal text-white` "Get Started"**.

### Filled Primary Action Button
Highest-priority conversion. `bg-sage-teal text-white rounded-lg px-5 py-3 font-inter text-body-sm font-medium`. Optional icon prefix 14px. **Only filled chromatic button in the system.**

### Outlined Secondary Button
Secondary action. `bg-paper border border-mist text-graphite rounded-lg px-5 py-3 font-inter text-body-sm font-medium`. Ghost-style, visually quiet.

### Feature Card
`bg-paper rounded-lg shadow-sm p-card`. Top-left: 48px icon in `bg-veil rounded-lg`. Title: `text-body font-semibold text-graphite`. Description: `text-body-sm text-slate`. Breathable, document-like.

### Product Screenshot Frame
Large container, `rounded-xl border border-cloud bg-paper shadow-md`. Floats as technical artifact showing Smith app interface (graphs, agent logs, etc.).

### Section Display Heading
`font-poppins text-display font-semibold text-graphite text-center leading-display`. **Only place Poppins appears.**

### Compliance / Status Badge
White pill, `rounded-lg border border-cloud px-4 py-3`. Small icon + label in `text-caption font-medium text-slate`. Above section headers or in dashboards.

---

## 7. Do's and Don'ts

### ✅ Do
-   Use `sage-teal` **only** for single primary action. Never for secondary buttons, links, or decoration.
-   Set cards/buttons/nav/tags to `rounded-lg`; reserve `rounded-xl` for screenshots.
-   Apply `font-poppins text-display` **exclusively** to major section display headings.
-   Use `bg-paper` as universal page/card surface.
-   Maintain hairline `border border-cloud` or `border-mist` for structure.
-   Stack sections with `gap-section`, cards with `p-card`.
-   Use defined utility classes from this spec. No arbitrary values.

### ❌ Don't
-   **Never** use `deep-cyan` or `signal-blue` as UI colors — decorative only.
-   **Never** use teal gradient as card/section fill — subtle wash for transitions only.
-   **Never** add shadows darker than `shadow-md`.
-   **Never** use Poppins for nav, buttons, or body text.
-   **Never** introduce accent colors beyond Sage Teal palette.
-   **Never** use `rounded-none` or `rounded-full` on containers.
-   **Never** set body text below `text-body-sm` or above `text-slate`.
-   **Never** use arbitrary Tailwind values like `w-[123px]` or `text-[#abc123]`.

---

## 8. Gradient System
Single subtle gradient: `linear-gradient(114deg, rgba(0, 185, 163, 0.12), rgba(0, 143, 122, 0.04))`. Atmospheric wash for section backgrounds only. Creates barely-visible teal tint. **Never on UI components.** Pair with white cards for "documents on tinted desk" effect.

**Tailwind utility:** `bg-gradient-wash`

---

## 9. Layout
Centered `max-w-page mx-auto`. Full-bleed sections alternate between `bg-paper` and subtle teal washes. Hero: asymmetric split (text-left 40%, screenshot-right 60%). Below: centered display headings + 4-col feature grids (`grid-cols-4 gap-6`). Deeper sections: 2-col split (`grid-cols-2 gap-section`), alternating. `gap-section` vertical rhythm. Minimal fixed top nav.

---

## 10. 🤖 Agent Enforcement Rules

### Tailwind Discipline
1.  **NO ARBITRARY VALUES.** Never use `[...]` syntax. If a token doesn't exist, compose from existing tokens or ask for clarification.
2.  **USE SEMANTIC TOKENS.** Always prefer `bg-sage-teal` over `bg-[#008F7A]`, `text-graphite` over `text-[#141414]`.
3.  **STRICT RADII.** Only `rounded-sm` (2px), `rounded-lg` (10px), `rounded-xl` (14px). No exceptions.
4.  **FONT DISCIPLINE.** `font-inter` for everything except display headings. `font-poppins` ONLY for `text-display`.
5.  **COLOR RESTRICTION.** `sage-teal` = primary action only. `deep-cyan` and `signal-blue` = decorative only.
6.  **SHADOW LIMIT.** Only `shadow-sm`, `shadow-md`, `shadow-inset`. No custom shadows.
7.  **SPACING SCALE.** Use only defined spacing tokens (`spacing-4` through `spacing-96`). No arbitrary spacing.

### Quick Color Reference
-   **Text:** `text-graphite` (headings), `text-slate` (body), `text-fog` (muted)
-   **Background:** `bg-paper` (page), `bg-veil` (surface), `bg-cloud` (button bg)
-   **Border:** `border-mist` (buttons), `border-cloud` (cards)
-   **Accent:** `bg-sage-teal` (primary action), `bg-mist-wash` (surface wash)

### Example Prompts
> Create Primary Action Button: `bg-sage-teal text-white rounded-lg px-5 py-3 font-inter text-body-sm font-medium`. Only filled chromatic button.

> Build 4-col feature card grid: `grid grid-cols-4 gap-6`. Each card: `bg-paper rounded-lg shadow-sm p-card`. Top: 48px icon in `bg-veil rounded-lg`. Title: `text-body font-semibold text-graphite`. Desc: `text-body-sm text-slate`.

> Section header: centered `font-poppins text-display font-semibold text-graphite leading-display`. Below: centered `font-inter text-body text-slate`. Bg `bg-paper`, `py-section`.

> Nav bar: `bg-paper h-16 flex items-center justify-between px-6 rounded-lg`. Links: `font-inter text-body-sm font-medium text-graphite`. Right: ghost "Login", outlined "Demo" (`border border-mist rounded-lg`), filled `bg-sage-teal text-white rounded-lg` "Get Started".

---

## 11. ⚙️ Tailwind v4 Config

```css
@theme {
  /* Colors */
  --color-sage-teal: #008F7A;
  --color-mist-wash: #00B9A3;
  --color-deep-cyan: #00D4C8;
  --color-signal-blue: #3DA8FF;
  --color-obsidian: #000000;
  --color-graphite: #141414;
  --color-carbon: #313232;
  --color-slate: #545454;
  --color-fog: #707070;
  --color-ash: #949494;
  --color-mist: #d6d6d6;
  --color-cloud: #e6e6e6;
  --color-veil: #f0f0f0;
  --color-paper: #fafafa;

  /* Typography */
  --font-inter: 'Inter Variable', ui-sans-serif, system-ui, sans-serif;
  --font-poppins: 'Poppins', ui-sans-serif, system-ui, sans-serif;

  /* Typography — Scale */
  --text-caption: 12px;      --leading-caption: 15.6;   --tracking-caption: -0.36px;
  --text-body-sm: 14px;     --leading-body-sm: 20;     --tracking-body-sm: -0.42px;
  --text-body: 16px;        --leading-body: 24;        --tracking-body: -0.32px;
  --text-subheading: 18px;  --leading-subheading: 27;  --tracking-subheading: -0.36px;
  --text-heading-sm: 20px;  --leading-heading-sm: 30;  --tracking-heading-sm: -0.4px;
  --text-heading: 32px;     --leading-heading: 41.6;   --tracking-heading: -0.96px;
  --text-heading-lg: 36px;  --leading-heading-lg: 46.8;--tracking-heading-lg: -1.08px;
  --text-display: 38px;     --leading-display: 47.5;

  /* Spacing */
  --spacing-4: 4px;   --spacing-8: 8px;   --spacing-12: 12px;
  --spacing-16: 16px; --spacing-20: 20px; --spacing-24: 24px;
  --spacing-32: 32px; --spacing-40: 40px; --spacing-48: 48px;
  --spacing-60: 60px; --spacing-72: 72px; --spacing-80: 80px;
  --spacing-96: 96px;

  /* Layout */
  --max-width-page: 1200px;

  /* Border Radius — STRICT */
  --radius-sm: 2px;
  --radius-lg: 10px;
  --radius-xl: 14px;

  /* Shadows */
  --shadow-sm: rgba(0,0,0,0.04) 0px 4px 8px 0px;
  --shadow-md: rgba(0,0,0,0.08) 0px 4px 16px 0px;
  --shadow-inset: rgba(0,0,0,0.05) 0px 0px 12px 0px inset;

  /* Gradient */
  --gradient-wash: linear-gradient(114deg, rgba(0, 185, 163, 0.12), rgba(0, 143, 122, 0.04));
}
```

## 12. CSS Custom Properties (Fallback)

```css
:root {
  --color-sage-teal: #008F7A;
  --color-mist-wash: #00B9A3;
  --color-deep-cyan: #00D4C8;
  --color-signal-blue: #3DA8FF;
  --color-obsidian: #000000;
  --color-graphite: #141414;
  --color-carbon: #313232;
  --color-slate: #545454;
  --color-fog: #707070;
  --color-ash: #949494;
  --color-mist: #d6d6d6;
  --color-cloud: #e6e6e6;
  --color-veil: #f0f0f0;
  --color-paper: #fafafa;

  --font-inter: 'Inter Variable', ui-sans-serif, system-ui, sans-serif;
  --font-poppins: 'Poppins', ui-sans-serif, system-ui, sans-serif;

  --text-caption: 12px;      --leading-caption: 15.6;   --tracking-caption: -0.36px;
  --text-body-sm: 14px;     --leading-body-sm: 20;     --tracking-body-sm: -0.42px;
  --text-body: 16px;        --leading-body: 24;        --tracking-body: -0.32px;
  --text-subheading: 18px;  --leading-subheading: 27;  --tracking-subheading: -0.36px;
  --text-heading-sm: 20px;  --leading-heading-sm: 30;  --tracking-heading-sm: -0.4px;
  --text-heading: 32px;     --leading-heading: 41.6;   --tracking-heading: -0.96px;
  --text-heading-lg: 36px;  --leading-heading-lg: 46.8;--tracking-heading-lg: -1.08px;
  --text-display: 38px;     --leading-display: 47.5;

  --spacing-4: 4px;   --spacing-8: 8px;   --spacing-12: 12px;
  --spacing-16: 16px; --spacing-20: 20px; --spacing-24: 24px;
  --spacing-32: 32px; --spacing-40: 40px; --spacing-48: 48px;
  --spacing-60: 60px; --spacing-72: 72px; --spacing-80: 80px;
  --spacing-96: 96px;

  --max-width-page: 1200px;

  --radius-sm: 2px;
  --radius-lg: 10px;
  --radius-xl: 14px;

  --shadow-sm: rgba(0,0,0,0.04) 0px 4px 8px 0px;
  --shadow-md: rgba(0,0,0,0.08) 0px 4px 16px 0px;
  --shadow-inset: rgba(0,0,0,0.05) 0px 0px 12px 0px inset;

  --surface-paper: #fafafa;
  --surface-cloud: #e6e6e6;
  --surface-veil: #f0f0f0;
}