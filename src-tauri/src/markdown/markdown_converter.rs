pub trait MarkdownResolver {
    fn resolve(&self, markdown: &str) -> String;
}

pub trait MarkdownBuilder {
    fn add_resolver(&mut self, resolver: Box<dyn MarkdownResolver>);
    fn build(self) -> MarkdownConverter;
}

pub struct MarkdownConverter {
    resolvers: Vec<Box<dyn MarkdownResolver>>,
}

impl MarkdownConverter {
    pub fn convert(&self, markdown: &str) -> String {
        let mut bbcode = String::new();
        for resolver in &self.resolvers {
            bbcode.push_str(&resolver.resolve(markdown));
        }
        bbcode
    }
}

pub struct MarkdownConverterBuilder {
    resolvers: Vec<Box<dyn MarkdownResolver>>,
}

impl MarkdownConverterBuilder {
    pub fn new() -> Self {
        Self { resolvers: Vec::new() }
    }
}

impl MarkdownBuilder for MarkdownConverterBuilder {
    fn add_resolver(&mut self, resolver: Box<dyn MarkdownResolver>) {
        self.resolvers.push(resolver);
    }

    fn build(self) -> MarkdownConverter {
        MarkdownConverter {
            resolvers: self.resolvers,
        }
    }
}