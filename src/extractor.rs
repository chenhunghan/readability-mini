use markup5ever_rcdom::RcDom;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::path::Path;

use crate::error::Error;
use crate::scorer::Candidate;
use crate::{dom, scorer};

#[derive(Debug)]
pub struct Product {
    pub title: String,
    pub text: String,
}

pub fn extract(mut dom: RcDom) -> Result<Product, Error>
{
    let mut title = String::new();
    let mut candidates = BTreeMap::new();
    let mut nodes = BTreeMap::new();
    let handle = dom.document.clone();
    scorer::preprocess(&mut dom, handle.clone(), &mut title);
    scorer::find_candidates(Path::new("/"), handle.clone(), &mut candidates, &mut nodes);
    let mut id: &str = "/";
    let mut top_candidate: &Candidate = &Candidate {
        node: handle.clone(),
        score: Cell::new(0.0),
    };
    for (i, c) in candidates.iter() {
        let score = c.score.get() * (1.0 - scorer::get_link_density(c.node.clone()));
        c.score.set(score);
        if score <= top_candidate.score.get() {
            continue;
        }
        id = i;
        top_candidate = c;
    }

    let node = top_candidate.node.clone();
    scorer::clean(&mut dom, Path::new(id), node.clone(), &candidates);


    let mut text: String = String::new();
    dom::extract_text(node.clone(), &mut text, true);
    Ok(Product {
        title,
        text,
    })
}
