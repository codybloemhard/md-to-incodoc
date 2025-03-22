use pulldown_cmark::{ Parser, Options, Event, Tag, TagEnd, CodeBlockKind };
use incodoc::*;

const INPUT: &str = "
# head

This is some `inline code`.
Now we will end this paragraph.

``` rust
let x = 0;
```

test
";

pub fn test() -> Doc {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(INPUT, options);

    let mut doc = Doc::default();
    let mut string = String::new();
    let mut scap = false; // string capture: if a tag started that captures a string
    let mut par = Paragraph::default();
    let mut head = Heading::default();
    let mut pre_head = true;
    let mut section_items = Vec::new();
    let mut pre_sections = Vec::new();
    let mut code_lang = "plain".to_string();
    let mut code_block = CodeBlock::default();

    for event in parser {
        // println!("{:?}", event);
        match event {
            Event::Text(text) => {
                string.push_str(&text);
                string.push('\n');
                if !scap {
                    par.items.push(ParagraphItem::Text(std::mem::take(&mut string)));
                }
            },
            Event::Start(Tag::Paragraph) => {
            },
            Event::End(TagEnd::Paragraph) => {
                if pre_head {
                    doc.items.push(DocItem::Paragraph(std::mem::take(&mut par)));
                } else {
                    section_items.push(SectionItem::Paragraph(std::mem::take(&mut par)));
                }
            },
            Event::Start(Tag::Heading { level, id, classes, attrs }) => {
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
                scap = true;
                pre_head = false;
            },
            Event::End(TagEnd::Heading(_level)) => {
                head.items.push(HeadingItem::String(std::mem::take(&mut string)));
                scap = false;
            },
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(language))) => {
                if !language.is_empty(){
                    code_lang = language.to_string();
                }
                scap = true;
            },
            Event::End(TagEnd::CodeBlock) => {
                code_block.language = std::mem::take(&mut code_lang);
                code_block.code = std::mem::take(&mut string);
                code_lang.push_str("plain");
                par.items.push(ParagraphItem::Code(Ok(std::mem::take(&mut code_block))));
                scap = false;
            },
            Event::Code(codet) => {
                let mut tags = Tags::default();
                tags.insert("inline-code".to_string());
                par.items.push(ParagraphItem::MText(TextWithMeta{
                    text: codet.to_string(),
                    tags,
                    ..Default::default()
                }));
            },
            _ => { },
        }
    }
    if !par.items.is_empty() {
        section_items.push(SectionItem::Paragraph(std::mem::take(&mut par)));
    }
    pre_sections.push((std::mem::take(&mut head), std::mem::take(&mut section_items)));

    let mega_section = pre_sections_to_sections(pre_sections);
    let sections = mega_section_to_sections(mega_section);
    for section in sections {
        doc.items.push(DocItem::Section(section));
    }

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

fn mega_section_to_sections(mega: Section) -> Vec<Section> {
    let mut res = Vec::new();
    for item in mega.items {
        if let SectionItem::Section(mut section) = item {
            downgrade_section(&mut section);
            res.push(section);
        }
    }
    res
}

fn downgrade_section(section: &mut Section) {
    section.heading.level -= section.heading.level.min(1);
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
