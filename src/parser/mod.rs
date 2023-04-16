use pest::Parser as _;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct MyParser;

pub fn compactor(input: &str) -> Result<String, ()> {
    let root = MyParser::parse(Rule::root, input)
        .map_err(|e| eprintln!("{e}"))?
        .next()
        .unwrap();
    let item_list = root.into_inner().next().unwrap();
    // Once you descend through the tree, down to the list of items, you can do a plain `concat`
    // and all the whitespace will be dropped (since it isn't a part of each item).
    // Higher up in the structure, we'd see whitespace captured for some reason.
    Ok(item_list.into_inner().concat())
}

#[cfg(test)]
mod tests;
