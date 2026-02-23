
/// sends a request
///
/// @param url string
/// @param retries int = 3 number of retries
/// @param timeout ms = 5000 request timeout
/// @return type description


/// description
/// @param <name> <type> [= default_val] [description]
/// @return <type> description



//type can be unioned with [sdf|]


struct DocComment {
    description: String,
    params: Option<Vec<Param>>,
    return_type: Option<Return>
}

struct Return {
    data_type: String,
    description: String,
}

struct Param {
    name: String,
    data_type: Vec<String>,
    default: Option<String>,
    description: String,
}


fn main() {
    println!("Hello, world!");

}
