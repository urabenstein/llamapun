//! offers the function to get all declarations in a document
extern crate senna;
extern crate libxml;

use patterns::*;
use data::Corpus;
use senna::senna::SennaParseOptions;
use dnm::*;
use libxml::xpath::Context;
use libxml::tree::*;
use std::rc::Rc;
use std::collections::HashMap;
use std::cell::Cell;
use std::path::PathBuf;
use std::fs;


/// turns a marker into a readable string representation
fn get_pattern_marker_string(marker : &PatternMarker) -> String {
    let mut result = String::new();
    result.push_str(&marker.name);
    result.push_str(" {");
    let mut first = true;
    for tag in &marker.tags {
        if !first {
            result.push_str(", ");
        }
        first = false;
        result.push('\'');
        result.push_str(tag);
        result.push('\'');
    }
    result.push('}');
    result
}


/// turns a math node into a readable string representation
fn math_node_to_string(node : &Node) -> String {
    let mut s = String::new();
    math_node_to_string_actual(node, &mut s);
    s
}

/// helper function
fn math_node_to_string_actual(node : &Node, mut string : &mut String) {
    match node.get_name().as_ref() {
        "semantics" => math_node_to_string_children(node, &mut string),
        "annotation" => { },
        "annotation-xml" => { },
        "text" => {
            if node.is_text_node() {
                string.push_str(&node.get_content());
            }
        }
        default => {
            string.push('<');
            string.push_str(default);
            string.push('>');
            math_node_to_string_children(node, &mut string);
            string.push('<');
            string.push('/');
            string.push_str(default);
            string.push('>');
        }
    }
}

/// helper function
fn math_node_to_string_children(node : &Node, mut string : &mut String) {
    let mut cur = node.get_first_child();
    loop {
        if cur.is_none() { break; }
        math_node_to_string_actual(cur.as_ref().unwrap(), &mut string);
        cur = cur.unwrap().get_next_sibling();
    }
}


/// prints a marker in a human readable way
fn print_marker(marker : &MarkerEnum, alt_dnm : &DNM, xpath_context : &Context) -> Option<String> {
    match marker {
        &MarkerEnum::Text(_) => {}
        &MarkerEnum::Math(ref math_marker) => {
            if math_marker.marker.name == "identifier" {
                let output = format!("{}", DNMRange::serialize_node(&alt_dnm.root_node, &math_marker.node, false));
                return Some(output);
            }
        }
    }

    return None;
}

/// gets a DNM that is more readable for printing
fn get_alternative_dnm(root: &Node) -> DNM {
    let mut name_options = HashMap::new();
    name_options.insert("math".to_string(),
                        SpecialTagsOption::FunctionNormalize(Rc::new(math_node_to_string)));
    name_options.insert("cite".to_string(),
                        SpecialTagsOption::Normalize("CitationElement".to_string()));
    name_options.insert("table".to_string(), SpecialTagsOption::Skip);
    name_options.insert("head".to_string(), SpecialTagsOption::Skip);

    let mut class_options = HashMap::new();
    class_options.insert("ltx_equation".to_string(),
                         SpecialTagsOption::FunctionNormalize(Rc::new(math_node_to_string)));
    class_options.insert("ltx_equationgroup".to_string(),
                         SpecialTagsOption::FunctionNormalize(Rc::new(math_node_to_string)));
    class_options.insert("ltx_note_mark".to_string(), SpecialTagsOption::Skip);
    class_options.insert("ltx_note_outer".to_string(), SpecialTagsOption::Skip);
    class_options.insert("ltx_bibliography".to_string(), SpecialTagsOption::Skip);

    let parameters = DNMParameters {
        special_tag_name_options: name_options,
        special_tag_class_options: class_options,
        normalize_white_spaces: true,
        wrap_tokens: false,
        normalize_unicode: false,
        ..Default::default()
    };

    DNM::new(root.clone(), parameters)
}

///returns a vec with the xpaths to the found declarations
pub fn get_declarations(file_name : String) -> Vec<String> {

    let dir = PathBuf::from("declaration_pattern.xml");
    println!("{:?}", fs::canonicalize(&dir));
    
    let pattern_file_result = PatternFile::load("declaration_pattern.xml");
    // let pattern_file_result = PatternFile::load("examples/ulrich/units_pattern.xml");
    let pattern_file = match pattern_file_result {
        Err(x) => panic!(x),
        Ok(x) => x,
    };

    let mut corpus = Corpus::new(file_name.clone());
    corpus.senna_options = Cell::new( SennaParseOptions { pos : true, psg : true } );
    corpus.dnm_parameters.support_back_mapping = true;

    let mut document = corpus.load_doc(file_name).unwrap();
    // let mut document = corpus.load_doc("examples/ulrich/physics9807021.html".to_string()).unwrap();
    // let mut document = corpus.load_doc("tests/resources/0903.1000.html".to_string()).unwrap();


    // get a more readable DNM for printing
    let alt_dnm = get_alternative_dnm(&document.dom.get_root_element());

    let mut xpath_vec = Vec::new();

    for mut sentence in document.sentence_iter() {
        let sentence_2 = sentence.senna_parse();
        let matches = match_sentence(&pattern_file, &sentence_2.senna_sentence.as_ref().unwrap(),
                                     &sentence_2.range, "declaration").unwrap();
        if matches.len() > 0 {
            let xpath_context = Context::new(&sentence_2.document.dom).unwrap();
            for m in &matches {
                for m2 in &m.get_marker_list() {
                    let opt_xpath = print_marker(m2, &alt_dnm, &xpath_context);
                    if opt_xpath.is_some(){
                      xpath_vec.push(opt_xpath.unwrap());
                    }
                }
            }
            
        }
    }

    return xpath_vec;
    
}
