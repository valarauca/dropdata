
#[macro_use]
extern crate lazy_static;
use regex::Regex;
use lrlex::lrlex_mod;
use lrpar::lrpar_mod;
use serde::{Serialize,Deserialize};

lrlex_mod!("rules.l");
lrpar_mod!("rules.y");


/// DropRecord describes a single drop instance
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Hash,Serialize,Deserialize)]
pub struct DropRecord<'a> {
    #[serde(borrow)]
    pub drops: Vec<Item<'a>>,
    pub quantity: usize,
}


/// Item describes a single item within a drop
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Hash,Serialize,Deserialize)]
pub struct Item<'a> {
    #[serde(borrow)]
    pub item: &'a str,
    pub item_id: usize,
    pub quantity: usize,
}

/// Top level parse call which will process records
pub fn parse<'a>(input: &'a str) -> Result<Vec<DropRecord<'a>>,()> {
    let lexerdef = rules_l::lexerdef();
    let (arg, _) = {
        let lexer = lexerdef.lexer(input);
        rules_y::parse(&lexer)
    };
    match arg {
        Option::Some(Ok(x)) => Ok(x),
        _ => Err(())
    }
}

pub fn pretty_json(arg: &[DropRecord<'_>]) -> String {
    serde_json::to_string_pretty(arg).unwrap()
}



/*
 * Pedantic Number Handling
 *
 */

lazy_static! {
    static ref MAYBE_QUOTED_NUM: Regex = Regex::new(r#"\s*"*\s*([0-9]+)\s*"*\s*"#).unwrap();
    static ref QUOTED_STRING: Regex = Regex::new(r#"\s*"*([^"]+)"*\s*"#).unwrap();
}

pub(in crate) fn read_num(arg: &str) -> Result<usize,()> {
    MAYBE_QUOTED_NUM
        .captures(arg)
        .into_iter()
        .flat_map(|c| c.get(1))
        .flat_map(|x| usize::from_str_radix(x.as_str(),10).ok())
        .next()
        .ok_or(())
}

pub(in crate) fn read_str<'a>(arg: &'a str) -> Result<&'a str,()> {
    QUOTED_STRING
        .captures(arg)
        .into_iter()
        .flat_map(|c| c.get(1))
        .map(|c| c.as_str())
        .next()
        .ok_or(())
}

#[test]
fn test_read_num() {

    let x = read_num("15").unwrap();
    assert_eq!(x, 15usize);

    let y = read_num("5125").unwrap();
    assert_eq!(y, 5125usize);

    let z = read_num(" \"9001\" ").unwrap();
    assert_eq!(z, 9001usize);
}

