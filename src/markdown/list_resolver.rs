use crate::markdown::markdown_converter::MarkdownResolver;

pub struct ListResolver;

impl ListResolver {
    const LIST_START: &'static str = "[LIST]\n";
    const LIST_END: &'static str = "[/LIST]\n";
    const ITEM_PREFIX: &'static str = "[*]";
    const MARKDOWN_LIST_PREFIX: &'static str = "- ";

    fn append_list_item(line: &str, bbcode: &mut String) {
        bbcode.push_str(Self::ITEM_PREFIX);
        bbcode.push_str(&line[2..]);
        bbcode.push_str("\n");
    }

    fn end_list(in_list: &mut bool, bbcode: &mut String) {
        if !*in_list { return; }
        bbcode.push_str(Self::LIST_END);
        *in_list = false;
    }
}

impl MarkdownResolver for ListResolver {
    fn resolve(&self, markdown: &str) -> String {
        let mut bbcode = String::new();
        let mut in_list = false;

        for line in markdown.lines() {
            if line.starts_with(Self::MARKDOWN_LIST_PREFIX) {
                if !in_list {
                    bbcode.push_str(Self::LIST_START);
                    in_list = true;
                }

                Self::append_list_item(line, &mut bbcode);
                continue;
            }
            Self::end_list(&mut in_list, &mut bbcode);
        }

        Self::end_list(&mut in_list, &mut bbcode);

        bbcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let resolver = ListResolver;
        let result = resolver.resolve("");

        assert_eq!(result, "");
    }

    #[test]
    fn test_no_list() {
        let resolver = ListResolver;
        let result = resolver.resolve("This is a string without a list.");

        assert_eq!(result, "");
    }

    #[test]
    fn test_single_list() {
        let resolver = ListResolver;
        let result = resolver.resolve("- item 1\n- item 2\n");

        assert_eq!(result, "[LIST]\n[*]item 1\n[*]item 2\n[/LIST]\n");
    }

    #[test]
    fn test_multiple_lists() {
        let resolver = ListResolver;
        let result = resolver.resolve("- item 1\n- item 2\n\n- item 3\n- item 4\n");

        assert_eq!(result, "[LIST]\n[*]item 1\n[*]item 2\n[/LIST]\n[LIST]\n[*]item 3\n[*]item 4\n[/LIST]\n");
    }
}