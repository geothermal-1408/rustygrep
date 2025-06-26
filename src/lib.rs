use std::process;

fn zero_lines_found() {
    eprintln!("[INFO]: no lines found");
    process::exit(69);
}

pub fn case_insensitive<'a> (query: &str, content: &'a str ) -> Vec<&'a str>{
    let mut res = Vec::new();

    let query = query.to_lowercase();
    for line in content.lines() {
	if line.to_lowercase().contains(&query) {
	    res.push(line);
    }
    }
    if res.len() == 0 {
        zero_lines_found();
    }
    return res;
}
 
pub fn case_sensitive<'a> (query: &str, content: &'a str ) -> Vec<&'a str>{
    let mut res = Vec::new();
    for line in content.lines() {
	if line.contains(&query) {
	    res.push(line);
    }
    }
    if res.len() == 0 {
        zero_lines_found();
    }
    return res;
}