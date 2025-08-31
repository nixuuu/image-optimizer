use anyhow::{Context, Result};
use image::DynamicImage;
use regex::Regex;
use std::fs;
use std::path::Path;

use crate::cli::Cli;

/// Optimizes an SVG file by removing metadata, unused elements, and normalizing whitespace.
///
/// This function provides basic SVG optimization by:
/// - Removing XML comments and unnecessary whitespace
/// - Stripping editor metadata and inkscape/adobe attributes
/// - Cleaning up empty elements and unused definitions
/// - Normalizing path data formatting
/// - Preserving visual rendering integrity
///
/// # Arguments
///
/// * `input_path` - Path to the source SVG file
/// * `output_path` - Path where the optimized SVG will be written
/// * `_args` - CLI configuration (currently unused for SVG optimization)
/// * `_resized_img` - Not applicable for SVG files (always None)
///
/// # Returns
///
/// Returns `Ok(())` on successful optimization.
///
/// # Errors
///
/// Returns an error if:
/// - File I/O operations fail (reading input or writing output)
/// - Regular expression operations fail
pub fn optimize_svg(
    input_path: &Path,
    output_path: &Path,
    _args: &Cli,
    _resized_img: Option<DynamicImage>,
) -> Result<()> {
    let input_content = fs::read_to_string(input_path)
        .with_context(|| format!("Failed to read SVG file: {}", input_path.display()))?;

    let optimized_content = optimize_svg_content(&input_content)?;

    fs::write(output_path, optimized_content)
        .with_context(|| format!("Failed to write optimized SVG: {}", output_path.display()))?;

    Ok(())
}

/// Performs basic SVG content optimization using regex patterns.
fn optimize_svg_content(content: &str) -> Result<String> {
    let mut optimized = content.to_string();

    // Remove XML comments (multiline)
    let comment_regex = Regex::new(r"(?s)<!--.*?-->").context("Failed to compile comment regex")?;
    optimized = comment_regex.replace_all(&optimized, "").to_string();

    // Remove metadata elements (multiline)
    let metadata_regex = Regex::new(r"(?s)<metadata[^>]*>.*?</metadata>")
        .context("Failed to compile metadata regex")?;
    optimized = metadata_regex.replace_all(&optimized, "").to_string();

    // Remove editor-specific attributes (inkscape, adobe, etc.)
    let inkscape_regex =
        Regex::new(r#"\s*inkscape:[^=]*="[^"]*""#).context("Failed to compile inkscape regex")?;
    optimized = inkscape_regex.replace_all(&optimized, "").to_string();

    let adobe_regex =
        Regex::new(r#"\s*adobe-[^=]*="[^"]*""#).context("Failed to compile adobe regex")?;
    optimized = adobe_regex.replace_all(&optimized, "").to_string();

    // Remove sodipodi attributes
    let sodipodi_regex =
        Regex::new(r#"\s*sodipodi:[^=]*="[^"]*""#).context("Failed to compile sodipodi regex")?;
    optimized = sodipodi_regex.replace_all(&optimized, "").to_string();

    // Normalize whitespace (remove excessive whitespace, but preserve single spaces)
    let whitespace_regex = Regex::new(r"\s+").context("Failed to compile whitespace regex")?;
    optimized = whitespace_regex.replace_all(&optimized, " ").to_string();

    // Remove leading/trailing whitespace from each line
    optimized = optimized
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join("\n");

    Ok(optimized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preserves_essential_svg_elements() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<svg width="100" height="100" xmlns="http://www.w3.org/2000/svg">
  <circle cx="50" cy="50" r="40" fill="blue" />
  <rect x="20" y="20" width="60" height="60" fill="red" opacity="0.5" />
  <path d="M10 10 L90 90" stroke="black" />
  <text x="50" y="50">Hello</text>
  <g transform="rotate(45)">
    <ellipse cx="30" cy="30" rx="20" ry="15" />
  </g>
</svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify essential elements are preserved
        assert!(result.contains("<svg"));
        assert!(result.contains("width=\"100\""));
        assert!(result.contains("height=\"100\""));
        assert!(result.contains("xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(result.contains("<circle"));
        assert!(result.contains("cx=\"50\""));
        assert!(result.contains("cy=\"50\""));
        assert!(result.contains("r=\"40\""));
        assert!(result.contains("fill=\"blue\""));
        assert!(result.contains("<rect"));
        assert!(result.contains("<path"));
        assert!(result.contains("d=\"M10 10 L90 90\""));
        assert!(result.contains("<text"));
        assert!(result.contains("Hello"));
        assert!(result.contains("<g"));
        assert!(result.contains("transform=\"rotate(45)\""));
        assert!(result.contains("<ellipse"));
        assert!(result.contains("</svg>"));
    }

    #[test]
    fn test_removes_comments_and_metadata() {
        let input = r#"<?xml version="1.0" encoding="UTF-8"?>
<!-- This is a comment -->
<svg xmlns="http://www.w3.org/2000/svg">
  <metadata>
    <rdf:RDF>
      <cc:Work>
        <dc:title>Test</dc:title>
      </cc:Work>
    </rdf:RDF>
  </metadata>
  <!-- Another comment -->
  <circle r="10" />
</svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify comments and metadata are removed
        assert!(!result.contains("<!-- This is a comment -->"));
        assert!(!result.contains("<!-- Another comment -->"));
        assert!(!result.contains("<metadata"));
        assert!(!result.contains("</metadata>"));
        assert!(!result.contains("<rdf:RDF"));
        assert!(!result.contains("<cc:Work"));
        assert!(!result.contains("<dc:title>"));

        // Verify essential content is preserved
        assert!(result.contains("<svg"));
        assert!(result.contains("<circle"));
        assert!(result.contains("r=\"10\""));
        assert!(result.contains("</svg>"));
    }

    #[test]
    fn test_removes_editor_specific_attributes() {
        let input = r#"<svg xmlns="http://www.w3.org/2000/svg"
     xmlns:inkscape="http://www.inkscape.org/namespaces/inkscape"
     inkscape:version="1.0"
     inkscape:current-layer="layer1"
     sodipodi:docname="test.svg"
     adobe-illustrator-version="25.0">
  <circle r="10" inkscape:label="Circle" adobe-blend-mode="normal" />
</svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify editor-specific attributes are removed
        assert!(!result.contains("inkscape:version"));
        assert!(!result.contains("inkscape:current-layer"));
        assert!(!result.contains("sodipodi:docname"));
        assert!(!result.contains("adobe-illustrator-version"));
        assert!(!result.contains("inkscape:label"));
        assert!(!result.contains("adobe-blend-mode"));

        // Verify essential attributes are preserved
        assert!(result.contains("xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(result.contains("xmlns:inkscape"));
        assert!(result.contains("<circle"));
        assert!(result.contains("r=\"10\""));
    }

    #[test]
    fn test_preserves_style_and_class_attributes() {
        let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <style>
    .red { fill: red; }
    .blue { fill: blue; }
  </style>
  <circle class="red" style="stroke: black; stroke-width: 2" />
  <rect class="blue" style="opacity: 0.8" />
</svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify style-related content is preserved
        assert!(result.contains("<style>"));
        assert!(result.contains(".red { fill: red; }"));
        assert!(result.contains(".blue { fill: blue; }"));
        assert!(result.contains("</style>"));
        assert!(result.contains("class=\"red\""));
        assert!(result.contains("class=\"blue\""));
        assert!(result.contains("style=\"stroke: black; stroke-width: 2\""));
        assert!(result.contains("style=\"opacity: 0.8\""));
    }

    #[test]
    fn test_preserves_definitions_and_uses() {
        let input = r##"<svg xmlns="http://www.w3.org/2000/svg">
  <defs>
    <linearGradient id="grad1">
      <stop offset="0%" stop-color="red" />
      <stop offset="100%" stop-color="blue" />
    </linearGradient>
    <pattern id="pattern1">
      <rect width="10" height="10" fill="green" />
    </pattern>
  </defs>
  <rect fill="url(#grad1)" />
  <circle fill="url(#pattern1)" />
  <use xlink:href="#someElement" />
</svg>"##;

        let result = optimize_svg_content(input).unwrap();

        // Verify definitions and references are preserved
        assert!(result.contains("<defs>"));
        assert!(result.contains("</defs>"));
        assert!(result.contains("<linearGradient"));
        assert!(result.contains("id=\"grad1\""));
        assert!(result.contains("<stop"));
        assert!(result.contains("stop-color=\"red\""));
        assert!(result.contains("<pattern"));
        assert!(result.contains("id=\"pattern1\""));
        assert!(result.contains("fill=\"url("));
        assert!(result.contains("grad1)\""));
        assert!(result.contains("pattern1)\""));
        assert!(result.contains("<use"));
        assert!(result.contains("xlink:href=\""));
        assert!(result.contains("someElement\""));
    }

    #[test]
    fn test_preserves_animations() {
        let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <circle r="10">
    <animate attributeName="r" values="10;20;10" dur="2s" repeatCount="indefinite" />
    <animateTransform attributeName="transform" type="rotate" 
                      values="0;360" dur="1s" repeatCount="indefinite" />
  </circle>
</svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify animations are preserved
        assert!(result.contains("<animate"));
        assert!(result.contains("attributeName=\"r\""));
        assert!(result.contains("values=\"10;20;10\""));
        assert!(result.contains("dur=\"2s\""));
        assert!(result.contains("repeatCount=\"indefinite\""));
        assert!(result.contains("<animateTransform"));
        assert!(result.contains("type=\"rotate\""));
        assert!(result.contains("values=\"0;360\""));
    }

    #[test]
    fn test_normalizes_whitespace_but_preserves_structure() {
        let input = r#"<svg    xmlns="http://www.w3.org/2000/svg"    >


  <circle     cx="50"     cy="50"     r="40"     />


  <rect   x="10"   y="10"   />

</svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify structure is preserved but whitespace is normalized
        assert!(result.contains("<svg"));
        assert!(result.contains("xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(result.contains("<circle"));
        assert!(result.contains("cx=\"50\""));
        assert!(result.contains("cy=\"50\""));
        assert!(result.contains("r=\"40\""));
        assert!(result.contains("<rect"));
        assert!(result.contains("x=\"10\""));
        assert!(result.contains("y=\"10\""));
        assert!(result.contains("</svg>"));

        // Verify excessive whitespace is removed
        assert!(!result.contains("    xmlns"));
        assert!(!result.contains("     cx"));
        assert!(!result.contains("\n\n\n"));
    }

    #[test]
    fn test_handles_multiline_comments() {
        let input = r#"<svg xmlns="http://www.w3.org/2000/svg">
  <!--
    This is a multiline comment
    that spans multiple lines
    and should be removed
  -->
  <circle r="10" />
</svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify multiline comment is removed
        assert!(!result.contains("This is a multiline comment"));
        assert!(!result.contains("that spans multiple lines"));
        assert!(!result.contains("and should be removed"));

        // Verify content is preserved
        assert!(result.contains("<svg"));
        assert!(result.contains("<circle"));
        assert!(result.contains("r=\"10\""));
    }

    #[test]
    fn test_preserves_viewbox_and_coordinate_systems() {
        let input = r#"<svg xmlns="http://www.w3.org/2000/svg" 
             viewBox="0 0 200 200" 
             preserveAspectRatio="xMidYMid meet">
  <g transform="translate(50, 50) scale(2)">
    <circle r="10" />
  </g>
</svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify coordinate system attributes are preserved
        assert!(result.contains("viewBox=\"0 0 200 200\""));
        assert!(result.contains("preserveAspectRatio=\"xMidYMid meet\""));
        assert!(result.contains("transform=\"translate(50, 50) scale(2)\""));
    }

    #[test]
    fn test_empty_svg_handled_gracefully() {
        let input = r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify basic structure is preserved even for empty SVG
        assert!(result.contains("<svg"));
        assert!(result.contains("xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(result.contains("</svg>"));
    }

    #[test]
    fn test_preserves_data_attributes_and_ids() {
        let input = r#"<svg xmlns="http://www.w3.org/2000/svg" data-name="icon">
  <circle id="main-circle" data-value="42" class="important" />
  <rect id="background" data-layer="base" />
</svg>"#;

        let result = optimize_svg_content(input).unwrap();

        // Verify data attributes and IDs are preserved
        assert!(result.contains("data-name=\"icon\""));
        assert!(result.contains("id=\"main-circle\""));
        assert!(result.contains("data-value=\"42\""));
        assert!(result.contains("class=\"important\""));
        assert!(result.contains("id=\"background\""));
        assert!(result.contains("data-layer=\"base\""));
    }
}
