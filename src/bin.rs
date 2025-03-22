use md_to_incodoc::test;
use incodoc::output::doc_out;

fn main() {
    let doc = test();
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

