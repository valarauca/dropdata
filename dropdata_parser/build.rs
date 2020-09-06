
use std::error::Error;

use cfgrammar::yacc::YaccKind;
use lrlex::LexerBuilder;
use lrpar::CTParserBuilder;

fn main() -> Result<(),Box<dyn Error>> {

    let lex_rule_ids_map = CTParserBuilder::new()
        .yacckind(YaccKind::Grmtools)
        .process_file_in_src("rules.y")?;
    LexerBuilder::new()
        .rule_ids_map(lex_rule_ids_map)
        .process_file_in_src("rules.l")?;
    Ok(())
}
