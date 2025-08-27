use clap::Parser;
use reda::{graph::NODES, module::starlark_mod, node::Node};
use starlark::{
    environment::{GlobalsBuilder, Module},
    eval::Evaluator,
    syntax::{AstModule, Dialect},
};
use std::path::PathBuf;

/// Reda program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Reda file
    fichier: PathBuf,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    // Check the program file exists
    if !args.fichier.exists() {
        eprintln!("Error : File '{}' does not exist.", args.fichier.display());
        std::process::exit(1);
    }

    let ast = match AstModule::parse_file(&args.fichier, &Dialect::Standard) {
        Ok(ast) => ast,
        Err(e) => return Err(format!("Failed to create AST: {}", e)),
    };

    let module = Module::new();
    let globals = GlobalsBuilder::standard().with(starlark_mod).build();
    let mut eval = Evaluator::new(&module);

    match eval.eval_module(ast, &globals) {
        Ok(_) => (),
        Err(e) => return Err(format!("Failed to create AST: {}", e)),
    };

    let graph = NODES.with(|f| f.borrow().clone());

    let run_nodes: Vec<Node> = graph
        .nodes
        .iter()
        .filter(|n| match n {
            Node::Run(_) => true,
            _ => false,
        })
        .map(|n| n.clone())
        .collect::<Vec<Node>>();

    for node in run_nodes {
        println!("Executing node: {}", node.clone().get_name());
        node.execute(&graph)?;
    }

    Ok(())
}
