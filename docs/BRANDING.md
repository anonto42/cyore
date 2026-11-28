<img src="../assets/logo/crest-primary.svg" alt="Cyore logo" width="96" align="right">

# Branding

The logo is the **crest**: a terminal window in which a tiny neural network (two inputs feeding one neuron) passes its signal to a `>_` prompt. In other words: plain language in, a command out.

## Colours
| Name | Value | Use |
|---|---|---|
| Ink | `#2B3035` | Lines, dots and the prompt on a light page |
| Paper | `#FFFFFF` | Window and node fill on a light page; lines on a dark page |
| Night | `#0D1117` | Window and node fill in the reversed logo (GitHub's dark background) |

Two colourways:
- **Primary** (ink on paper, `assets/logo/crest-primary.svg`) for light pages.
- **Reversed** (paper on night, `assets/logo/crest-reversed.svg`) for dark pages.

The window and input nodes are filled, so the logo should sit on a flat background close to its fill colour.

## Where it is used
| Place | File | Notes |
|---|---|---|
| README header | `crest-primary.svg`, `crest-reversed.svg` | A `<picture>` element switches with the reader's colour scheme |
| Docs | `crest-primary.svg` | Right-aligned at 96 px, as on this page |

## Size
Each SVG is about 1 KB, with no fonts, images or scripts.

## Changing it
Both files use the same drawing. Only the three colour classes in the `<style>` block differ (`.i` stroke, `.f` fill, `.p` paper). Edit the drawing in `crest-primary.svg`, then regenerate the reversed file:

```bash
sed -e 's/\.i{stroke:#2B3035}\.f{fill:#2B3035}\.p{fill:#FFFFFF}/.i{stroke:#FFFFFF}.f{fill:#FFFFFF}.p{fill:#0D1117}/' \
  assets/logo/crest-primary.svg > assets/logo/crest-reversed.svg
```
