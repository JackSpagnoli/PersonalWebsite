# Personal Website

Written in Leptos with Tailwind CSS.

## Development

### Setup

Find install instructions in Makefile.

Set
```json
  "rust-analyzer.rustfmt.overrideCommand": ["leptosfmt", "--stdin", "--rustfmt"]
```

for leptos `view!` block formatting.

Install [Tailwind CSS Intellisense](https://marketplace.visualstudio.com/items?itemName=bradlc.vscode-tailwindcss)

and set
```json
  "emmet.includeLanguages": {
    "rust": "html",
    "*.rs": "html"
  },
  "tailwindCSS.includeLanguages": {
      "rust": "html",
      "*.rs": "html"
  },
  "files.associations": {
      "*.rs": "rust"
  },
  "editor.quickSuggestions": {
    "other": "on",
    "comments": "on",
    "strings": true
  },
  "css.validate": false,
```

### Live-Preview

Run `trunk serve --release` for live preview of the website and styling (Thanks to hook in `Trunk.toml`).
