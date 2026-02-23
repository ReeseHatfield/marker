use regex::Regex;

#[derive(Debug)]
struct DocComment {
    description: String,
    params: Vec<Param>,
    return_type: Option<Return>,
}

#[derive(Debug)]
struct Return {
    data_type: String,
    description: String,
}

#[derive(Debug)]
struct Param {
    name: String,
    data_type: Vec<String>,
    default: Option<String>,
    description: String,
}

fn parse_document(input: &str) -> Vec<DocComment> {
    let mut comment_chunks = Vec::new();
    let mut cur = String::new();

    for line in input.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("///") {
            // strip the /// for parsing
            // every doc comment MUST have a /// anyway
            cur.push_str(trimmed.trim_start_matches("///").trim());
            cur.push('\n');

        // should handle breaks in between the doc comments
        } else if !cur.is_empty() {
            comment_chunks.push(cur.clone());
            cur.clear();
        }
    }

    comment_chunks
        .into_iter()
        .map(|block| parse_block(&block))
        .collect()
}

fn parse_block(block: &str) -> DocComment {
    let mut lines = block.lines();

    // asserting that description is everything before the first @ tag
    let description = lines
        .clone()
        .take_while(|l| !l.starts_with('@'))
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string();

    let mut params = Vec::new();
    let mut return_type = None;

    let param_re = Regex::new(
        r"(?x)
        @param\s+
        (?P<name>\w+)\s+
        (?P<type>\[[^\]]+\]|\S+)\s*   # matches [a | b] or single word
        (?:=\s*(?P<default>\S+))?\s*  # optional default after '='
        (?P<desc>.*)                  # everything else is description
    ",
    )
    .unwrap();

    let return_re = Regex::new(r"@return\s+(?P<type>\S+)\s+(?P<desc>.*)").unwrap();

    for line in lines {


        if let Some(caps) = param_re.captures(line) {

            // strip the [ ]s from the types
            // but leave the |s
            let type_raw = caps["type"].trim_matches(|c| c == '[' || c == ']');

            // split on |s map to string and collect
            let data_types = type_raw.split('|').map(|s| s.trim().to_string()).collect();

            let p =  Param {
                name: caps["name"].to_string(),
                data_type: data_types,
                default: caps.name("default").map(|m| m.as_str().to_string()),
                description: caps["desc"].trim().to_string(),
            };

            params.push(p);

        } else if let Some(caps) = return_re.captures(line) {
            return_type = Some(Return {
                data_type: caps["type"].to_string(),
                description: caps["desc"].trim().to_string(),
            });
        }
        // Descriptions should have been parsed disparately
        // only check if we captured the param lines, or the SINGLE return line
    }

    DocComment {
        description,
        params,
        return_type,
    }
}

fn main() {
    let input = r#"
/// sends a request
/// @param url string
/// @param retries [int | float] = 3 number of retries
/// @param timeout ms = 5000 request timeout
/// @return Response description
bunch of typst code here ....


/// sends a different
/// @param url string
/// @param retries [float | content] = 3 number of retries
/// @param timeout int = 5000 request timeout
/// @return Response description
more real typst code


/// does something that takes no params and doesnt return
code here

    "#;

    let docs = parse_document(input);
    println!("{:#?}", docs);
}
