use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;

const INPUT: &str =
"
- aaa
- aaa
- aaa
  1. bbb
  2. bbb
     bbb
  3.
     - [ ] ccc
     - [x] ccc
  4. - ddd
     - ddd
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // println!("{:?}", doc);
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

