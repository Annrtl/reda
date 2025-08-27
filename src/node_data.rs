#[derive(Debug, Clone)]
pub enum NodeData {
    None(),
    String(String),
}

// impl NodeData {
//     fn auto(name: &str) -> NodeData {
//         if name.starts_with(":") {
//             return NodeData::None();
//         } else {
//             return NodeData::String(name.to_string());
//         }
//     }
// }
