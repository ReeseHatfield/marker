use std::{env::{self}, fs::File, io::Read};

use regex::Regex;


trait Markdownable {
    fn markdown(&self) -> String;
}


#[derive(Debug)]
struct DocComment {
    description: String,
    params: Vec<Param>,
    return_type: Option<Return>,
}

impl Markdownable for DocComment {
    fn markdown(&self) -> String {
        let mut md = String::new();

        // get the title via split on ": " from descriptions

        let parts: Vec<String> = self.description.split(": ").map(|s| s.to_string()).collect();
        let panic_msg = "Could not parse doc header. Ensure your header follows the `title: description' format".to_string();
        let title = parts.first().expect(&panic_msg).to_owned();
        let real_description = parts.get(1).expect(&panic_msg).to_owned();

        md.push_str("## ");
        md.push_str(&title);
        md.push('\n');
        md.push_str(&real_description);
        md.push('\n');


        if !self.params.is_empty() {
            md.push_str("### Parameters: ");
            md.push('\n');

            self.params.iter().for_each(|p| {

                md.push_str(&p.markdown());
            });
        }

        if let Some(ret) = self.return_type.clone() {
            md.push_str("### Returns: ");
            md.push('\n');
            md.push_str(&ret.markdown());
        }
        md.push('\n');

        md
    }
}

#[derive(Debug, Clone)]
struct Return {
    data_type: String,
    description: String,
}

impl Markdownable for Return{
    fn markdown(&self) -> String {
       format!("`{}`: {} \n", self.data_type, self.description)
    }
}


#[derive(Debug)]
struct Param {
    name: String,
    data_type: Vec<String>,
    default: Option<String>,
    description: String,
}

impl Markdownable for Param{
    fn markdown(&self) -> String {
        let data_type_str = self.data_type.join(" | ");

        let mut default_str = String::new();
        if let Some(def) = self.default.clone() {
            default_str = format!("(default: {})", def);
        };

        format!("{}: `{}` {} {} \n \n",self.name, data_type_str, default_str, self.description)
    }
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
    let lines = block.lines();

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
    let args: Vec<String> = env::args().collect::<Vec<String>>().split_off(1);

    args.iter().for_each(|f| {
        let mut file = File::open(f).unwrap_or_else(|_| panic!("Could not read file {f}"));

        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap_or_else(|_| panic!("Could not read file {f}"));

        let docs = parse_document(&contents);

        docs.iter().for_each(|f| {
            println!("{}", f.markdown());
        });

    });

}
