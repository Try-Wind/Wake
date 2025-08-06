use std::path::Path;

pub struct SyntaxTheme {
    pub keyword: &'static str,
    pub string: &'static str,
    pub number: &'static str,
    pub comment: &'static str,
    pub function: &'static str,
    pub type_name: &'static str,
    pub reset: &'static str,
}

impl SyntaxTheme {
    pub fn dark() -> Self {
        Self {
            keyword: "\x1b[34m",     // Blue
            string: "\x1b[32m",      // Green  
            number: "\x1b[33m",      // Yellow
            comment: "\x1b[90m",     // Dim gray
            function: "\x1b[36m",    // Cyan
            type_name: "\x1b[35m",   // Magenta
            reset: "\x1b[0m",        // Reset
        }
    }
    
    pub fn light() -> Self {
        Self {
            keyword: "\x1b[94m",     // Bright blue
            string: "\x1b[92m",      // Bright green
            number: "\x1b[93m",      // Bright yellow
            comment: "\x1b[37m",     // Light gray
            function: "\x1b[96m",    // Bright cyan
            type_name: "\x1b[95m",   // Bright magenta
            reset: "\x1b[0m",        // Reset
        }
    }
}

pub fn highlight_content(content: &str, file_path: &str) -> String {
    // Determine language from file extension
    let extension = Path::new(file_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");
    
    let language_name = match extension {
        "rs" => "rust",
        "js" | "jsx" => "javascript",
        "ts" | "tsx" => "typescript",
        "py" => "python",
        "go" => "go",
        "java" => "java",
        "c" => "c",
        "cpp" | "cc" | "cxx" => "cpp",
        "h" | "hpp" => "c",
        "html" => "html",
        "css" => "css",
        "json" => "json",
        "xml" => "xml",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "md" => "markdown",
        "sh" | "bash" => "bash",
        _ => return content.to_string(), // No highlighting for unknown extensions
    };

    let theme = SyntaxTheme::dark();
    
    // Simple ANSI color highlighting for basic syntax
    let highlighted = match language_name {
        "rust" => highlight_rust(content, &theme),
        "javascript" | "typescript" => highlight_js_ts(content, &theme),
        "python" => highlight_python(content, &theme),
        "json" => highlight_json(content, &theme),
        _ => content.to_string(),
    };
    
    highlighted
}

fn highlight_rust(content: &str, theme: &SyntaxTheme) -> String {
    let mut result = String::new();
    for line in content.lines() {
        let line = line
            .replace("fn ", &format!("{}fn{} ", theme.keyword, theme.reset))
            .replace("let ", &format!("{}let{} ", theme.keyword, theme.reset))
            .replace("pub ", &format!("{}pub{} ", theme.keyword, theme.reset))
            .replace("use ", &format!("{}use{} ", theme.keyword, theme.reset))
            .replace("impl ", &format!("{}impl{} ", theme.keyword, theme.reset))
            .replace("struct ", &format!("{}struct{} ", theme.keyword, theme.reset))
            .replace("enum ", &format!("{}enum{} ", theme.keyword, theme.reset))
            .replace("match ", &format!("{}match{} ", theme.keyword, theme.reset))
            .replace("if ", &format!("{}if{} ", theme.keyword, theme.reset))
            .replace("else", &format!("{}else{}", theme.keyword, theme.reset))
            .replace("return", &format!("{}return{}", theme.keyword, theme.reset));
        result.push_str(&line);
        result.push('\n');
    }
    result
}

fn highlight_js_ts(content: &str, theme: &SyntaxTheme) -> String {
    let mut result = String::new();
    for line in content.lines() {
        let line = line
            .replace("function ", &format!("{}function{} ", theme.keyword, theme.reset))
            .replace("const ", &format!("{}const{} ", theme.keyword, theme.reset))
            .replace("let ", &format!("{}let{} ", theme.keyword, theme.reset))
            .replace("var ", &format!("{}var{} ", theme.keyword, theme.reset))
            .replace("if ", &format!("{}if{} ", theme.keyword, theme.reset))
            .replace("else", &format!("{}else{}", theme.keyword, theme.reset))
            .replace("return", &format!("{}return{}", theme.keyword, theme.reset));
        result.push_str(&line);
        result.push('\n');
    }
    result
}

fn highlight_python(content: &str, theme: &SyntaxTheme) -> String {
    let mut result = String::new();
    for line in content.lines() {
        let line = line
            .replace("def ", &format!("{}def{} ", theme.keyword, theme.reset))
            .replace("class ", &format!("{}class{} ", theme.keyword, theme.reset))
            .replace("import ", &format!("{}import{} ", theme.keyword, theme.reset))
            .replace("from ", &format!("{}from{} ", theme.keyword, theme.reset))
            .replace("if ", &format!("{}if{} ", theme.keyword, theme.reset))
            .replace("else:", &format!("{}else{}:", theme.keyword, theme.reset))
            .replace("return", &format!("{}return{}", theme.keyword, theme.reset));
        result.push_str(&line);
        result.push('\n');
    }
    result
}

fn highlight_json(content: &str, theme: &SyntaxTheme) -> String {
    let mut result = String::new();
    for line in content.lines() {
        let line = line
            .replace("\"", &format!("{}\"{}", theme.string, theme.reset))
            .replace("true", &format!("{}true{}", theme.keyword, theme.reset))
            .replace("false", &format!("{}false{}", theme.keyword, theme.reset))
            .replace("null", &format!("{}null{}", theme.keyword, theme.reset));
        result.push_str(&line);
        result.push('\n');
    }
    result
}

