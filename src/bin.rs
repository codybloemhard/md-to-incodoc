use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;

const INPUT: &str =
"
Here's a simple footnote,[^1] and here's a longer one.[^bignote]

### Footnotes

[^1]: This is the first footnote.

[^bignote]:
    Here's one with multiple paragraphs and code.

    Indent paragraphs to include them in the footnote.
    `{ my code }`

    Add as many paragraphs as you like.
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // println!("{:#?}", doc);
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

