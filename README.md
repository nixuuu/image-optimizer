# Image Optimizer

A fast, parallel CLI tool for optimizing images (JPEG, PNG, WebP) written in Rust.

## Features

- **Multiple formats**: Supports JPEG, PNG, and WebP optimization
- **High-quality compression**: Uses mozjpeg, oxipng with zopfli, and WebP encoders
- **Parallel processing**: Optimizes multiple images concurrently for speed
- **Flexible output**: In-place optimization or separate output directory
- **Image resizing**: Optional resizing with `--max-size` parameter
- **Backup support**: Create backup files before optimization
- **Quality control**: Adjustable quality (1-100) or lossless compression
- **Progress tracking**: Real-time progress bar with file-by-file status

## Installation

### Quick Install (Linux/macOS)

```bash
curl -sSL https://raw.githubusercontent.com/nixuuu/image-optimizer-rs/master/install.sh | bash
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
git clone https://github.com/nixuuu/image-optimizer-rs.git
cd image-optimizer-rs
cargo build --release
```

## Usage

```bash
# Basic usage - optimize images in place
image-optimizer-rs -i ./images -r

# Optimize to output directory with custom quality
image-optimizer-rs -i input_dir -o output_dir --quality 90

# Create backups and use lossless compression
image-optimizer-rs -i images --backup --lossless

# Resize images to max 1920px on longer edge
image-optimizer-rs -i photos --max-size 1920
```

### Options

- `-i, --input <PATH>` - Input directory to scan for images
- `-o, --output <PATH>` - Output directory (optional, defaults to in-place)
- `--backup` - Create backup files (.bak extension)
- `--lossless` - Use lossless compression
- `-q, --quality <1-100>` - JPEG quality (default: 85, ignored if lossless)
- `-r, --recursive` - Recursively scan subdirectories
- `--max-size <PIXELS>` - Maximum size for longer edge (resizes if larger)

## Supported Formats

- **JPEG** (.jpg, .jpeg) - Optimized with mozjpeg
- **PNG** (.png) - Optimized with oxipng and zopfli compression
- **WebP** (.webp) - Optimized with WebP encoder

## Performance

The tool uses parallel processing to optimize multiple images simultaneously, making it efficient for batch operations on large image collections.
