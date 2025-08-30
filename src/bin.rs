use md_to_incodoc::parse_md_to_incodoc;
use incodoc::output::doc_out;

const INPUT: &str =
"
test line

+++
tags a b c
prop lang en
prop id some-id
nav
  nav top level
    link home $ /home
    link about $ /about
    nav sub nav
      link extra link $ /sub/extra
    end
    nav other
      link i like cheese $ ./cheese
      link sosig is happiness $ ./sosig
    end
  end
end

yay

line
+++
";

fn main() {
    let doc = parse_md_to_incodoc(INPUT);
    // println!("{:#?}", doc);
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

