use crate::markdown::markdown_converter::MarkdownResolver;

pub struct LinkResolver;

impl LinkResolver {
    const LINK_START_CHAR: char = '[';
    const LINK_MID_CHAR: &'static str = "](";

    fn is_link(line: &str) -> bool {
        line.starts_with(Self::LINK_START_CHAR) && line.contains(Self::LINK_MID_CHAR)
    }

    fn convert_link_to_bbcode(line: &str) -> String {
        let parts: Vec<&str> = line.split(Self::LINK_MID_CHAR).collect();
        let text = &parts[0][1..];
        let url = &parts[1][..parts[1].len() - 1];

        format!("[url={}]{}[/url]\n", url, text)
    }
}

impl MarkdownResolver for LinkResolver {
    fn resolve(&self, markdown: &str) -> String {
        let mut bbcode = String::new();
        for line in markdown.lines() {
            if Self::is_link(line) {
                bbcode.push_str(&Self::convert_link_to_bbcode(line));
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
        let resolver = LinkResolver;
        let result = resolver.resolve("");

        assert_eq!(result, "");
    }

    #[test]
    fn test_no_link() {
        let resolver = LinkResolver;
        let result = resolver.resolve("This is a test string without any link.");

        assert_eq!(result, "");
    }

    #[test]
    fn test_single_link() {
        let resolver = LinkResolver;
        let result = resolver.resolve("[link text](http://example.com)");

        assert_eq!(result, "[url=http://example.com]link text[/url]\n");
    }

    #[test]
    fn test_multiple_links() {
        let resolver = LinkResolver;
        let result = resolver.resolve("[link1 text](http://example1.com)\n[link2 text](http://example2.com)\n");

        assert_eq!(result, "[url=http://example1.com]link1 text[/url]\n[url=http://example2.com]link2 text[/url]\n");
    }
}