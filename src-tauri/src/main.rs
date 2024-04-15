// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::markdown::{
    CodeBlockResolver, HeaderResolver, ImageResolver, LinkResolver, ListResolver,
    markdown_converter::MarkdownConverterBuilder,
};
use crate::markdown::markdown_converter::{MarkdownBuilder, MarkdownConverter};

mod markdown;

pub fn build_converter() -> MarkdownConverter {
    let mut builder = MarkdownConverterBuilder::new();

    builder.add_resolver(Box::new(HeaderResolver {}));
    builder.add_resolver(Box::new(CodeBlockResolver {}));
    builder.add_resolver(Box::new(ImageResolver {}));
    builder.add_resolver(Box::new(LinkResolver {}));
    builder.add_resolver(Box::new(ListResolver {}));

    builder.build()
}

#[tauri::command]
fn convert_to_bbcode(markdown_text: String) -> String {
    let converter = build_converter();
    converter.convert(&markdown_text)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert_to_bbcode])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}