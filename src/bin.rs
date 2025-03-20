use md_to_incodoc::test;
use incodoc::output::doc_out;

fn main() {
    let doc = test();
    let mut output = String::new();
    doc_out(&doc, &mut output);
    println!("{output}");
}

#[derive(Debug)]
struct A {
    l: usize,
    c: Vec<A>,
}

fn a(v: Vec<usize>) -> A {
    if v.is_empty() {
        return A { l: 0, c: vec![] };
    }
    let mut min = 999;
    for val in v.iter().skip(1) {
        if *val < min {
            min = *val;
        }
    }
    let first = v[0];
    let mut buckets = Vec::new();
    let mut bucket = Vec::new();
    for val in v.into_iter().skip(1) {
        if val == min {
            buckets.push(std::mem::take(&mut bucket));
            bucket.push(val);
        } else {
            bucket.push(val);
        }
    }
    buckets.push(std::mem::take(&mut bucket));
    let mut res = Vec::new();
    for b in buckets.into_iter() {
        if !b.is_empty() {
            res.push(a(b));
        }
    }
    A {
        l: first,
        c: res,
    }
}
