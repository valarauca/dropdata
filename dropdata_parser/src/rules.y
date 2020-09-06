%start TOP
%%

NUM -> Result<usize,()>:
      'QUOTED_INT' { let v = $1.map_err(|_| ())?; read_num($lexer.span_str(v.span())) }
    | 'INT' { let v = $1.map_err(|_| ())?; read_num($lexer.span_str(v.span())) };

NAME -> Result<&'input str, ()>:
      'STRING' { let v = $1.map_err(|_| ())?; read_str($lexer.span_str(v.span())) };

ITEM -> Result<Item<'input>,()>:
    'OPEN' NAME 'COMMA' NUM 'COMMA' NUM 'CLOSE' { Ok(Item { item: $2?, item_id: $4?, quantity: $6? }) };

ITEM_LIST -> Result<Vec<Item<'input>>,()>:
      ITEM_LIST 'COMMA' ITEM { let mut v = $1?; v.push($3?); Ok(v) }
    | ITEM { Ok(vec![$1?]) };

WRAPPED_ITEM_LIST -> Result<Vec<Item<'input>>,()>:
    'OPEN' ITEM_LIST 'CLOSE' { Ok($2?) };

DROP_RECORD -> Result<DropRecord<'input>,()>:
    'OPEN' WRAPPED_ITEM_LIST 'COMMA' NUM 'CLOSE' { Ok(DropRecord{ drops: $2?, quantity: $4? }) };

DROP_RECORD_LIST -> Result<Vec<DropRecord<'input>>,()>:
      DROP_RECORD_LIST 'COMMA' DROP_RECORD { let mut v = $1?; v.push($3?); Ok(v) }
    | DROP_RECORD { Ok(vec![$1?]) };

TOP -> Result<Vec<DropRecord<'input>>,()>:
    'OPEN' DROP_RECORD_LIST 'CLOSE' { $2 };
%%

use crate::{read_num, read_str, Item, DropRecord};
