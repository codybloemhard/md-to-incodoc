use pulldown_cmark::{ Parser, Options, Event, Tag, TagEnd };
use incodoc::*;

const INPUT: &str = "
# Header

haha this is some text.
and some more.
new sentence.

new paragraph?
yes it is.

## H2

h2 par

#### H4

h4 par

### H3

h3 par

###### H6

h6 par

## H2

h2 par
";

pub fn test() -> Doc {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(INPUT, options);

    let mut doc = Doc::default();
    let mut string = String::new();
    let mut par = Paragraph::default();
    let mut head = Heading::default();
    let mut section_items = Vec::new();
    let mut pre_sections = Vec::new();

    for event in parser {
        match event {
            Event::Text(text) => {
                string.push_str(&text);
                string.push('\n');
                println!("  T");
            },
            Event::Start(Tag::Paragraph) => {
                println!("+P");
            },
            Event::End(TagEnd::Paragraph) => {
                println!("-P");
                par.items.push(ParagraphItem::Text(std::mem::take(&mut string)));
                section_items.push(SectionItem::Paragraph(std::mem::take(&mut par)));
            },
            Event::Start(Tag::Heading { level, id, classes, attrs }) => {
                println!("+H");
                // commit current section
                pre_sections.push((std::mem::take(&mut head), std::mem::take(&mut section_items)));
                // set up new heading for new section
                head.level = level as u8; // not the final head level
                if let Some(id) = id {
                    head.props.insert(
                        "id".to_string(),
                        PropVal::String(id.to_string())
                    );
                }
                for (attr, attr_val) in attrs {
                    if let Some(val) = attr_val {
                        head.props.insert(attr.to_string(), PropVal::String(val.to_string()));
                    }
                }
                for class in classes {
                    head.tags.insert(class.to_string());
                }
            },
            Event::End(TagEnd::Heading(_level)) => {
                println!("-H");
                head.items.push(HeadingItem::String(std::mem::take(&mut string)));
            },
            _ => { },
        }
    }
    pre_sections.push((std::mem::take(&mut head), std::mem::take(&mut section_items)));

    let mega_section = pre_sections_to_sections(pre_sections);
    let sections = mega_section_to_sections(mega_section);

    println!("{:#?}", sections);

    doc
}

fn pre_sections_to_sections(mut pres: Vec<(Heading, Vec<SectionItem>)>) -> Section {
    if pres.is_empty() {
        return Section::default();
    }
    let mut min = u8::MAX;
    for (head, _) in pres.iter().skip(1) {
        if head.level < min {
            min = head.level;
        }
    }
    let first = std::mem::take(&mut pres[0]);
    let mut buckets = Vec::new();
    let mut bucket = Vec::new();
    for pre in pres.into_iter().skip(1) {
        if pre.0.level == min {
            buckets.push(std::mem::take(&mut bucket));
            bucket.push(pre);
        } else {
            bucket.push(pre);
        }
    }
    buckets.push(std::mem::take(&mut bucket));
    let mut res = Vec::new();
    for item in first.1 {
        res.push(item);
    }
    for b in buckets.into_iter() {
        if !b.is_empty() {
            res.push(SectionItem::Section(pre_sections_to_sections(b)));
        }
    }
    Section {
        heading: first.0,
        items: res,
        tags: Tags::default(),
        props: Props::default(),
    }
}

pub fn mega_section_to_sections(mega: Section) -> Vec<Section> {
    let mut res = Vec::new();
    for item in mega.items {
        if let SectionItem::Section(mut section) = item {
            downgrade_section(&mut section);
            res.push(section);
        }
    }
    res
}

pub fn downgrade_section(section: &mut Section) {
    if section.heading.level > 0 {
        section.heading.level -= 1;
    }
    for item in &mut section.items {
        if let SectionItem::Section(sub_section) = item {
            downgrade_section(sub_section);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
