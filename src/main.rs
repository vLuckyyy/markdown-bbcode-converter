use crate::markdown::{
    CodeBlockResolver, HeaderResolver, ImageResolver, LinkResolver, ListResolver,
    markdown_converter::MarkdownConverterBuilder,
};
use crate::markdown::markdown_converter::{MarkdownBuilder, MarkdownConverter};

mod markdown;

fn main() {
    let markdown_text = r#"
    ## Enhancements

    - Add configurable default home name.
    - Add option to disable mark as afk.
    - Add Jail system.
    - Add private chat api.
    - Refactor base code of Warp and Teleport system.
    - Other minor improvements.
    "#;

    let bbcode = build_converter().convert(markdown_text);
    println!("{}", bbcode);
}

fn build_converter() -> MarkdownConverter {
    let mut builder = MarkdownConverterBuilder::new();

    builder.add_resolver(Box::new(HeaderResolver {}));
    builder.add_resolver(Box::new(CodeBlockResolver {}));
    builder.add_resolver(Box::new(ImageResolver {}));
    builder.add_resolver(Box::new(LinkResolver {}));
    builder.add_resolver(Box::new(ListResolver {}));

    builder.build()
}