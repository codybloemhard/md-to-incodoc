use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;

const INPUT: &str =
"
- a
  - b
  - b
    1. c
    1. c
       - [x] d
       - [ ] d
       - [ ] d

- [ ] T
- [x] T
  - a
  - a
- [ ] T
  - b
  - b
- [x] T
  - c
  - c
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // println!("{:?}", doc);
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

