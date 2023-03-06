pub fn no_convention(input: Vec<String>) -> String {
    input.join(" ")
}

pub fn flat(input: Vec<String>) -> String {
    input.join("")
}

pub fn upper(input: Vec<String>) -> String {
    input.join("").to_uppercase()
}

pub fn lower_camel(input: Vec<String>) -> String {
    let output = uppercase_vec(input, 1);
    output.join("")
}

pub fn snake(input: Vec<String>) -> String {
    input.join("_")
}

pub fn kebab(input: Vec<String>) -> String {
    input.join("-")
}

pub fn train(input: Vec<String>) -> String {
    let output = uppercase_vec(input, 0);
    output.join("-")
}

pub fn pascal(input: Vec<String>) -> String {
    let output = uppercase_vec(input, 0);
    output.join("")
}

pub fn camel_snake(input: Vec<String>) -> String {
    snake(uppercase_vec(input, 1))
}

pub fn pascal_snake(input: Vec<String>) -> String {
    snake(uppercase_vec(input, 0))
}

pub fn screaming_snake(input: Vec<String>) -> String {
    snake(vec_to_scream(input))
}

pub fn screaming_kebab(input: Vec<String>) -> String {
    kebab(vec_to_scream(input))
}

fn vec_to_scream(mut input: Vec<String>) -> Vec<String> {
    for i in 0..input.len() {
        input[i] = input[i].to_uppercase();
    }
    
    input
}

fn uppercase_vec(mut input: Vec<String>, start_pos: usize) -> Vec<String> {
    for i in start_pos..input.len() {
        input[i] = uppercase_first_letter(&input[i]);
    }

    input
}

fn uppercase_first_letter(string: &str) -> String {
    let mut c = string.chars();

    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
