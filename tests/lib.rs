use std::fs::File;

use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::RcDom;

#[test]
fn test_extract_title() {
    let mut file = File::open("./data/title.html").unwrap();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut file).unwrap();
    let product = readability_mini::extractor::extract(dom).unwrap();
    assert_eq!(product.title, "This is title");
}

#[test]
fn test_fix_rel_links() {
    let mut file = File::open("./data/rel.html").unwrap();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut file).unwrap();
    let product = readability_mini::extractor::extract(dom).unwrap();
    assert_eq!(product.text, "This is title poop ");
}

#[test]
fn test_fix_img_links() {
    let mut file = File::open("./data/img.html").unwrap();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut file).unwrap();
    let product = readability_mini::extractor::extract(dom).unwrap();
    assert_eq!(product.text, "This is title");
}
