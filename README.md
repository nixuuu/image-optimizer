# Image Optimizer

A fast, parallel CLI tool for optimizing images (JPEG, PNG, WebP, SVG) written in Rust.

## Features

- **Multiple formats**: Supports JPEG, PNG, WebP, and SVG optimization
- **High-quality compression**: Uses mozjpeg, oxipng with zopfli, WebP encoders, and regex-based SVG optimization
- **Parallel processing**: Optimizes multiple images concurrently for speed
- **Flexible output**: In-place optimization or separate output directory
- **Image resizing**: Optional resizing with `--max-size` parameter (applies to raster formats only)
- **Backup support**: Create backup files before optimization
- **Quality control**: Adjustable quality (1-100) or lossless compression (applies to raster formats only)
- **Progress tracking**: Real-time progress bar with file-by-file status

## Installation

### From crates.io (Recommended)

```bash
cargo install image-optimizer
```

### Quick Install (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/nixuuu/image-optimizer/refs/heads/master/install.sh | bash
```

This script will:
- Detect your platform and architecture
- Download prebuilt binaries (if available) or build from source
- Install to `~/.local/bin` and configure your PATH

### Manual Installation

#### System Dependencies

This tool requires system libraries for JPEG optimization:

**Ubuntu/Debian:**
```bash
sudo apt-get install libjpeg-dev
```

**macOS:**
```bash
brew install mozjpeg
```

**Fedora/RHEL:**
```bash
sudo dnf install libjpeg-turbo-devel
```

#### Build from Source

```bash
git clone https://github.com/nixuuu/image-optimizer.git
cd image-optimizer
cargo build --release
```

## Usage

```bash
# Basic usage - optimize images in place
image-optimizer -i ./images -r

# Optimize to output directory with custom quality
image-optimizer -i input_dir -o output_dir --quality 90

# Create backups and use lossless compression
image-optimizer -i images --backup --lossless

# Optimize SVG files (removes metadata, comments, editor attributes)
image-optimizer -i icons/ -r

# Resize raster images to max 1920px on longer edge
image-optimizer -i photos --max-size 1920

# Update to the latest version
image-optimizer --update
```

### Options

- `-i, --input <PATH>` - Input directory to scan for images
- `-o, --output <PATH>` - Output directory (optional, defaults to in-place)
- `--backup` - Create backup files (.bak extension)
- `--lossless` - Use lossless compression
- `-q, --quality <1-100>` - JPEG quality (default: 85, ignored if lossless, applies to raster formats only)
- `-r, --recursive` - Recursively scan subdirectories
- `--max-size <PIXELS>` - Maximum size for longer edge (resizes if larger, applies to raster formats only)
- `--update` - Update to the latest version from GitHub releases

## Supported Formats

### Raster Images
- **JPEG** (.jpg, .jpeg) - Optimized with mozjpeg for superior compression
- **PNG** (.png) - Optimized with oxipng and zopfli compression algorithms
- **WebP** (.webp) - Optimized with Google's WebP encoder

### Vector Graphics  
- **SVG** (.svg) - Optimized with regex-based processing that safely removes:
  - XML comments and unnecessary whitespace
  - Editor metadata (Inkscape, Adobe Illustrator, Sodipodi attributes)
  - RDF metadata blocks and temporary editing data
  - While preserving all visual elements, animations, styles, and interactive features

## Performance

The tool uses parallel processing to optimize multiple images simultaneously, making it efficient for batch operations on large image collections.
