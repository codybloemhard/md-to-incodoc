use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;

const INPUT: &str =
"
regular line [^ref]

[^ref]: test ref

regular line

> regular quote

regular line

> [!NOTE]
> note quote
> > [!WARNING]
> > test
> test

regular line

line
> quote
quote
quote

line
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // println!("{:#?}", doc);
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

