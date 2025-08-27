use std::collections::HashMap;

use starlark::{environment::GlobalsBuilder, starlark_module, values::dict::DictRef};

use crate::{
    graph::NODES,
    node::{Node, output::Output, run::Run, template::Template},
    node_field::NodeField,
};

#[starlark_module]
pub fn starlark_mod(globals: &mut GlobalsBuilder) {
    fn template(name: &str, pattern: &str, data: DictRef) -> starlark::Result<String> {
        let mut data_dict: HashMap<String, String> = HashMap::new();

        for (k, v) in data.iter() {
            data_dict.insert(k.to_str(), v.to_str());
        }

        let node = Template {
            name: name.to_string(),
            pattern: pattern.to_string(),
            data: data_dict,
        };
        NODES.with(|f| f.borrow_mut().nodes.push(Node::Template(node.clone())));
        Ok(format!(":{}", node.name))
    }

    fn output(name: &str, file: &str, content: &str) -> starlark::Result<String> {
        let node = Output {
            name: name.to_string(),
            file: file.to_string(),
            content: Box::new(NodeField::auto(content)),
        };
        NODES.with(|f| f.borrow_mut().nodes.push(Node::Output(node.clone())));
        Ok(format!(":{}", node.name))
    }

    fn run(name: &str, target: &str) -> starlark::Result<String> {
        let node = Run {
            name: name.to_string(),
            target: Box::new(NodeField::auto(target)),
        };
        NODES.with(|f| f.borrow_mut().nodes.push(Node::Run(node.clone())));
        Ok(format!(":{}", node.name))
    }
}
