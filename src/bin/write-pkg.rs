use std::fs;

const PATH: &str = "pkg/package.json";

fn main() {
    let wrong = fs::read_to_string(PATH).unwrap();
    if wrong.contains("\"snippets/*\"") {
        return;
    }
    let right = wrong.replace("\"files\": [", "\"files\": [\n    \"snippets/*\",");
    fs::write(PATH, right).unwrap();
}
