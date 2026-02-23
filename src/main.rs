use std::ops::Add;

use regex::{Regex, bytes::SetMatchesIter};


trait Markdownable {
    fn into_markdown(&self) -> String;
}


#[derive(Debug)]
struct DocComment {
    description: String,
    params: Vec<Param>,
    return_type: Option<Return>,
}

impl Markdownable for DocComment {
    fn into_markdown(&self) -> String {
        let mut md = String::new();

        // get the title via split on ": " from descriptions

        let parts: Vec<String> = self.description.split(": ").map(|s| s.to_string()).collect();
        let panic_msg = "Could not parse doc header. Ensure your header follows the `title: description' format".to_string();
        let title = parts.get(0).expect(&panic_msg).to_owned();
        let real_description = parts.get(1).expect(&panic_msg).to_owned();

        md.push_str("## ");
        md.push_str(&title);
        md.push_str("\n");


        if self.params.len() > 0 {
            md.push_str("### Parameters: ");
            md.push_str("\n");

            self.params.iter().for_each(|p| {

                md.push_str(&p.into_markdown());
            });
        }

        if let Some(ret) = self.return_type.clone() {
            md.push_str("### Returns: ");
            md.push_str("\n");
            md.push_str(&ret.into_markdown());
        }

        md.push_str("\n");
        return md;
    }
}

#[derive(Debug, Clone)]
struct Return {
    data_type: String,
    description: String,
}

impl Markdownable for Return{
    fn into_markdown(&self) -> String {
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
    fn into_markdown(&self) -> String {
        let data_type_str = self.data_type.join(" | ");

        let mut default_str = String::new();
        if let Some(def) = self.default.clone() {
            default_str = format!("(default: {})", def);
        };

        format!("{}: `{}` {} {} \n",self.name, data_type_str, default_str, self.description)
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
// exam stuff

#let title-state = state("title", "")

#let total_points = counter("points")


/// exam_init: Initialize an exam with a show rule TODO fix me
#let exam_init(body) = {
    set page(margin: 40pt)
    set text(
    font: ("Verdana"),
    size: 12pt,
    fill: black,
    weight: "regular"
    )
    set raw(theme: "../themes/InspiredGitHub.tmTheme")
    show raw: set text(font: "Courier New", weight: "bold", size: 10pt)

    set par(spacing: 1.2em)

    body
}

/// header: Render a header for the exam, will check total number of points TODO fix me
#let header() = [
    #grid(
    columns: (1fr, 1fr),
    align(center)[
        #text(size: 17pt)[
        #align(left)[
            #"Name:_____________________________"
        ]
        ]
    ],
    align(center)[
        #text(size: 17pt)[
        #align(right)[
            #grid(
            rows: (0pt, 20pt),
            align(center)[
                //#context title-state.get()
            ],
            align(right)[
                #"____ /" #context total_points.final().at(0) pts
            ]
            )
        ]
        ]
    ]
    )
]

#set page(header: [
    #context title-state.get()
])

#let setup(title) = {
    title-state.update(title)
}

#let spacer() = {
    v(10pt)
}


#let cur-question = state(
    "num_qs", 1
)


/// question: Create a generic question
/// @param body content Question Body
/// @param points int Number of points the question is worth
#let question(body, points: 1) = context {
    cur-question.update(n => n + 1)
    total_points.update(p => p + points)
    let qnum = cur-question.get()

    block(width: 100%, breakable: true, inset: (bottom: 0.5em))[
    #grid(
        columns: (20pt, 1fr),
        column-gutter: 8pt,
        text(weight: "bold")[#qnum.],
        [#body #h(5pt) (#points pts)]
    )
    ]
}

#let c = counter("letter")

#let answer_indents = (1fr, 10fr, 1fr)


/// _num_to_fr_units: Map a number into a tuple of 1fr units
/// primarily used to make optional column passing to #multiple_choice easier
/// input = 3 -> output = (1fr, 1fr, 1fr)
/// input = 5 -> output = (1fr, 1fr, 1fr, 1fr, 1fr)
/// @param num int number to map
/// @return array Array of num fr units
#let _num_to_fr_units(num) = {
    range(num).map(i => 1fr)
}


/// multiple_choice: Create a multiple choice question
/// @param body content Body of question
/// @param points int = 1 Points the question is worth
/// @param cols [int | array ] = 1 Number of columns to render the answer. Pass an array of units for specific spacing e.g. (1fr, 1fr, 12pt)
#let multiple_choice(body, points: 1, cols: 1, ..answers) = {
    let cols_type = type(cols)

    block[
    #c.update(0)
    #question(body, points: points)
    #v(5pt)
    #grid(
        columns: answer_indents,
        rows: (auto),
        "",
        block[
        #grid(
            columns: {
            if(cols_type) == int {
                _num_to_fr_units(cols)
            } else {
                cols
            }
            },
            rows: auto,
            column-gutter: 5pt,
            row-gutter: 15pt,
            ..answers.pos().map(answer => {
            c.step()
            block[
                #context c.display("a"). #" " #answer
            ]
            })
        )
        ],
        "",
    )
    ]
}

/// matching: Create a matching question
/// e.g
/// Cat      A. Canine
/// Dog      B. Feline
/// Fish     C. Aquatic Create
/// @param q_body content body of question to ask
/// @param left_opts array options for the left side of question
/// @param right_opts array options for the right side of question
/// @param points int = 1 points the question is worth
#let matching(q_body, left_opts, right_opts, points: 1) = {
    // left and right are shadows
    block[
    #c.update(0)
    #question(q_body, points: points)
    #spacer()
    #grid(
    // should sum to 12, to match answer_indents
    // 1st is just a spacer
        columns: (1fr, 4fr, 7fr),
        "",
        align(left)[
        #for word in left_opts {
            block[
            #"____" #word
            #spacer()
            ]
        }
        ],
        align(left)[
        #for x in right_opts {
            block[
            #c.step()
            #context c.display("a"). #" " #x
            #spacer()
            ]
        }
        ]
    )
    ]
}

// need better name
#let multi_true_false(q_body, ..statements, points: 1) = {
    let num = counter("I")
    // Note: If you want to skip the first value (N),
    // ensure your counter logic matches your document setup.
    num.step()
    block[
    #question(q_body, points: points)
    #for statement in statements.pos() {
        block[
        #grid(
            columns: (42pt, 18pt, 9fr, 1fr),
            rows: (auto),
            "",
            block[
            #set text(font: "Libertinus Serif")
            #context num.display("I").
            #context num.step()
            ],
            statement,
            "______"
        )
        ]
    }
    ]
}

/// short_answer: Create a short answer question
/// @param q_body content Question Body
/// @param lines int = 1 lines of space to give the user, renders as actual lines
/// @param points int = 1 points the question is worth
#let short_answer(q_body, lines: 1, points: 1) = {
    question(q_body, points: points)

    // you don't need the full spacing from the question before the first line
    v(-10pt)
    block(width: 100%, inset: (left: 20pt))[
    #for _ in range(lines) {
        // line spacing
        v(25pt)
        line(length: 90%, stroke: 0.5pt)
    }
    ]
}

/// free_response: Create a free response question
/// @param q_body content Question Body
/// @param lines int = 1 lines of space to give the user, renders as empty space
/// @param points int = 1 points the question is worth
#let free_response(q_body, lines: 1, points: 1) = {
    question(q_body, points: points)

    // i did not know you could just multiply units like that
    v(15pt * lines)
}


/// code_block: Create a code block formatted for exams
/// Wraps in box to the edge of the code, can add white space if need it to be longer
/// @param raw_code content(raw) raw code block, eg. ```java public class...```
#let code_block(raw_code) = {
    box(stroke: (paint: rgb("d9d9d9"), thickness: 2pt, cap: "round"), inset: (8pt))[
        #raw_code
    ]
}
"#;


    /// sends a request
    /// @param url string
    /// @param retries [int | float] = 3 number of retries
    /// @param timeout ms = 5000 request timeout
    /// @return Response description

    let docs = parse_document(input);

    docs.iter().for_each(|f| {
        println!("{}", f.into_markdown());
    });
}
