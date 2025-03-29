use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;

const INPUT: &str =
"
*^super^*
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // println!("{:?}", doc);
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

