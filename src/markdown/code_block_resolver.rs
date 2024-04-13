use crate::markdown::markdown_converter::MarkdownResolver;

pub struct CodeBlockResolver;

impl CodeBlockResolver {
    const CODE_BLOCK_BOUNDARY: &'static str = "```";
    const BB_START_CODE: &'static str = "[code]\n";
    const BB_END_CODE: &'static str = "[/code]\n";

    fn process_code_block_line(line: &str, in_code_block: &mut bool) -> String {
        if line.starts_with(Self::CODE_BLOCK_BOUNDARY) {
            *in_code_block = !*in_code_block;
            return if *in_code_block { Self::BB_START_CODE.to_string() } else { Self::BB_END_CODE.to_string() };
        }

        if *in_code_block {
            return format!("{}\n", line);
        }

        String::new()
    }
}

impl MarkdownResolver for CodeBlockResolver {
    fn resolve(&self, markdown: &str) -> String {
        let mut bbcode = String::new();
        let mut in_code_block = false;

        for line in markdown.lines() {
            bbcode.push_str(&Self::process_code_block_line(line, &mut in_code_block));
        }

        bbcode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        let resolver = CodeBlockResolver;
        let result = resolver.resolve("");

        assert_eq!(result, "");
    }

    #[test]
    fn test_no_code_block() {
        let resolver = CodeBlockResolver;
        let result = resolver.resolve("This is a string without a code block.");

        assert_eq!(result, "");
    }

    #[test]
    fn test_single_code_block() {
        let resolver = CodeBlockResolver;
        let result = resolver.resolve("```\nlet x = 10;\n```");

        assert_eq!(result, "[code]\nlet x = 10;\n[/code]\n");
    }

    #[test]
    fn test_multiple_code_blocks() {
        let resolver = CodeBlockResolver;
        let result = resolver.resolve("```\nlet x = 10;\n```\n```\nlet y = 20;\n```");

        assert_eq!(result, "[code]\nlet x = 10;\n[/code]\n[code]\nlet y = 20;\n[/code]\n");
    }
}