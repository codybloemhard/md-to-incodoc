mod tests;

use std::mem;

use incodoc::*;
use pulldown_cmark::{ Parser, Options, Event, Tag, TagEnd, CodeBlockKind };

pub fn parse_md_to_incodoc(input: &str) -> Doc {
    let options = Options::all();
    let parser = Parser::new_ext(input, options);

    let mut scap = false; // string capture: if a tag started that captures a string
    let mut pre_head = true;
    let mut in_list_item = false;
    let mut em_lvl = 0;

    let mut string = String::new();
    let mut code_lang = String::new();

    let mut par_stack = Vec::new();
    let mut list_stack = Vec::new();
    let mut section_items = Vec::new();
    let mut pre_sections = Vec::new();

    let mut par = Paragraph::default();
    let mut head = Heading::default();
    let mut code_block = CodeBlock::default();
    let mut list = List::default();
    let mut doc = Doc::default();

    for event in parser {
        println!("{:?}", event);
        match event {
            Event::Text(text) => {
                string.push_str(&text);
                if !scap {
                    par.items.push(ParagraphItem::Text(mem::take(&mut string)));
                }
            },
            Event::SoftBreak | Event::HardBreak => {
                string.push('\n');
            },
            // Event::Start(Tag::Paragraph) => {},
            Event::End(TagEnd::Paragraph) if !in_list_item => {
                let par = mem::take(&mut par);
                if pre_head {
                    doc.items.push(DocItem::Paragraph(par));
                } else {
                    section_items.push(SectionItem::Paragraph(par));
                }
            },
            Event::Start(Tag::Heading { level, id, classes, attrs }) => {
                // commit current section
                pre_sections.push((mem::take(&mut head), mem::take(&mut section_items)));
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
                    } else {
                        head.tags.insert(attr.to_string());
                    }
                }
                for class in classes {
                    head.tags.insert(class.to_string());
                }
                scap = true;
                pre_head = false;
            },
            Event::End(TagEnd::Heading(_level)) => {
                head.items.push(HeadingItem::String(mem::take(&mut string)));
                scap = false;
            },
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(language))) => {
                if !language.is_empty(){
                    code_lang = language.to_string();
                }
                scap = true;
            },
            Event::End(TagEnd::CodeBlock) => {
                code_block.language = mem::take(&mut code_lang);
                code_block.code = mem::take(&mut string);
                par.items.push(ParagraphItem::Code(Ok(mem::take(&mut code_block))));
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
            Event::Start(Tag::List(start_nr)) => {
                par_stack.push(mem::take(&mut par));
                list_stack.push(mem::take(&mut list));
                if start_nr.is_some() {
                    list.ltype = ListType::Distinct;
                }
            },
            Event::Start(Tag::Item) => {
                in_list_item = true;
            },
            Event::TaskListMarker(ticked) => {
                if ticked {
                    par.tags.insert("checked".to_string());
                }
                list.ltype = ListType::Checked;
            },
            Event::End(TagEnd::Item) => {
                list.items.push(mem::take(&mut par));
                in_list_item = false;
            },
            Event::End(TagEnd::List(_)) => {
                par = par_stack.pop().expect("oof");
                par.items.push(ParagraphItem::List(mem::take(&mut list)));
                list = list_stack.pop().unwrap_or_default();
            },
            Event::Start(Tag::Emphasis) => {
                if !string.is_empty() {
                    par.items.push(ParagraphItem::Text(mem::take(&mut string)));
                }
                scap = true;
                em_lvl += 1;
            },
            Event::Start(Tag::Strong) => {
                if !string.is_empty() {
                    let text = mem::take(&mut string);
                    if em_lvl == 1 {
                        par.items.push(ParagraphItem::Em(Emphasis {
                            strength: EmStrength::Light,
                            etype: EmType::Emphasis,
                            text,
                            ..Default::default()
                        }));
                    } else {
                        par.items.push(ParagraphItem::Text(text));
                    }
                }
                scap = true;
                em_lvl += 2;
            },
            Event::Start(Tag::Strikethrough) => {
                if !string.is_empty() {
                    par.items.push(ParagraphItem::Text(mem::take(&mut string)));
                }
                scap = true;
                em_lvl = -1;
            },
            Event::End(TagEnd::Strong) => {
                if em_lvl >= 2 && !string.is_empty() {
                    let strength = match em_lvl {
                        2 => EmStrength::Medium,
                        _ => EmStrength::Strong,
                    };
                    par.items.push(ParagraphItem::Em(Emphasis {
                        strength,
                        etype: EmType::Emphasis,
                        text: mem::take(&mut string),
                        ..Default::default()
                    }));
                    em_lvl -= 2;
                    if em_lvl == 0 {
                        scap = false;
                    }
                }
            }
            Event::End(TagEnd::Emphasis) => {
                let strength = match em_lvl {
                    2 => EmStrength::Medium,
                    3 => EmStrength::Strong,
                    _ => EmStrength::Light,
                };
                if !string.is_empty() {
                    par.items.push(ParagraphItem::Em(Emphasis {
                        strength,
                        etype: EmType::Emphasis,
                        text: mem::take(&mut string),
                        ..Default::default()
                    }));
                }
                em_lvl = 0;
                scap = false;
            },
            Event::End(TagEnd::Strikethrough) => {
                if !string.is_empty() {
                    par.items.push(ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        etype: EmType::Deemphasis,
                        text: mem::take(&mut string),
                        ..Default::default()
                    }));
                }
                em_lvl += 1;
                scap = false;
            },
            _ => { },
        }
    }
    if !par.items.is_empty() {
        section_items.push(SectionItem::Paragraph(mem::take(&mut par)));
    }
    pre_sections.push((mem::take(&mut head), mem::take(&mut section_items)));

    let mega_section = pre_sections_to_sections(pre_sections);
    populate_doc(&mut doc, mega_section);

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
    let first = mem::take(&mut pres[0]);
    let mut buckets = Vec::new();
    let mut bucket = Vec::new();
    for pre in pres.into_iter().skip(1) {
        if pre.0.level == min {
            buckets.push(mem::take(&mut bucket));
            bucket.push(pre);
        } else {
            bucket.push(pre);
        }
    }
    buckets.push(mem::take(&mut bucket));
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

fn populate_doc(doc: &mut Doc, mega: Section) {
    for item in mega.items {
        match item {
            SectionItem::Section(section) => {
                doc.items.push(DocItem::Section(downgraded_section(section)));
            }
            SectionItem::Paragraph(par) => {
                doc.items.push(DocItem::Paragraph(par));
            },
        }
    }
}

fn downgraded_section(mut section: Section) -> Section {
    section.heading.level -= section.heading.level.min(1);
    for item in &mut section.items {
        if let SectionItem::Section(sub_section) = item {
            *sub_section = downgraded_section(mem::take(sub_section));
        }
    }
    section
}

