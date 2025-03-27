use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;

const INPUT: &str =
"
This is a ***test** test*.
This is a *test **test***.
*a*
**b**
***c***
*A**B**A**B**A*
*A*B*A*
pre ~~strike~~ post
***A~~strike~~B***
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // println!("{:?}", doc);
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

