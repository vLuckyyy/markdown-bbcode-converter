use crate::markdown::markdown_converter::MarkdownResolver;

pub struct HeaderResolver;

impl HeaderResolver {
    const HASH: char = '#';
    const BB_SIZE_END: &'static str = "[/size]";

    fn process_header_line(line: &str) -> String {
        let level = line.chars().take_while(|&c| c == Self::HASH).count();
        let size = 7 - level;
        let text = line.trim_start_matches(Self::HASH).trim();
        let size_tag = format!("[size={}]", size);

        format!("{}{}{}\n", size_tag, text, Self::BB_SIZE_END)
    }
}

impl MarkdownResolver for HeaderResolver {
    fn resolve(&self, markdown: &str) -> String {
        let mut bbcode = String::new();
        for line in markdown.lines() {
            if line.starts_with(Self::HASH) {
                bbcode.push_str(&Self::process_header_line(line));
            }
        }

        bbcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_header_line() {
        let line = "### This is a test header";
        let expected = "[size=4]This is a test header[/size]\n";
        assert_eq!(HeaderResolver::process_header_line(line), expected);
    }

    #[test]
    fn test_resolve() {
        let markdown = "## First Header";
        let header_resolver = HeaderResolver;
        let bbcode = header_resolver.resolve(markdown);
        let expected = "[size=5]First Header[/size]\n";
        assert_eq!(bbcode, expected);
    }

    #[test]
    fn test_resolve_with_no_headers() {
        let markdown = "This is a line with no header\nAnd so is this one\n";
        let header_resolver = HeaderResolver;
        let bbcode = header_resolver.resolve(markdown);
        assert_eq!(bbcode, "");
    }
}