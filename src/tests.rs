#[cfg(test)]
mod tests {
    use crate::*;

    use std::collections::{ HashSet, HashMap };

    macro_rules! props {
        ($slice:expr) => {
            HashMap::from($slice)
        }
    }

    macro_rules! hset {
        ($slice:expr) => {
            HashSet::from_iter($slice.iter().map(|s| s.to_string()))
        }
    }

    macro_rules! test {
        ($name:ident, $string:expr, $result:expr) => {
            #[test]
            fn $name() {
                let incodoc = parse_md_to_incodoc($string);
                assert_eq!(incodoc, $result);
            }
        }
    }

    test!(
        t_empty,
        "",
        Doc::default()
    );

    test!(
        t_par_c0,
        "
par par par
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("par par par".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_par_c1,
        "
par par par
par par par
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("par par par".to_string()),
                    ParagraphItem::Text("\npar par par".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_par_c2,
        "
par par par

par par par
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("par par par".to_string()),
                    ],
                    ..Default::default()
                }),
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("par par par".to_string()),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_par_c3,
        "
par par par
par par par

par par par
par par par
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("par par par".to_string()),
                        ParagraphItem::Text("\npar par par".to_string()),
                    ],
                    ..Default::default()
                }),
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("par par par".to_string()),
                        ParagraphItem::Text("\npar par par".to_string()),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c0,
        "
# H1
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("H1".to_string()),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c1,
        "
# H1
# H1
# H1
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("H1".to_string()),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("H1".to_string()),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("H1".to_string()),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c2,
        "
## H2
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 1,
                        items: vec![
                            HeadingItem::String("H2".to_string()),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c3,
        "
### H3
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 2,
                        items: vec![
                            HeadingItem::String("H3".to_string()),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c4,
        "
#### H4
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 3,
                        items: vec![
                            HeadingItem::String("H4".to_string()),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c5,
        "
##### H5
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 4,
                        items: vec![
                            HeadingItem::String("H5".to_string()),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c6,
        "
###### H6
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 5,
                        items: vec![
                            HeadingItem::String("H6".to_string()),
                        ],
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c7,
        "
####### H7
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("####### H7".to_string()),
                    ],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c8,
        "
# H1
## H2
### H3
#### H4
##### H5
###### H6
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![HeadingItem::String("H1".to_string())],
                        ..Default::default()
                    },
                    items: vec![SectionItem::Section(Section {
                        heading: Heading {
                            level: 1,
                            items: vec![HeadingItem::String("H2".to_string())],
                            ..Default::default()
                        },
                        items: vec![SectionItem::Section(Section {
                            heading: Heading {
                                level: 2,
                                items: vec![HeadingItem::String("H3".to_string())],
                                ..Default::default()
                            },
                            items: vec![SectionItem::Section(Section {
                                heading: Heading {
                                    level: 3,
                                    items: vec![HeadingItem::String("H4".to_string())],
                                    ..Default::default()
                                },
                                items: vec![SectionItem::Section(Section {
                                    heading: Heading {
                                        level: 4,
                                        items: vec![HeadingItem::String("H5".to_string())],
                                        ..Default::default()
                                    },
                                    items: vec![SectionItem::Section(Section {
                                        heading: Heading {
                                            level: 5,
                                            items: vec![HeadingItem::String("H6".to_string())],
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    })],
                                    ..Default::default()
                                })],
                                ..Default::default()
                            })],
                            ..Default::default()
                        })],
                        ..Default::default()
                    })],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_section_c9,
        "
par
# H1
par
## H2
par
#### H4
par
### H3
par
###### H6
par
# H1
par
## H2
par
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![ParagraphItem::Text("par".to_string())],
                    ..Default::default()
                }),
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![HeadingItem::String("H1".to_string())],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Paragraph(Paragraph {
                            items: vec![ParagraphItem::Text("par".to_string())],
                            ..Default::default()
                        }),
                        SectionItem::Section(Section {
                            heading: Heading {
                                level: 1,
                                items: vec![HeadingItem::String("H2".to_string())],
                                ..Default::default()
                            },
                            items: vec![
                                SectionItem::Paragraph(Paragraph {
                                    items: vec![ParagraphItem::Text("par".to_string())],
                                    ..Default::default()
                                }),
                                SectionItem::Section(Section {
                                    heading: Heading {
                                        level: 3,
                                        items: vec![HeadingItem::String("H4".to_string())],
                                        ..Default::default()
                                    },
                                    items: vec![
                                        SectionItem::Paragraph(Paragraph {
                                            items: vec![ParagraphItem::Text("par".to_string())],
                                            ..Default::default()
                                        }),
                                    ],
                                    ..Default::default()
                                }),
                                SectionItem::Section(Section {
                                    heading: Heading {
                                        level: 2,
                                        items: vec![HeadingItem::String("H3".to_string())],
                                        ..Default::default()
                                    },
                                    items: vec![
                                        SectionItem::Paragraph(Paragraph {
                                            items: vec![ParagraphItem::Text("par".to_string())],
                                            ..Default::default()
                                        }),
                                        SectionItem::Section(Section {
                                            heading: Heading {
                                                level: 5,
                                                items: vec![HeadingItem::String("H6".to_string())],
                                                ..Default::default()
                                            },
                                            items: vec![
                                                SectionItem::Paragraph(Paragraph {
                                                    items: vec![
                                                        ParagraphItem::Text("par".to_string())
                                                    ],
                                                    ..Default::default()
                                                }),
                                            ],
                                            ..Default::default()
                                        }),
                                    ],
                                    ..Default::default()
                                }),
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![HeadingItem::String("H1".to_string())],
                        ..Default::default()
                    },
                    items: vec![
                        SectionItem::Paragraph(Paragraph {
                            items: vec![ParagraphItem::Text("par".to_string())],
                            ..Default::default()
                        }),
                        SectionItem::Section(Section {
                            heading: Heading {
                                level: 1,
                                items: vec![HeadingItem::String("H2".to_string())],
                                ..Default::default()
                            },
                            items: vec![
                                SectionItem::Paragraph(Paragraph {
                                    items: vec![ParagraphItem::Text("par".to_string())],
                                    ..Default::default()
                                }),
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_heading_meta_c0,
        "
# H1 { #id }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("H1".to_string()),
                        ],
                        props: props!([("id".to_string(), PropVal::String("id".to_string()))]),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_heading_meta_c1,
        "
# H1 { .class0 .class1 }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("H1".to_string()),
                        ],
                        tags: hset!(["class0", "class1"]),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_heading_meta_c2,
        "
# H1 { key=val valueless }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("H1".to_string()),
                        ],
                        tags: hset!(["valueless"]),
                        props: props!([("key".to_string(), PropVal::String("val".to_string()))]),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_heading_meta_c3,
        "
# H1 { key=val valueless #id .class }
        ",
        Doc {
            items: vec![
                DocItem::Section(Section {
                    heading: Heading {
                        level: 0,
                        items: vec![
                            HeadingItem::String("H1".to_string()),
                        ],
                        tags: hset!(["class", "valueless"]),
                        props: props!([
                            ("id".to_string(), PropVal::String("id".to_string())),
                            ("key".to_string(), PropVal::String("val".to_string())),
                        ]),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_inlinecode_c0,
        "
`code`
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::MText(TextWithMeta {
                            text: "code".to_string(),
                            tags: hset!(["inline-code"]),
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_inlinecode_c1,
        "
text `code` text
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("text ".to_string()),
                        ParagraphItem::MText(TextWithMeta {
                            text: "code".to_string(),
                            tags: hset!(["inline-code"]),
                            ..Default::default()
                        }),
                        ParagraphItem::Text(" text".to_string()),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_codeblock_c0,
        "
```
code {
    code
}
```
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Code(Ok(CodeBlock{
                            code:
"code {
    code
}
"
                                .to_string(),
                            ..Default::default()
                        })),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_codeblock_c1,
        "
``` rust
let x = 0;
```
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Code(Ok(CodeBlock{
                            language: "rust".to_string(),
                            code:
"let x = 0;
"
                                .to_string(),
                            ..Default::default()
                        })),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c0,
        "
- aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Identical,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c1,
        "
1. aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Distinct,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c2,
        "
- [ ] aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Checked,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c3,
        "
- [x] aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Checked,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    tags: hset!(["checked"]),
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );
    test!(
        t_list_c4,
        "
- aaa
- aaa
- aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Identical,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c5,
        "
1. aaa
2. aaa
3. aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Distinct,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c6,
        "
- [x] aaa
- [ ] aaa
- [x] aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Checked,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    tags: hset!(["checked"]),
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    tags: hset!(["checked"]),
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c7,
        "
- aaa
  aaa
- aaa
  aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Identical,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                        ParagraphItem::Text("\naaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                        ParagraphItem::Text("\naaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c8,
        "
1. aaa
   aaa
2. aaa
   aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Distinct,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                        ParagraphItem::Text("\naaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                        ParagraphItem::Text("\naaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c9,
        "
- [ ] aaa
      aaa
- [x] aaa
      aaa
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Checked,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                        ParagraphItem::Text("\naaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                        ParagraphItem::Text("\naaa".to_string()),
                                    ],
                                    tags: hset!(["checked"]),
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_list_c10,
        "
- aaa
- aaa
- aaa
  1. bbb
  2. bbb
     bbb
  3.
     - [ ] ccc
     - [x] ccc
  4. - ddd
     - ddd
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::List(List{
                            ltype: ListType::Identical,
                            items: vec![
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                    ],
                                    ..Default::default()
                                },
                                Paragraph {
                                    items: vec![
                                        ParagraphItem::Text("aaa".to_string()),
                                        ParagraphItem::List(List {
                                            ltype: ListType::Distinct,
                                            items: vec![
                                                Paragraph {
                                                    items: vec![
                                                        ParagraphItem::Text("bbb".to_string()),
                                                    ],
                                                    ..Default::default()
                                                },
                                                Paragraph {
                                                    items: vec![
                                                        ParagraphItem::Text("bbb".to_string()),
                                                        ParagraphItem::Text("\nbbb".to_string()),
                                                    ],
                                                    ..Default::default()
                                                },
                                                Paragraph {
                                                    items: vec![
                                                        ParagraphItem::List(List {
                                                            ltype: ListType::Checked,
                                                            items: vec![
                                                                Paragraph {
                                                                    items: vec![
                                                                        ParagraphItem::Text("ccc".to_string()),
                                                                    ],
                                                                    ..Default::default()
                                                                },
                                                                Paragraph {
                                                                    items: vec![
                                                                        ParagraphItem::Text("ccc".to_string()),
                                                                    ],
                                                                    tags: hset!(["checked"]),
                                                                    ..Default::default()
                                                                },
                                                            ],
                                                            ..Default::default()
                                                        }),
                                                    ],
                                                    ..Default::default()
                                                },
                                                Paragraph {
                                                    items: vec![
                                                        ParagraphItem::List(List {
                                                            ltype: ListType::Identical,
                                                            items: vec![
                                                                Paragraph {
                                                                    items: vec![
                                                                        ParagraphItem::Text("ddd".to_string()),
                                                                    ],
                                                                    ..Default::default()
                                                                },
                                                                Paragraph {
                                                                    items: vec![
                                                                        ParagraphItem::Text("ddd".to_string()),
                                                                    ],
                                                                    ..Default::default()
                                                                },
                                                            ],
                                                            ..Default::default()
                                                        }),
                                                    ],
                                                    ..Default::default()
                                                },
                                            ],
                                            ..Default::default()
                                        }),
                                    ],
                                    ..Default::default()
                                },
                            ],
                            ..Default::default()
                        }),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c0,
        "
*a*
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c1,
        "
**a**
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c2,
        "
***a***
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c3,
        "
~~a~~
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        etype: EmType::Deemphasis,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c4,
        "
pre *a* int **a** int ***a*** int ~~a~~ post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" int ".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" int ".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" int ".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        etype: EmType::Deemphasis,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c5,
        "
pre *a **b** a **b** a* post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: "a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: " a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: " a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c6,
        "
pre ***a** b **a** b **a*** post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: " b ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: " b ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c7,
        "
pre **a *b* a *b* a** post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        text: "a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        text: " a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        text: " a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c8,
        "
pre ***a* b *a* b *a*** post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        text: " b ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Medium,
                        text: " b ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Strong,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_emphasis_c9,
        "
pre *a*b*a*b*a* post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text("b".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text("b".to_string()),
                    ParagraphItem::Em(Emphasis {
                        strength: EmStrength::Light,
                        text: "a".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_supersub_c0,
        "
pre ^super^ post
pre ~sub~ post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::MText(TextWithMeta{
                        text: "super".to_string(),
                        tags: hset!(["super"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                    ParagraphItem::Text("\npre ".to_string()),
                    ParagraphItem::MText(TextWithMeta{
                        text: "sub".to_string(),
                        tags: hset!(["sub"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_supersub_c1,
        "
pre *^super^* post
pre *~sub~* post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: "super".to_string(),
                        tags: hset!(["super"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                    ParagraphItem::Text("\npre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: "sub".to_string(),
                        tags: hset!(["sub"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_supersub_c2,
        "
pre **^super^** post
pre **~sub~** post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Medium,
                        text: "super".to_string(),
                        tags: hset!(["super"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                    ParagraphItem::Text("\npre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Medium,
                        text: "sub".to_string(),
                        tags: hset!(["sub"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_supersub_c3,
        "
pre ***^super^*** post
pre ***~sub~*** post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Strong,
                        text: "super".to_string(),
                        tags: hset!(["super"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                    ParagraphItem::Text("\npre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Strong,
                        text: "sub".to_string(),
                        tags: hset!(["sub"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    // pulldown-cmark doesn't generate the right events or combining strikethrough and subscript
    // could be supported if it did
    test!(
        t_supersub_c4,
        "
pre ~~^super^~~ post
pre ~~~sub~~~ post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Medium,
                        etype: EmType::Deemphasis,
                        text: "super".to_string(),
                        tags: hset!(["super"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                    ParagraphItem::Text("\npre ~~~sub~~~ post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_supersub_c5,
        "
pre *a *^super^* b* post
pre *a *~sub~* b* post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: "a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Medium,
                        text: "super".to_string(),
                        tags: hset!(["super"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: " b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                    ParagraphItem::Text("\npre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: "a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Medium,
                        text: "sub".to_string(),
                        tags: hset!(["sub"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: " b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_supersub_c6,
        "
pre *a **^super^** b* post
pre *a **~sub~** b* post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: "a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Strong,
                        text: "super".to_string(),
                        tags: hset!(["super"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: " b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                    ParagraphItem::Text("\npre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: "a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Strong,
                        text: "sub".to_string(),
                        tags: hset!(["sub"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Light,
                        text: " b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_supersub_c7,
        "
pre **a *^super^* b** post
pre **a *~sub~* b** post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Medium,
                        text: "a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Strong,
                        text: "super".to_string(),
                        tags: hset!(["super"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Medium,
                        text: " b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                    ParagraphItem::Text("\npre ".to_string()),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Medium,
                        text: "a ".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Strong,
                        text: "sub".to_string(),
                        tags: hset!(["sub"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Em(Emphasis{
                        strength: EmStrength::Medium,
                        text: " b".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_rule_c0,
        "
pre

---

post
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("pre".to_string()),
                    ],
                    ..Default::default()
                }),
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("\npost".to_string()),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_rule_c1,
        "
pre

***

post
        ",
        Doc {
            items: vec![
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("pre".to_string()),
                    ],
                    ..Default::default()
                }),
                DocItem::Paragraph(Paragraph {
                    items: vec![
                        ParagraphItem::Text("\npost".to_string()),
                    ],
                    ..Default::default()
                }),
            ],
            ..Default::default()
        }
    );

    test!(
        t_link_c0,
        "
pre [link *em*](url 'title') post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("link ".to_string()),
                            LinkItem::Em(Emphasis {
                                text: "em".to_string(),
                                ..Default::default()
                            }),
                        ],
                        url: "url".to_string(),
                        props: props!([("title".to_string(), PropVal::String("title".to_string()))]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_link_c1,
        "
pre <https://url> post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("https://url".to_string()),
                        ],
                        url: "https://url".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_link_c2,
        "
pre <a@b.c> post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("a@b.c".to_string()),
                        ],
                        url: "a@b.c".to_string(),
                        tags: hset!(["email-address".to_string()]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_link_c3,
        "
pre [[url]] post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("url".to_string()),
                        ],
                        url: "url".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_link_c4,
        "
pre [[url|link]] post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("link".to_string()),
                        ],
                        url: "url".to_string(),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_link_c5,
        "
pre [link][ref] post

[ref]: url 'title'
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("link".to_string()),
                        ],
                        url: "url".to_string(),
                        props: props!([
                            ("title".to_string(), PropVal::String("title".to_string())),
                            ("link-ref".to_string(), PropVal::String("ref".to_string())),
                        ]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_link_c6,
        "
pre [ref][] post

[ref]: url 'title'
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("ref".to_string()),
                        ],
                        url: "url".to_string(),
                        props: props!([
                            ("title".to_string(), PropVal::String("title".to_string())),
                            ("link-ref".to_string(), PropVal::String("ref".to_string())),
                        ]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_link_c7,
        "
pre [ref] post

[ref]: url 'title'
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("ref".to_string()),
                        ],
                        url: "url".to_string(),
                        props: props!([
                            ("title".to_string(), PropVal::String("title".to_string())),
                            ("link-ref".to_string(), PropVal::String("ref".to_string())),
                        ]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_image_c0,
        "
pre ![image *em*](url 'title') post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("image ".to_string()),
                            LinkItem::Em(Emphasis {
                                text: "em".to_string(),
                                ..Default::default()
                            }),
                        ],
                        url: "url".to_string(),
                        tags: hset!(["image"]),
                        props: props!([("title".to_string(), PropVal::String("title".to_string()))]),
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_image_c1,
        "
pre ![[url]] post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("url".to_string()),
                        ],
                        url: "url".to_string(),
                        tags: hset!(["image"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_image_c2,
        "
pre ![[url|image]] post
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("image".to_string()),
                        ],
                        url: "url".to_string(),
                        tags: hset!(["image"]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_image_c3,
        "
pre ![image][ref] post

[ref]: url 'title'
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("image".to_string()),
                        ],
                        url: "url".to_string(),
                        tags: hset!(["image"]),
                        props: props!([
                            ("title".to_string(), PropVal::String("title".to_string())),
                            ("link-ref".to_string(), PropVal::String("ref".to_string())),
                        ]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_image_c4,
        "
pre ![ref][] post

[ref]: url 'title'
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("ref".to_string()),
                        ],
                        url: "url".to_string(),
                        tags: hset!(["image"]),
                        props: props!([
                            ("title".to_string(), PropVal::String("title".to_string())),
                            ("link-ref".to_string(), PropVal::String("ref".to_string())),
                        ]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );

    test!(
        t_image_c5,
        "
pre ![ref] post

[ref]: url 'title'
        ",
        Doc {
            items: vec![DocItem::Paragraph(Paragraph {
                items: vec![
                    ParagraphItem::Text("pre ".to_string()),
                    ParagraphItem::Link(Link {
                        items: vec![
                            LinkItem::String("ref".to_string()),
                        ],
                        url: "url".to_string(),
                        tags: hset!(["image"]),
                        props: props!([
                            ("title".to_string(), PropVal::String("title".to_string())),
                            ("link-ref".to_string(), PropVal::String("ref".to_string())),
                        ]),
                        ..Default::default()
                    }),
                    ParagraphItem::Text(" post".to_string()),
                ],
                ..Default::default()
            })],
            ..Default::default()
        }
    );
}

