mod tests;

use std::mem;

use incodoc::*;
use pulldown_cmark::{ Parser, Options, Event, Tag, TagEnd, CodeBlockKind, LinkType };

pub fn parse_md_to_incodoc(input: &str) -> Doc {
    let options = Options::all();
    let parser = Parser::new_ext(input, options);

    let mut scap = false; // string capture: if a tag started that captures a string
    let mut lcap = false; // link capture: capture em and text for links
    let mut pre_head = true;
    let mut in_list_item = false;
    let mut em_lvl = 0;
    let mut sc_lvl = 0;
    let mut html_indent = 0;

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
    let mut link = Link::default();
    let mut doc = Doc::default();

    for event in parser {
        println!("{:?}", event);
        match event {
            Event::Text(text) => {
                string.push_str(&text);
                if lcap && em_lvl == 0 && sc_lvl == 0 {
                    link.items.push(LinkItem::String(mem::take(&mut string)));
                } else if !scap && em_lvl == 0 && sc_lvl == 0 {
                    par.items.push(ParagraphItem::Text(mem::take(&mut string)));
                }
            },
            Event::SoftBreak | Event::HardBreak | Event::Rule => {
                string.push('\n');
            },
            // Event::Start(Tag::Paragraph) => {},
            Event::End(TagEnd::Paragraph) if !in_list_item & !par.items.is_empty() => {
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
                tags.insert("code".to_string());
                par.items.push(ParagraphItem::MText(TextWithMeta {
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
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                em_lvl += 1;
            },
            Event::Start(Tag::Strong) => {
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                em_lvl += 2;
            },
            Event::Start(Tag::Strikethrough) => {
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                em_lvl = -1;
            },
            Event::Start(Tag::Superscript) => {
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                sc_lvl = 1;
            },
            Event::Start(Tag::Subscript) => {
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                sc_lvl = -1;
            },
            Event::End(TagEnd::Strong) => {
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                em_lvl -= 2;
            }
            Event::End(TagEnd::Emphasis) => {
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                em_lvl -= 1;
            },
            Event::End(TagEnd::Strikethrough) => {
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                em_lvl += 1;
            },
            Event::End(TagEnd::Superscript) => {
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                sc_lvl = 0;
            },
            Event::End(TagEnd::Subscript) => {
                finish_text_piece(
                    em_lvl, sc_lvl, lcap, &mut string, &mut par.items, &mut link.items
                );
                sc_lvl = 0;
            },
            Event::Start(Tag::Link { link_type, dest_url, title, id }) => {
                link.url = dest_url.to_string();
                if !id.is_empty() {
                    link.props.insert("link-ref".to_string(), PropVal::String(id.to_string()));
                }
                if !title.is_empty() {
                    link.props.insert("title".to_string(), PropVal::String(title.to_string()));
                }
                if link_type == LinkType::Email {
                    link.tags.insert("email-address".to_string());
                }
                lcap = true;
            },
            Event::End(TagEnd::Link) => {
                par.items.push(ParagraphItem::Link(mem::take(&mut link)));
                lcap = false;
            },
            Event::Start(Tag::Image { link_type, dest_url, title, id }) => {
                link.url = dest_url.to_string();
                if !id.is_empty() {
                    link.props.insert("link-ref".to_string(), PropVal::String(id.to_string()));
                }
                if !title.is_empty() {
                    link.props.insert("title".to_string(), PropVal::String(title.to_string()));
                }
                if link_type == LinkType::Email {
                    link.tags.insert("email-address".to_string());
                }
                link.tags.insert("image".to_string());
                lcap = true;
            },
            Event::End(TagEnd::Image) => {
                par.items.push(ParagraphItem::Link(mem::take(&mut link)));
                lcap = false;
            },
            Event::Start(Tag::HtmlBlock) => {
            },
            Event::Html(html_line) => {
                string.push_str(&html_line);
            },
            Event::End(TagEnd::HtmlBlock) => {
                code_block.language = "html".to_string();
                code_block.code = mem::take(&mut string);
                code_block.tags.insert("unconv-corp".to_string());
                par.items.push(ParagraphItem::Code(Ok(mem::take(&mut code_block))));
            },
            Event::InlineHtml(tag) => {
                let old = html_indent;
                html_indent += if tag.contains("</") { -1 } else { 1 };
                if html_indent > 0 && old == 0 {
                    let em = Emphasis {
                        strength: EmStrength::Light,
                        etype: EmType::Deemphasis,
                        text: "html(".to_string(),
                        ..Default::default()
                    };
                    par.items.push(ParagraphItem::Em(em));
                } else if html_indent == 0 && old > 0 {
                    let em = Emphasis {
                        strength: EmStrength::Light,
                        etype: EmType::Deemphasis,
                        text: ")".to_string(),
                        ..Default::default()
                    };
                    par.items.push(ParagraphItem::Em(em));
                }
            },
            Event::InlineMath(math) => {
                let mut tags = Tags::default();
                tags.insert("latex-math".to_string());
                par.items.push(ParagraphItem::MText(TextWithMeta {
                    text: math.to_string(),
                    tags,
                    ..Default::default()
                }));
            },
            Event::DisplayMath(math) => {
                code_block.language = "latex-math".to_string();
                code_block.code = math.to_string();
                code_block.mode = CodeModeHint::Replace;
                par.items.push(ParagraphItem::Code(Ok(mem::take(&mut code_block))));
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

fn finish_text_piece(
    em_lvl: i32, sc_lvl: i32, lcap: bool,
    string: &mut String, pis: &mut Vec<ParagraphItem>, lis: &mut Vec<LinkItem>,
) {
    if string.is_empty() { return; }
    let text = mem::take(string);
    if em_lvl == 0 && sc_lvl == 0 {
        if lcap {
            lis.push(LinkItem::String(text));
        } else {
            pis.push(ParagraphItem::Text(text));
        }
        return;
    }
    let (strength, etype) = match em_lvl {
        2  => (EmStrength::Medium, EmType::Emphasis),
        3  => (EmStrength::Strong, EmType::Emphasis),
        -1 => (EmStrength::Medium, EmType::Deemphasis),
        _  => (EmStrength::Light, EmType::Emphasis),
    };
    let mut tags = Tags::default();
    if sc_lvl == 1 {
        tags.insert("super".to_string());
    } else if sc_lvl == -1 {
        tags.insert("sub".to_string());
    }
    if em_lvl == 0 {
        if lcap {
            lis.push(LinkItem::String(text));
        } else {
            pis.push(ParagraphItem::MText(TextWithMeta{ text, tags, ..Default::default() }));
        }
    } else {
        let em = Emphasis { strength, etype, text, tags, ..Default::default() };
        if lcap {
            lis.push(LinkItem::Em(em));
        } else {
            pis.push(ParagraphItem::Em(em));
        }
    }
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

