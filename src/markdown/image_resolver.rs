use crate::markdown::markdown_converter::MarkdownResolver;

pub struct ImageResolver;

impl ImageResolver {
    const MD_IMAGE_START_CHARS: &'static str = "![";
    const MD_IMAGE_MIDDLE_CHARS: &'static str = "](";

    fn is_image(line: &str) -> bool {
        line.starts_with(Self::MD_IMAGE_START_CHARS) && line.contains(Self::MD_IMAGE_MIDDLE_CHARS)
    }

    fn convert_image_to_bbcode(line: &str) -> String {
        let parts: Vec<&str> = line.split(Self::MD_IMAGE_MIDDLE_CHARS).collect();
        let url = &parts[1][..parts[1].len() - 1];

        format!("[img]{}[/img]\n", url)
    }
}

impl MarkdownResolver for ImageResolver {
    fn resolve(&self, markdown: &str) -> String {
        let mut bbcode = String::new();
        for line in markdown.lines() {
            if Self::is_image(line) {
                bbcode.push_str(&Self::convert_image_to_bbcode(line));
            }
        }

        bbcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let resolver = ImageResolver;
        let result = resolver.resolve("");

        assert_eq!(result, "");
    }

    #[test]
    fn test_no_img() {
        let resolver = ImageResolver;
        let result = resolver.resolve("This is a string without any image.");

        assert_eq!(result, "");
    }

    #[test]
    fn test_single_img() {
        let resolver = ImageResolver;
        let result = resolver.resolve("![alt text](http://example.com/image.jpg)");

        assert_eq!(result, "[img]http://example.com/image.jpg[/img]\n");
    }

    #[test]
    fn test_multiple_img() {
        let resolver = ImageResolver;
        let result = resolver.resolve("![alt text1](http://example1.com/image1.jpg)\n![alt text2](http://example2.com/image2.jpg)");

        assert_eq!(result, "[img]http://example1.com/image1.jpg[/img]\n[img]http://example2.com/image2.jpg[/img]\n");
    }
}