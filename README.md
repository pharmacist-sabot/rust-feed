# Rust Feed

```
██████╗ ██╗   ██╗ ██████╗████████╗███████╗███████╗███████╗██████╗
██╔══██╗██║   ██║██╔════╝╚══██╔══╝██╔════╝██╔════╝██╔════╝██╔══██╗
██████╔╝██║   ██║███████╗   ██║   █████╗  █████╗  █████╗  ██║  ██║
██╔══██╗██║   ██║╚════██║   ██║   ██╔══╝  ██╔══╝  ██╔══╝  ██║  ██║
██║  ██║╚██████╔╝██████╔╝   ██║   ██║     ███████╗███████╗██████╔╝
╚═╝  ╚═╝ ╚═════╝ ╚═════╝   ╚═╝╚═╝╚══════╝╚══════╝╚═════╝
```

---

## ◆ PULSE

The Rust community writes in a hundred places; the Facebook Rust Dev
Community writes in one that vanishes with every scroll. Rust Feed is
the rescue: a centralized, open-source collection of those articles
and tutorials - case studies, deep dives, tool reviews, release
notes, crate guides - organized into a knowledge base built with
mdBook, served at [rust-feed.github.io](https://rust-feed.github.io).
What the community shared is no longer lost in the feed; it is a
book.

| Curated ▸ sealed | Indexed ▸ sealed | mdBook ▸ sealed | Open ▸ sealed |
|---|---|---|---|

*The feed-to-book pipeline - collect, categorize, index - is sealed.*

> Built with mdBook and a custom Rust auto-indexer - the community's
> writing, organized by machines so it reads like a book.
>
> **suradet-ps**, artifact keeper

---

## ◆ IGNITION

One install, one script.

```
⟫ cargo install mdbook
⟫ git clone https://github.com/rust-feed/rust-feed.github.io.git
⟫ cd rust-feed.github.io
⟫ ./update.sh
```

The book is served at [rust-feed.github.io](https://rust-feed.github.io);
locally, `⟫ mdbook serve --open` previews it on your machine.

<details>
<summary>Prerequisites</summary>

- [Rust and Cargo](https://rustup.rs/) - to install mdBook and run
  the auto-indexer
- [mdBook](https://rust-lang.github.io/mdBook/) - installed above

</details>

---

## ◆ ANATOMY

One script, one indexer, a book that files itself.

- **Collects** - articles live as markdown under `src/` in category
  folders - `case-study/`, `deep-dive/`, `rust-tools/`,
  `rust-update/`, `rust-crates/`, `rust-project/` - each file with
  its `# Heading 1` as the title.
- **Indexes** - the custom Rust CLI in `scripts/auto_index/` scans
  the folders, regenerates each category index, and rewrites
  `SUMMARY.md` - the book's structure maintains itself, no manual
  sidebar surgery.
- **Builds** - `update.sh` runs the indexer then the mdBook build;
  the compiled HTML lands in `book/` (gitignored), and CI ships it
  to GitHub Pages.
- **Lints** - `markdownlint-cli2` enforces the markdown contract in
  CI, and `lychee` checks every link - a broken link or a malformed
  heading fails the build before the reader meets it.
- **Tests** - the auto-indexer carries its own test suite under
  `scripts/auto_index/` - the same suite CI runs.

---

## ◆ RITUALS

**The core ceremony** - adding an article:

1. Write the article as markdown with a `# Heading 1` title.
2. Place it in the category it belongs to:
   `src/case-study/my-new-article.md`.
3. Run `./update.sh` - the indexer files it, updates the category
   index, and regenerates `SUMMARY.md`.
4. Preview with `mdbook serve --open`, then push - CI lints, checks
   links, runs the indexer tests, and ships the book.

**The ceremony of the auto-index** - no one edits `SUMMARY.md` by
hand: the indexer reads the folders and writes the structure. The
book stays consistent because the machine files the additions, and
the tests stay green because the machine verifies itself.

**The ceremony of the shared community** - every article carries the
credit of its original post; the collection exists to give the
community's writing a permanent shelf. What was ephemeral in the
feed becomes reference material.

---

## ◆ ECHOES

**Where this artifact is heading**

```
collect ▸ category-organized article folders ───────────────────────── ▸ sealed
index   ▸ Rust auto-indexer regenerates SUMMARY.md ─────────────────── ▸ sealed
build   ▸ mdBook + update.sh pipeline ──────────────────────────────── ▸ sealed
guard   ▸ markdownlint + lychee + indexer tests in CI ──────────────── ▸ sealed
```

**Raising the artifact** - the contribution rules live in
`CONTRIBUTING.md`; the security posture in `SECURITY.md`; the spelling
allow-list in `.cspell.json`. The auto-indexer tests run with
`cargo test --manifest-path scripts/auto_index/Cargo.toml`. Open an
issue first to discuss a change.

**Status** - CI lints, link-checks, tests, and deploys every push.
[Watch the gates](.github/workflows).

---

```
  ─────────────────────────────────────────
   A community's feed scrolls away.
   A community's book stays.
  ─────────────────────────────────────────
```

Licensed under the [MIT License](LICENSE).