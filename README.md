![Description](assets/mejiro-wide.png)

---

# mejiro-cli

**mejiro-cli** is a tiny blog management CLI tool, inspired by the **mejiro** (a small bird in Japan). This CLI helps you manage a small, personal blog by creating new posts and compiling Markdown files into static HTML.


## ✨ Features

- **Posts Management**: Quickly create a new Markdown article and View metadata for your blog posts.
- **Compile**: Compile your Markdown posts into static HTML files you can serve anywhere.
- **Image Management**: Store and copy images used in your posts.

Example: https://blog.tsugumisys.com/

## 🚀 Installation

```bash
curl -fsSL https://raw.githubusercontent.com/tsugumi-sys/mejiro/main/install/install.sh | bash
```

The installer puts `mejiro-cli` in `~/.local/bin`. Make sure that directory is on your `PATH`.


## 📦 Usage

```bash
mejiro-cli init
```

This will create:
- mejiro.yml (default config)
- posts/ (empty posts directory)
- style.css (default styling)
- icon.png (default icon)


```bash
mejiro-cli new
```

This will Creates a new article in Markdown format.

```bash
mejiro-cli compile
```

This compiles your Markdown posts into static HTML files.

## 💬 Giscus Comments

Mejiro can embed [giscus](https://github.com/giscus/giscus) comments on post pages. Generate your repository values at https://giscus.app, then add them to `mejiro.yml`:

```yaml
giscus:
  repo: "owner/repo"
  repo_id: "R_..."
  category: "Announcements"
  category_id: "DIC_..."
  mapping: "pathname"
  strict: false
  reactions_enabled: true
  emit_metadata: false
  input_position: "bottom"
  theme: "preferred_color_scheme"
  lang: "en"
  loading: "lazy"
```

If the `giscus` block is omitted, no comments section is rendered.

```bash
mejiro-cli image add ./path/to/image.png
```

Use the `image` command to add and list images stored for your blog.

```bash
mejiro-cli list
```

List post metadata stored in the `posts` directory. Use `-a` to include
unpublished drafts.

```bash
mejiro-cli upgrade
```

Download and install the latest published `mejiro-cli` release over the current binary.

## 🧩 Code Block Insertion

Embed code from external files directly into your Markdown posts:

1. Create a directory for your post, e.g. `posts/20250618-test/`.
2. Add the code files you want to reference into that directory.
3. Use `@code[path/to/file]` in your Markdown to inline the file contents.

Example:

```markdown
Here's my code:

@code[20250618-test/main.py]
```


## About

Mejiro (メジロ) means "tiny bird" in Japanese.

This CLI is designed for minimal and portable blog workflows.

## 📄 License

Licensed under MIT OR Apache-2.0.
