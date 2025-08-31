use std::collections::HashMap;

use starlark::{environment::GlobalsBuilder, starlark_module, values::dict::DictRef};

use crate::{
    graph::NODES,
    node::{Node, json_file::JsonFile, output::Output, run::Run, template::Template},
    node_field::NodeField,
};

fn dict_to_hashmap(dict: DictRef) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    for (k, v) in dict.iter() {
        hashmap.insert(k.to_str(), v.to_str());
    }
    hashmap
}

#[starlark_module]
pub fn starlark_mod(globals: &mut GlobalsBuilder) {
    fn json_file(name: &str, file: &str, outputs: DictRef) -> starlark::Result<String> {
        let node = JsonFile {
            name: name.to_string(),
            file: file.to_string(),
            outputs: dict_to_hashmap(outputs),
        };
        NODES.with(|f| f.borrow_mut().nodes.push(Node::JsonFile(node.clone())));
        Ok(format!(":{}", node.name))
    }

    fn output(name: &str, file: &str, content: &str) -> starlark::Result<String> {
        let node = Output {
            name: name.to_string(),
            file: file.to_string(),
            content: NodeField::auto(content),
        };
        NODES.with(|f| f.borrow_mut().nodes.push(Node::Output(node.clone())));
        Ok(format!(":{}", node.name))
    }

    fn run(name: &str, target: &str) -> starlark::Result<String> {
        let node = Run {
            name: name.to_string(),
            target: NodeField::auto(target),
        };
        NODES.with(|f| f.borrow_mut().nodes.push(Node::Run(node.clone())));
        Ok(format!(":{}", node.name))
    }

    fn template(name: &str, pattern: &str, data: DictRef) -> starlark::Result<String> {
        let node = Template {
            name: name.to_string(),
            pattern: pattern.to_string(),
            data: dict_to_hashmap(data),
        };
        NODES.with(|f| f.borrow_mut().nodes.push(Node::Template(node.clone())));
        Ok(format!(":{}", node.name))
    }
}
