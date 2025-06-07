# Blog Image Management Design

## Overview

This document proposes a simple approach for managing images within Mejiro blogs. Currently the CLI focuses on Markdown posts and static HTML generation but does not provide tooling for bundling or organizing images that accompany posts.

## Goals

- Allow users to store blog images alongside their posts.
- Ensure referenced images are copied to the output `public/` directory during `compile`.
- Provide configuration for the location of original image files.
- Offer basic CLI helpers for adding and listing images.

## Proposed Changes

### 1. Configuration

Extend `MejiroConfig` with a new field specifying where images are stored.

```yaml
styles:
  css_file: "style.css"
  icon: "icon.png"
images_dir: "images"
```

A default `images_dir` value of `"images"` will be generated when running `mejiro-cli init`.

### 2. Directory Structure

```
posts/
images/
public/
  posts/
  images/
```

All original images live under `images/`. The compile step copies them to `public/images/` preserving subfolders.

### 3. CLI Extensions

Introduce a new subcommand group `image` with minimal functionality:

- `mejiro-cli image add <path>` – copies the specified file into the configured `images_dir`.
- `mejiro-cli image list` – prints relative paths of stored images.

These helpers simplify managing local assets but do not attempt to edit Markdown references.

### 4. Compile Step

`compile.rs` will recursively copy the entire `images_dir` to `public/images/`. This ensures any `![alt](../images/foo.png)` references in Markdown work in the output.

### 5. Markdown Usage

Users reference images using standard Markdown syntax, for example:

```markdown
![Sunset](../images/sunset.jpg)
```

During compilation the relative path remains valid because `images/` is mirrored under `public/`.

## Out of Scope

- Image resizing or optimization.
- Remote image hosting.
- Automatic modification of Markdown files.

## Future Improvements

The above design keeps image support minimal. Potential enhancements include optional optimization during compile, or integration with cloud storage providers for hosting images.

