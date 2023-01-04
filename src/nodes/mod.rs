use std::fmt::{Display, Formatter, Result as FmtResult};
use serde::{Deserialize, Serialize};
use serde_json::Result;

pub mod node;
pub use node::{Node,Type};

pub struct RgExplorer {
    nodes: Nodes,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_camel_case_types)]
struct AuxType {
    r#type: Type,
}

#[derive(Debug)]
pub struct Nodes(pub Vec<Node>);

/*
 * ESTRATEGIA:
 * Tomar data_raw
 * iterarlo
 * Identificar el tipo de data_raw node: begin, match, context, end, summary
 * Si es begin, match, context, agregarlo a un un vec auxiliar
 * Si es end, cerrar el arreglo y construir el Node con ese vector auxiliar de strings. Vaciar el
 *   vec auxiliar
 * Si summary, crear el nodo final y terminar. Agregar los nodos a node
 */
impl Nodes {
    pub fn new(data_raw: Vec<&str>) -> Self {
        let mut v: Nodes = Nodes { 0: vec![]};
        let mut aux_vecs: Vec<(&str, Type)> = vec![];
        // let mut current_node = Node();

        for d in data_raw {
            let t = Self::parse_type(d).expect("Error parsing type at first level. Expected begin, match, end, context or summary");
            match t.r#type {
                Type::begin | Type::r#match | Type::summary => {
                    aux_vecs.push((d, t.r#type))
                },
                Type::end => {
                    aux_vecs.push((d, t.r#type));
                    let n: Node = Node::new(aux_vecs);
                    v.0.push(n);
                    aux_vecs = vec![];
                }
                _ => {}
            }
            /*
            match n {
                // Ok(val) => v.0.push(val),
                Ok(val) => {
                    match val {
                    }
                },
                Err(e) => println!("{e:?}\n{}", d),
            }
            */
        }
        v
    }

    pub fn len(&self) -> usize {
        let Nodes(foo) = self;
        foo.len()
    }

    fn parse_type(d: &str) -> Result<AuxType> {
        let n: AuxType = serde_json::from_str(d)?;
        Ok(n)
    }
}

impl RgExplorer {
    pub fn new(data_raw: Vec<&str>) -> Self {
        let mut v: Nodes = Nodes { 0: vec![]};

        for d in data_raw {
            let n = Self::parse_data(d);

            match n {
                Ok(val) => v.0.push(val),
                Err(e) => println!("{e:?}\n{}", d),
            }
        }
        Self {
            nodes: v,
        }
    }
    fn parse_data(d: &str) -> Result<Node> {
        let n: Node = serde_json::from_str(d)?;
        Ok(n)
    }
}

impl Display for RgExplorer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Rg Explorer: {:?}\n\n====================\n\n{}", self.nodes, self.nodes)
    }
}

impl Display for Nodes {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.iter().fold(Ok(()), |result, node| {
            result.and_then(|_| writeln!(f, "{}", node))
        })
    }
}

