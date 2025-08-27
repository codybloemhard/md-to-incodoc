use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;

const INPUT: &str =
"
Here's a simple footnote,[^1] and here's a longer one.[^bignote]

[^1]: This is the first footnote.

### Footnotes

[^bignote]:

  line par 0.
  line par 0.

  `{ code }`

  line par 2.
  line par 2.
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // println!("{:#?}", doc);
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

