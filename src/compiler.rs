// # Mech Syntax Compiler

// ## Preamble

use mech_core::*;
use mech_core::nodes::*;
use mech_core::function::table::*;
use mech_core::function::matrix::*;

use crate::ast::Ast;
use crate::parser::{parse, parse_fragment};
//use super::formatter::Formatter;

#[cfg(not(feature = "no-std"))] use core::fmt;
#[cfg(feature = "no-std")] use alloc::fmt;
#[cfg(feature = "no-std")] use alloc::string::String;
#[cfg(feature = "no-std")] use alloc::vec::Vec;
use hashbrown::hash_set::{HashSet};
use hashbrown::hash_map::{HashMap};
use std::sync::Arc;
use std::mem;

fn get_sections(nodes: &Vec<AstNode>) -> Vec<Vec<AstNode>> {
  let mut sections: Vec<Vec<AstNode>> = Vec::new();
  let mut statements = Vec::new();
  let mut blocks = vec![];
  for n in nodes {
    match n {
      AstNode::Section{children,..} => {
        for child in children {
          match child {
            AstNode::Block{children, ..} => {
              blocks.push(child.clone());
            }
            AstNode::UserFunction{children} => {
              blocks.push(child.clone());
            }
            AstNode::Statement{children, ..} => {
              statements.append(&mut children.clone());
            }
            _ => (),
          }
        }
        sections.push(blocks.clone());
        blocks.clear();
      },
      AstNode::Root{children} |
      AstNode::Body{children} |
      AstNode::Program{children,..} |
      AstNode::Fragment{children} => {
        sections.append(&mut get_sections(children));
      }
      _ => (), 
    }
  }
  if statements.len() > 0 {
    sections.push(vec![AstNode::Block{children: statements, src_range: SourceRange::default()}]);
  }
  sections
}

fn get_blocks(nodes: &Vec<AstNode>) -> Vec<AstNode> {
  let mut blocks = Vec::new();
  let mut statements = Vec::new();
  for n in nodes {
    match n {
      AstNode::Block{..} => blocks.push(n.clone()),
      AstNode::MechCodeBlock{children} => {
        // Do something with the block state string.
        // ```mech: disabled
        match &children[0] {
          AstNode::String{text, ..} => {
            let block_state = text.iter().collect::<String>();
            if block_state != "disabled".to_string() {
              blocks.append(&mut get_blocks(children));
            }
          }
          _ => (),
        }
      }
      _ => (), 
    }
  }
  if statements.len() > 0 {
    blocks.push(AstNode::Block{children: statements, src_range: SourceRange::default()});
  }
  blocks
}

pub struct Compiler {

}

impl Compiler {

  pub fn new() -> Compiler {
    Compiler{}
  }

  pub fn compile_str(&mut self, code: &str) -> Result<Vec<Vec<SectionElement>>,MechError> {
    let parse_tree = parse(code)?;
    let mut ast = Ast::new();
    ast.build_syntax_tree(&parse_tree);
    let mut compiler = Compiler::new();
    compiler.compile_sections(&vec![ast.syntax_tree.clone()])
  }

  pub fn compile_fragment(&mut self, code: &str) -> Result<Vec<Vec<SectionElement>>,MechError> {
    let parse_tree = parse_fragment(code)?;
    let mut ast = Ast::new();
    ast.build_syntax_tree(&parse_tree);
    let mut compiler = Compiler::new();
    compiler.compile_sections(&vec![ast.syntax_tree.clone()])
  }

  pub fn compile_fragment_from_parse_tree(&mut self, parse_tree: ParserNode) -> Result<Vec<Vec<SectionElement>>,MechError> {
    let mut ast = Ast::new();
    ast.build_syntax_tree(&parse_tree);
    let mut compiler = Compiler::new();
    compiler.compile_sections(&vec![ast.syntax_tree.clone()])
  }

  pub fn compile_sections(&mut self, nodes: &Vec<AstNode>) -> Result<Vec<Vec<SectionElement>>,MechError> {
    let mut sections: Vec<Vec<SectionElement>> = Vec::new();
    for section in get_sections(nodes) {
      let mut elements: Vec<SectionElement> = Vec::new();
      for node in section {
        match node {
          AstNode::Block{..} => {
            let mut block = Block::new();
            let mut tfms = self.compile_node(&node)?;
            let tfms_before = tfms.clone();
            tfms.sort();
            tfms.dedup();
            for tfm in tfms {
              block.add_tfm(tfm);
            }
            block.ast = node.clone();
            elements.push(SectionElement::Block(block));
          }
          AstNode::UserFunction{children} => {
            let mut user_function = UserFunction::new();


            let mut out_args = self.compile_node(&children[0])?;
            let mut name = self.compile_node(&children[1])?;
            let mut in_args = self.compile_node(&children[2])?;
            let mut body = self.compile_node(&children[3])?;

            // TODO Check that all arguments are used
            // TODO Check that output arguments are created in body
            // TODO Check that all variables used in body are sourced in body or inargs

            match &name[0] {
              Transformation::Identifier{name,id} => {
                user_function.name = *id;
              }
              _ => (),
            }

            let mut i = 0;
            while i < out_args.len() {
              match (&out_args[i],&out_args[i+1]) {
                (Transformation::Identifier{name,id},Transformation::ColumnKind{table_id,column_ix,kind}) => {
                  user_function.outputs.insert(*id,ValueKind::F32);
                }
                _ => (),
              }
              i += 3;
            }

            let mut i = 0;
            while i < in_args.len() {
              match (&in_args[i],&in_args[i+1]) {
                (Transformation::Identifier{name,id},Transformation::ColumnKind{table_id,column_ix,kind}) => {
                  user_function.inputs.insert(*id,ValueKind::F32);
                }
                _ => (),
              }
              i += 3;
            }

            user_function.transformations = body;
            elements.push(SectionElement::UserFunction(user_function));
          }
          _ => (),
        }
      }
      sections.push(elements);
    }
    if sections.len() > 0 {
      Ok(sections)
    } else {
      Err(MechError{msg: "".to_string(), id: 3749, kind: MechErrorKind::None})
    }
  }

  pub fn compile_nodes(&mut self, nodes: &Vec<AstNode>) -> Result<Vec<Transformation>,MechError> {
    let mut compiled = Vec::new();
    for node in nodes {
      let mut result = self.compile_node(node)?;
      compiled.append(&mut result);
    }
    Ok(compiled)
  }

  pub fn compile_node(&mut self, node: &AstNode) -> Result<Vec<Transformation>,MechError> {
    let mut tfms = vec![];
    match node {
      AstNode::Identifier{name, id, ..} => {
        tfms.push(Transformation::Identifier{name: name.to_vec(), id: *id});
      },
      AstNode::Empty => {
        let table_id = TableId::Local(hash_str("_"));
        tfms.push(Transformation::NewTable{table_id: table_id.clone(), rows: 1, columns: 1 });
        tfms.push(Transformation::Constant{table_id: table_id, value: Value::Empty});
      },
      AstNode::True => {
        let table_id = TableId::Local(hash_str("true"));
        tfms.push(Transformation::NewTable{table_id: table_id.clone(), rows: 1, columns: 1 });
        tfms.push(Transformation::Constant{table_id: table_id, value: Value::Bool(true)});
      },
      AstNode::False => {
        let table_id = TableId::Local(hash_str("false"));
        tfms.push(Transformation::NewTable{table_id: table_id.clone(), rows: 1, columns: 1 });
        tfms.push(Transformation::Constant{table_id: table_id, value: Value::Bool(false)});
      },
      AstNode::String{text, ..} => {
        let table_id = TableId::Local(hash_str(&format!("string: {:?}", text)));
        tfms.push(Transformation::NewTable{table_id: table_id.clone(), rows: 1, columns: 1 });
        tfms.push(Transformation::Constant{table_id: table_id, value: Value::String(MechString::from_chars(text))});
      },
      AstNode::NumberLiteral{kind, bytes, ..} => {
        let string = bytes.iter().cloned().collect::<String>();
        let bytes = if *kind == *cU8 { string.parse::<u8>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cU16 { string.parse::<u16>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cU32 { string.parse::<u32>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cU64 { string.parse::<u64>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cU128 { string.parse::<u128>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cI8 { string.parse::<i8>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cI16 { string.parse::<i16>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cI32 { string.parse::<i32>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cI64 { string.parse::<i64>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cI128 { string.parse::<i128>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cF32 { string.parse::<f32>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cF64 { string.parse::<f64>().unwrap().to_be_bytes().to_vec() }
          else if *kind == *cHEX {
            bytes.iter().map(|c| c.to_digit(16).unwrap() as u8).collect::<Vec<u8>>()
          }
          else { string.parse::<f32>().unwrap().to_be_bytes().to_vec() };
        let table_id = TableId::Local(hash_str(&format!("{:?}{:?}", kind, bytes)));
        tfms.push(Transformation::NewTable{table_id: table_id, rows: 1, columns: 1 });
        tfms.push(Transformation::NumberLiteral{kind: *kind, bytes: bytes.to_vec()});
      },
      AstNode::Table{name, id, ..} => {
        tfms.push(Transformation::NewTable{table_id: TableId::Global(*id), rows: 1, columns: 1});
        tfms.push(Transformation::Identifier{name: name.clone(), id: *id});
      }
      // dest := src
      // dest{ix} := src
      AstNode::SetData{children} => {
        let mut src = self.compile_node(&children[1])?;
        let mut dest = self.compile_node(&children[0])?.clone();

        let (src_table_id, src_indices) = match &mut src[0] {
          Transformation::NewTable{table_id,..} => {
            Some((table_id.clone(),vec![(TableIndex::All, TableIndex::All)]))
          },
          Transformation::Select{table_id,ref indices} => {
            let table_id = table_id.clone();
            let indices = indices.clone();
            src.remove(0);
            Some((table_id,indices))
          },
          Transformation::TableReference{table_id, reference: Value::Reference(id)} => {
            let table_id = id.clone();
            src.remove(0);
            src.remove(0);
            src.remove(0);
            Some((table_id.clone(),vec![(TableIndex::All, TableIndex::All)]))
          },
          _ => None,
        }.unwrap();     
        let mut first = dest[0].clone();
        match first {
          Transformation::Select{table_id, indices} => {
            let dest_id = table_id.clone();
            let (dest_row, dest_col) = &indices[0];
            dest.remove(0);
            let (src_row,src_col) = &src_indices[0];
            tfms.push(Transformation::Set{
              src_id: src_table_id, 
              src_row: src_row.clone(), 
              src_col: src_col.clone(),
              dest_id, 
              dest_row: dest_row.clone(), 
              dest_col: dest_col.clone()
            });
          }
          _ => (),
        }

        tfms.append(&mut dest);
        tfms.append(&mut src);
      }
      // dest :+= src
      // dest{ix} :+= src
      AstNode::UpdateData{name, children, ..} => {
        let mut src = self.compile_node(&children[1])?;
        let mut dest = self.compile_node(&children[0])?.clone();

        let (src_table_id, src_indices) = match &mut src[0] {
          Transformation::NewTable{table_id,..} => {
            Some((table_id.clone(),vec![(TableIndex::All, TableIndex::All)]))
          },
          Transformation::Select{table_id,ref indices} => {
            let table_id = table_id.clone();
            let indices = indices.clone();
            src.remove(0);
            Some((table_id,indices))
          },
          Transformation::TableReference{table_id, reference: Value::Reference(id)} => {
            let table_id = id.clone();
            src.remove(0);
            src.remove(0);
            src.remove(0);
            Some((table_id.clone(),vec![(TableIndex::All, TableIndex::All)]))
          },
          _ => None,
        }.unwrap();     
        let mut first = dest[0].clone();
        match first {
          Transformation::Select{table_id, indices} => {
            let dest_id = table_id.clone();
            let (dest_row, dest_col) = &indices[0];
            dest.remove(0);
            let (src_row,src_col) = &src_indices[0];
            let name_hash = hash_chars(name);
            tfms.push(Transformation::UpdateData{
              name: name_hash,
              src_id: src_table_id, 
              src_row: src_row.clone(), 
              src_col: src_col.clone(),
              dest_id, 
              dest_row: dest_row.clone(), 
              dest_col: dest_col.clone()
            });
          }
          _ => (),
        }
        tfms.append(&mut dest);
        tfms.append(&mut src);
      }
      AstNode::TableDefine{children} => {
        let mut output = self.compile_node(&children[0])?;
        // Get the output table id
        let output_table_id = match &output[0] {
          Transformation::NewTable{table_id, ..} => {
            Some(table_id.clone())
          },
          _ => None,
        }.unwrap();

        tfms.append(&mut output);
        let mut input = self.compile_node(&children[1])?;
        let mut rhs = vec![];
        if input.len() > 0 {
          loop { 
            match &mut input[0] {
              Transformation::NewTable{table_id,..} => {
                rhs.push((table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
                break;
              },
              Transformation::Select{table_id,ref indices} => {
                let table_id = table_id.clone();
                let indices = indices.clone();
                input.remove(0);
                rhs.push((table_id,indices));
                break;
              },
              Transformation::TableReference{table_id, reference: Value::Reference(id)} => {
                input.remove(0);
                input.remove(0);
                if let Transformation::Select{table_id,indices} = input.remove(0) {
                  rhs.push((table_id,indices));
                }
                continue;
              },
              _ => break,
            }
          }
          let (input_table_id, input_indices) = &rhs[0];
          if *input_table_id != output_table_id {
            tfms.push(Transformation::NewTable{table_id: output_table_id, rows: 1, columns: 1});
            tfms.append(&mut input);
            tfms.push(Transformation::TableDefine{
              table_id: input_table_id.clone(), 
              indices: input_indices.clone(), 
              out: output_table_id
            });
          } else {
            tfms.append(&mut input);
          }
        }
      }
      AstNode::TableSelect{children} => {
        let output_table_id = TableId::Local(hash_str(&format!("{:?}", children)));

        let mut input = self.compile_node(&children[0])?;
        let mut rhs = vec![];
        if input.len() > 0 {
          loop { 
            match &mut input[0] {
              Transformation::NewTable{table_id,..} => {
                rhs.push((table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
                break;
              },
              Transformation::Select{table_id,ref indices} => {
                let table_id = table_id.clone();
                let indices = indices.clone();
                input.remove(0);
                rhs.push((table_id,indices));
                break;
              },
              Transformation::TableReference{table_id, reference: Value::Reference(id)} => {
                input.remove(0);
                input.remove(0);
                if let Transformation::Select{table_id,indices} = input.remove(0) {
                  rhs.push((table_id,indices));
                }
                continue;
              },
              _ => break,
            }
          }
          let (input_table_id, input_indices) = &rhs[0];
          
          match input_indices[0] {
            (TableIndex::All,TableIndex::All) => {
              tfms.push(Transformation::TableAlias{table_id: *input_table_id, alias: *output_table_id.unwrap()});
              tfms.append(&mut input);
            }
            _ => {
              if *input_table_id != output_table_id {
                tfms.push(Transformation::NewTable{table_id: output_table_id, rows: 1, columns: 1});
                tfms.append(&mut input);
                tfms.push(Transformation::TableDefine{
                  table_id: input_table_id.clone(), 
                  indices: input_indices.clone(), 
                  out: output_table_id
                });
              }
            }
          }
        }
      }
      AstNode::VariableDefine{children} => {
        let mut output = self.compile_node(&children[0])?;
        // Get the output table id
        let output_table_id = match &output[0] {
          Transformation::Identifier{name, id} => {
            let name_hash = hash_chars(name);
            Some(TableId::Local(name_hash))
          },
          _ => None,
        }.unwrap();

        tfms.append(&mut output);
        let mut input = self.compile_node(&children[1])?;
        let mut rhs = vec![];
        if input.len() > 0 {
          loop { 
            match &mut input[0] {
              Transformation::NewTable{table_id,..} => {
                rhs.push((table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
                break;
              },
              Transformation::Select{table_id,ref indices} => {
                let table_id = table_id.clone();
                let indices = indices.clone();
                input.remove(0);
                rhs.push((table_id,indices));
                break;
              },
              Transformation::TableReference{table_id, reference: Value::Reference(id)} => {
                input.remove(0);
                input.remove(0);
                if let Transformation::Select{table_id,indices} = input.remove(0) {
                  rhs.push((table_id,indices));
                }
                continue;
              },
              _ => break,
            }
          }
          let (input_table_id, input_indices) = &rhs[0];
          match input_indices[0] {
            (TableIndex::All,TableIndex::All) => {
              tfms.push(Transformation::TableAlias{table_id: *input_table_id, alias: *output_table_id.unwrap()});
              tfms.append(&mut input);
            }
            _ => {
              if *input_table_id != output_table_id {
                tfms.push(Transformation::NewTable{table_id: output_table_id, rows: 1, columns: 1});
                tfms.append(&mut input);
                /*tfms.push(Transformation::Function{
                  name: *TABLE_DEFINE,
                  arguments: vec![(0,input_table_id.clone(),input_indices.clone())],
                  out: (output_table_id, TableIndex::All, TableIndex::All),
                });*/
                tfms.push(Transformation::TableDefine{
                  table_id: input_table_id.clone(), 
                  indices: input_indices.clone(), 
                  out: output_table_id
                });
              }
            }
          }
        }
      }
      AstNode::UserFunction{children} => {
        let output_args = &children[0];
        let function_name = &children[1];
        let input_args = &children[2];
        if let AstNode::FunctionBody{children} = &children[3] {
          let mut result = self.compile_nodes(children)?;
          tfms.append(&mut result);
        };
        println!("{:?}", tfms);
      }
      AstNode::Function{name, children, ..} => {
        let mut args: Vec<Argument>  = vec![];
        let mut arg_tfms = vec![];
        let mut identifiers = vec![];
        for child in children {
          // get the argument identifier off the function binding. Default to 0 if there is no named arg
          let mut result = self.compile_node(&child)?;
          let arg: u64 = match &result[0] {
            Transformation::Identifier{name, id} => {
              let arg_id = id.clone();
              result.remove(0);
              arg_id
            },
            _ => 0,
          };
          match &result[0] {
            Transformation::NewTable{table_id,..} => {
              args.push((arg, *table_id, vec![(TableIndex::All, TableIndex::All)]));
            },
            Transformation::Select{table_id, indices} => {
              args.push((arg, *table_id, indices.to_vec()));
              result.remove(0);
            }
            Transformation::TableReference{table_id, reference: Value::Reference(id)} => {
              let table_id = id.clone();
              result.remove(0);
              result.remove(0);
              result.remove(0);
              args.push((arg, table_id, vec![(TableIndex::All, TableIndex::All)]));
            },
            _ => (),
          }
          let mut string_identifiers = result.drain_filter(|x| if let Transformation::Identifier{..} = x {true} else {false}).collect::<Vec<Transformation>>();
          identifiers.append(&mut string_identifiers);
          arg_tfms.append(&mut result);
        }
        let name_hash = hash_chars(name);
        identifiers.push(Transformation::Identifier{name: name.clone(), id: name_hash});
        let id = hash_str(&format!("{:?}{:?}", name, args));
        tfms.push(Transformation::NewTable{table_id: TableId::Local(id), rows: 1, columns: 1});
        tfms.append(&mut arg_tfms);
        tfms.push(Transformation::Function{
          name: name_hash,
          arguments: args,
          out: (TableId::Local(id), TableIndex::All, TableIndex::All),
        });
        tfms.append(&mut identifiers);
      },
      AstNode::InlineTable{children} => {
        let columns = children.len();
        let mut table_row_children = vec![];
        let mut aliases = vec![];
        let mut kinds = vec![];
        let mut identifiers = vec![];
        // Compile bindings
        for (ix, binding) in children.iter().enumerate() {
          match binding {
            AstNode::Binding{children} => {
              let mut identifier = self.compile_node(&children[0])?;
              match &identifier[0] {
                Transformation::Identifier{name,id} => {
                  let column_alias = id.clone();
                  let column_ix = ix.clone();
                  let alias_tfm = move |x| Transformation::ColumnAlias{table_id: x, column_ix, column_alias};
                  identifiers.push(identifier[0].clone());
                  aliases.push(alias_tfm);
                }
                _ => (),
              }
              table_row_children.push(children[1].clone());
              if children.len() == 3 {
                let mut kind = self.compile_node(&children[2])?;
                match &kind[0] {
                  Transformation::ColumnKind{table_id,column_ix,kind} => {
                    let column_ix = ix.clone();
                    let kind = kind.clone();
                    let kind_tfm = move |x| Transformation::ColumnKind{table_id: x, column_ix, kind};
                    kinds.push(kind_tfm);
                  }
                  _ => (),
                }                
              }
            }
            _ => (),
          }
        }
        let table_row = AstNode::TableRow{children: table_row_children};
        let mut compiled_row_tfms = self.compile_node(&table_row)?;
        let mut a_tfms = vec![];
        loop {
          match &compiled_row_tfms[0] {
            Transformation::NewTable{table_id,..} => {
              let mut alias_tfms = aliases.iter().map(|a| a(*table_id)).collect();
              let mut kind_tfms = kinds.iter().map(|a| a(*table_id)).collect();
              a_tfms.append(&mut alias_tfms);
              a_tfms.append(&mut kind_tfms);
              break;
            }
            Transformation::TableReference{..} => {
              compiled_row_tfms.remove(0);
              compiled_row_tfms.remove(0);
              compiled_row_tfms.remove(0);
            },
            _ => break,
          }
        }
        tfms.append(&mut compiled_row_tfms);
        tfms.append(&mut a_tfms);
        match &tfms[0] {
          Transformation::NewTable{table_id,..} |
          Transformation::Select{table_id, ..} => {
            let reference_table_id = TableId::Local(hash_str(&format!("reference:{:?}", tfms[0])));
            let value = Value::Reference(table_id.clone());
            let out = TableId::Global(*table_id.unwrap());
            let in_t = table_id.clone();
            tfms.insert(0,Transformation::NewTable{table_id: reference_table_id, rows: 1, columns: 1});
            /*tfms.insert(0,Transformation::Function{
              name: *TABLE_DEFINE,
              arguments: vec![(0,in_t,vec![(TableIndex::All, TableIndex::All)])],
              out: (out,TableIndex::All, TableIndex::All),
            });*/
            tfms.insert(0,Transformation::TableDefine{table_id: in_t, indices: vec![(TableIndex::All, TableIndex::All)], out});
            tfms.insert(0,Transformation::TableReference{table_id: reference_table_id, reference: value});
          }
          _ => (),
        }  
        tfms.append(&mut identifiers);
      }
      AstNode::EmptyTable{children} => {
        let anon_table_id = hash_str(&format!("anonymous-table: {:?}",children));
        let mut table_children = children.clone();
        let mut column_aliases = Vec::new();
        let mut header_tfms = Vec::new();
        let mut columns = 1;
        match table_children.first() {
          Some(AstNode::TableHeader{children}) => {
            let mut ix = 0;
            for child in children {
              let mut result = self.compile_node(child)?;
              columns = result.len();
              // Get the column ID
              match &result[0] {
                Transformation::Identifier{name,id} => {
                  let alias_tfm = Transformation::ColumnAlias{table_id: TableId::Local(anon_table_id), column_ix: ix, column_alias: id.clone()};
                  column_aliases.push(alias_tfm);
                  column_aliases.push(result[0].clone());
                  ix+=1;
                  result.remove(0);
                }
                _ => (),
              }
              // Process the optional kind annotation
              if result.len() > 0 {
                match &result[0] {
                  Transformation::ColumnKind{table_id,column_ix,kind} => {
                    let kind_tfm = Transformation::ColumnKind{table_id: TableId::Local(anon_table_id), column_ix: ix - 1, kind: *kind};
                    result.remove(0);
                    column_aliases.append(&mut result);
                    column_aliases.push(kind_tfm);
                  }
                  _ => (),
                }
              }
            }
            header_tfms.append(&mut column_aliases);
            table_children.remove(0);
          }
          _ => (),
        };
        header_tfms.insert(0,Transformation::NewTable{table_id: TableId::Local(anon_table_id), rows: 1, columns: 1});
        tfms.append(&mut header_tfms);
      }
      AstNode::KindAnnotation{children} => {
        let mut result = self.compile_nodes(&children)?;
        for (ix,tfm) in result.iter().enumerate() {
          match tfm {
            Transformation::Identifier{name,id} => {
              let alias_tfm = Transformation::ColumnKind{table_id: TableId::Local(0), column_ix: ix.clone(), kind: id.clone()};
              tfms.push(alias_tfm);
              tfms.push(tfm.clone());
            }
            _ => (),
          }
        }        
      }
      AstNode::Token{token, chars, ..} => {
        tfms.push(Transformation::Identifier{name: chars.to_vec(), id: hash_chars(chars)});
      }
      AstNode::AnonymousTableDefine{children} => {
        let anon_table_id = hash_str(&format!("anonymous-table: {:?}",children));
        let mut table_children = children.clone();
        let mut header_tfms = Vec::new();
        let mut column_aliases = Vec::new();
        let mut body_tfms = Vec::new();
        let mut columns = 0;
        match table_children.first() {
          Some(AstNode::TableHeader{children}) => {
            let mut ix = 0;
            for child in children {
              let mut result = self.compile_node(child)?;
              columns = result.len();
              // Get the column ID
              match &result[0] {
                Transformation::Identifier{name,id} => {
                  let alias_tfm = Transformation::ColumnAlias{table_id: TableId::Local(anon_table_id), column_ix: ix, column_alias: id.clone()};
                  column_aliases.push(alias_tfm);
                  column_aliases.push(result[0].clone());
                  ix+=1;
                  result.remove(0);
                }
                _ => (),
              }
              // Process the optional kind annotation
              if result.len() > 0 {
                match &result[0] {
                  Transformation::ColumnKind{table_id,column_ix,kind} => {
                    let kind_tfm = Transformation::ColumnKind{table_id: TableId::Local(anon_table_id), column_ix: ix - 1, kind: *kind};
                    result.remove(0);
                    column_aliases.append(&mut result);
                    column_aliases.push(kind_tfm);
                  }
                  _ => (),
                }
              }
            }
            header_tfms.append(&mut column_aliases);
            table_children.remove(0);
          }
          _ => (),
        };
        if header_tfms.len() > 1 || table_children.len() > 1  {
          let mut args: Vec<Argument> = vec![];
          let mut result_tfms = vec![];
          for child in table_children {
            let mut result = self.compile_node(&child)?;
            match &result[0] {
              Transformation::NewTable{table_id,..} => {
                args.push((0,table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
              }
              Transformation::Select{table_id, indices} => {
                args.push((0,table_id.clone(),indices.to_vec()));
                result.remove(0);
              }
              _ => (),
            }  
            result_tfms.append(&mut result);       
          }
          header_tfms.insert(0,Transformation::NewTable{table_id: TableId::Local(anon_table_id), rows: 1, columns: 1});
          body_tfms.append(&mut result_tfms);
          body_tfms.push(Transformation::Function{
            name: *TABLE_VERTICAL__CONCATENATE,
            arguments: args,
            out: (TableId::Local(anon_table_id), TableIndex::All, TableIndex::All),
          });
          tfms.append(&mut header_tfms);
          tfms.append(&mut body_tfms);
        } else {
          let mut result = self.compile_nodes(&table_children)?;
          tfms.append(&mut result);          
        }
        match &tfms[0] {
          Transformation::NewTable{table_id,..} |
          Transformation::Select{table_id, ..} => {
            let reference_table_id = TableId::Local(hash_str(&format!("reference:{:?}", tfms[0])));
            let value = Value::Reference(table_id.clone());
            let out = TableId::Global(*table_id.unwrap());
            let in_t = table_id.clone();
            if in_t != out {
              tfms.insert(0,Transformation::NewTable{table_id: reference_table_id, rows: 1, columns: 1});
              /*tfms.insert(0,Transformation::Function{
                name: *TABLE_DEFINE,
                arguments: vec![(0,in_t,vec![(TableIndex::All, TableIndex::All)])],
                out: (out,TableIndex::All, TableIndex::All),
              });*/
              tfms.insert(0,Transformation::TableDefine{table_id: in_t, indices: vec![(TableIndex::All, TableIndex::All)], out});
              tfms.insert(0,Transformation::TableReference{table_id: reference_table_id, reference: value});
            } else {
              tfms.insert(0,Transformation::NewTable{table_id: reference_table_id, rows: 1, columns: 1});
              tfms.insert(0,Transformation::TableReference{table_id: reference_table_id, reference: value});
            }
          
          }
          _ => (),
        }  
      },
      AstNode::TableColumn{children} => {
        let mut result = self.compile_nodes(children)?;
        tfms.append(&mut result);
      },
      AstNode::TableRow{children} => {
        let mut row_id = hash_str(&format!("horzcat:{:?}", children));
        let mut args: Vec<Argument> = vec![];
        let mut result_tfms = vec![];
        let mut all = false;
        let mut all_arg = vec![];
        for child in children {
          let mut result = self.compile_node(child)?;
          match &result[0] {
            Transformation::NewTable{table_id,..} => {
              args.push((0,table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
            }
            Transformation::Select{table_id, indices} => {
              if indices.len() == 1 {
                match (table_id, indices[0].clone()) {
                  (TableId::Global(table_id2), (TableIndex::All, TableIndex::All)) => {
                    all = true;
                    all_arg.push(result[0].clone());
                  }
                  _ => ()
                } 
              }
              args.push((0,table_id.clone(),indices.to_vec()));
              result.remove(0);
            }
            Transformation::TableReference{table_id,..} => {
              let table_id = table_id.clone();
              args.push((0,table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
            }
            _ => (),
          }  
          result_tfms.append(&mut result);       
        }
        if args.len() == 1 && all {
          tfms.append(&mut all_arg);
          tfms.append(&mut result_tfms);
        } else {
          tfms.push(Transformation::NewTable{table_id: TableId::Local(row_id), rows: 1, columns: 1});
          tfms.append(&mut result_tfms);
          tfms.push(Transformation::Function{
            name: *TABLE_HORIZONTAL__CONCATENATE,
            arguments: args,
            out: (TableId::Local(row_id), TableIndex::All, TableIndex::All),
          });
        }
      },
      AstNode::AddRow{children} => {
        let mut result_tfms = Vec::new();
        let mut args: Vec<Argument> = Vec::new();

        let mut result = self.compile_node(&children[0])?;
        match &result[0] {
          Transformation::NewTable{table_id,..} => {
            args.push((0,table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
            result.remove(0);
          }
          Transformation::Select{table_id, indices} => {
            args.push((0,table_id.clone(),indices.to_vec()));
            result.remove(0);
          }
          _ => (),
        }
        result_tfms.append(&mut result); 

        let mut result = self.compile_node(&children[1])?;
        loop {
          match &result[0] {
            Transformation::NewTable{table_id,..} => {
              args.push((0,table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
              break;
            }
            Transformation::Select{table_id, indices} => {
              args.push((0,table_id.clone(),indices.to_vec()));
              result.remove(0);
              break;
            }
            Transformation::TableReference{table_id,..} => {
              result.remove(0);
              result.remove(0);
              result.remove(0);
              continue;
            }
            _ => (),
          }
        }
        result_tfms.append(&mut result); 

        let (_,o,oi) = &args[0];
        let (or,oc) = &oi[0];
        tfms.append(&mut result_tfms);
        tfms.push(Transformation::Function{
          name: *TABLE_APPEND,
          arguments: vec![args[1].clone()],
          out: (*o,or.clone(),oc.clone()),
        });
      },
      AstNode::SplitData{children} => {
        let mut result_tfms = Vec::new();
        let mut args: Vec<Argument> = Vec::new();
        let mut out = self.compile_node(&children[0])?;
        let mut out_id = TableId::Local(0);
        match &out[0] {
          Transformation::NewTable{table_id,..} => {
            out_id = *table_id;
            result_tfms.append(&mut out); 
          }
          Transformation::Identifier{name, id} => {
            out_id = TableId::Local(*id);
            result_tfms.append(&mut out); 
            result_tfms.push(Transformation::NewTable{table_id: out_id, rows: 1, columns: 1}); 
          }
          _ => (),
        }
        let mut result = self.compile_node(&children[1])?;
        match &result[0] {
          Transformation::NewTable{table_id,..} => {
            args.push((0,table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
            result.remove(0);
          }
          Transformation::Select{table_id, indices} => {
            args.push((0,table_id.clone(),indices.to_vec()));
            result.remove(0);
          }
          _ => (),
        }
        result_tfms.append(&mut result); 
        tfms.append(&mut result_tfms);
        tfms.push(Transformation::Function{
          name: *TABLE_SPLIT,
          arguments: vec![args[0].clone()],
          out: (out_id,TableIndex::All,TableIndex::All),
        });
      }
      AstNode::FlattenData{children} => {
        let mut result_tfms = Vec::new();
        let mut args: Vec<Argument> = Vec::new();
        let mut out = self.compile_node(&children[0])?;
        let mut out_id = TableId::Local(0);
        match &out[0] {
          Transformation::NewTable{table_id,..} => {
            out_id = *table_id;
            result_tfms.append(&mut out); 
          }
          Transformation::Identifier{name, id} => {
            out_id = TableId::Local(*id);
            result_tfms.append(&mut out); 
            result_tfms.push(Transformation::NewTable{table_id: out_id, rows: 1, columns: 1}); 
          }
          _ => (),
        }
        let mut result = self.compile_node(&children[1])?;
        match &result[0] {
          Transformation::NewTable{table_id,..} => {
            args.push((0,table_id.clone(),vec![(TableIndex::All, TableIndex::All)]));
            result.remove(0);
          }
          Transformation::Select{table_id, indices} => {
            args.push((0,table_id.clone(),indices.to_vec()));
            result.remove(0);
          }
          _ => (),
        }
        result_tfms.append(&mut result); 
        tfms.append(&mut result_tfms);
        tfms.push(Transformation::Function{
          name: *TABLE_FLATTEN,
          arguments: vec![args[0].clone()],
          out: (out_id,TableIndex::All,TableIndex::All),
        });
      }
      AstNode::TransposeSelect{children} => {
        let mut result = self.compile_node(&children[0])?;
        let mut args = vec![];
        match &result[0] {
          Transformation::Select{table_id, indices} => {
            args.push((0, *table_id, indices.to_vec()));
            result.remove(0);
          }
          _=>(),
        }
        let id = hash_str(&format!("transpose{:?}",args));
        tfms.push(Transformation::NewTable{table_id: TableId::Local(id), rows: 1, columns: 1});
        tfms.append(&mut result);
        tfms.push(Transformation::Function{
          name: *MATRIX_TRANSPOSE,
          arguments: args,
          out: (TableId::Local(id),TableIndex::All,TableIndex::All),
        });
      }
      AstNode::SelectData{name, id, children, ..} => {
        let mut indices = vec![];
        let mut all_indices = vec![];
        let mut local_tfms = vec![];
        for child in children {
          match child {
            AstNode::ReshapeColumn => {
              indices.push(TableIndex::ReshapeColumn);
              indices.push(TableIndex::All);
            }
            AstNode::Swizzle{children} => {
              let mut aliases = vec![];
              for child in children {
                match child {
                  AstNode::Identifier{name,id,..} => {
                    aliases.push(*id);
                  }
                  _ => (),
                }
              }
              indices.push(TableIndex::All);
              indices.push(TableIndex::Aliases(aliases));
            }
            AstNode::DotIndex{children} => {
              for child in children {
                match child {
                  AstNode::Null => {
                    indices.push(TableIndex::All);
                  }
                  AstNode::Identifier{name, id, ..} => {
                    indices.push(TableIndex::Alias(*id));
                  }
                  AstNode::SubscriptIndex{children} => {
                    for child in children {
                      match child {
                        AstNode::SelectAll => {
                          indices.push(TableIndex::All);
                        }
                        AstNode::WheneverIndex{..} => {
                          let id = hash_str("~");
                          if indices.len() == 2 && indices[0] == TableIndex::All {
                            indices[0] = TableIndex::IxTable(TableId::Local(id));
                          } else {
                            indices.push(TableIndex::IxTable(TableId::Local(id)));
                          }
                        }
                        AstNode::SelectData{name, id, children, ..} => {
                          if indices.len() == 2 && indices[0] == TableIndex::All {
                            indices[0] = TableIndex::IxTable(*id);
                          } else {
                            indices.push(TableIndex::IxTable(*id));
                          }
                        }
                        AstNode::Expression{..} => {
                          let mut result = self.compile_node(child)?;
                          match &result[1] {
                            Transformation::NewTable{table_id, ..} => {
                              indices.push(TableIndex::IxTable(*table_id));
                            }
                            Transformation::NumberLiteral{kind, bytes} => {
                              let mut value = NumberLiteral::new(*kind, bytes.clone());
                              if indices.len() == 2 && indices[0] == TableIndex::All {
                                indices[0] = TableIndex::Index(value.as_usize());
                              } else {
                                indices.push(TableIndex::Index(value.as_usize()));
                              }
                            }
                            Transformation::Function{name, arguments, out} => {
                              let (output_table_id, output_row, output_col) = out;
                              if indices.len() == 2 && indices[0] == TableIndex::All {
                                indices[0] = TableIndex::IxTable(*output_table_id);
                              } else {
                                indices.push(TableIndex::IxTable(*output_table_id));
                              }
                            }
                            _ => (),
                          }
                          local_tfms.append(&mut result);
                        }
                        _ => (),
                      }
                    }
                  }
                  _ => (),
                }
              }
            }
            AstNode::SubscriptIndex{children} => {
              for child in children {
                match child {
                  AstNode::SelectAll => {
                    indices.push(TableIndex::All);
                  }
                  AstNode::WheneverIndex{..} => {
                    let id = hash_str("~");
                    if indices.len() == 2 && indices[0] == TableIndex::All {
                      indices[0] = TableIndex::IxTable(TableId::Local(id));
                    } else {
                      indices.push(TableIndex::IxTable(TableId::Local(id)));
                    }
                  }
                  AstNode::SelectData{name, id, children, ..} => {
                    if indices.len() == 2 && indices[0] == TableIndex::All {
                      indices[0] = TableIndex::IxTable(*id);
                    } else {
                      indices.push(TableIndex::IxTable(*id));
                    }
                  }
                  AstNode::Expression{..} => {
                    let mut result = self.compile_node(child)?;
                    match &result[1] {
                      Transformation::NewTable{table_id, ..} => {
                        indices.push(TableIndex::IxTable(*table_id));
                      }
                      Transformation::NumberLiteral{kind, bytes} => {
                        let mut value = NumberLiteral::new(*kind, bytes.clone());
                        if indices.len() == 2 && indices[0] == TableIndex::All {
                          indices[0] = TableIndex::Index(value.as_usize());
                        } else {
                          indices.push(TableIndex::Index(value.as_usize()));
                        }
                      }
                      Transformation::Function{name, arguments, out} => {
                        let (output_table_id, output_row, output_col) = out;
                        if indices.len() == 2 && indices[0] == TableIndex::All {
                          indices[0] = TableIndex::IxTable(*output_table_id);
                        } else {
                          indices.push(TableIndex::IxTable(*output_table_id));
                        }
                      }
                      _ => (),
                    }
                    local_tfms.append(&mut result);
                  }
                  _ => (),
                }
              }
              if children.len() == 1 {
                indices.push(TableIndex::None);
              }
            }
            _ => {
              indices.push(TableIndex::All);
              indices.push(TableIndex::All);
            },
          }
          if indices.len() == 2 {
            all_indices.push((indices[0].clone(),indices[1].clone()));
            indices.clear();
          }
        }
        tfms.push(Transformation::Select{table_id: *id, indices: all_indices});
        tfms.append(&mut local_tfms);
        tfms.push(Transformation::Identifier{name: name.clone(), id: *id.unwrap()});
      }
      AstNode::Whenever{children} => {
        let mut result = self.compile_nodes(children)?;
        match &result[0] {
          Transformation::Select{table_id, indices} => {
            tfms.push(Transformation::Whenever{table_id:*table_id, indices: indices.to_vec()});
          }
          _ => (),
        }
        tfms.append(&mut result);
      }
      AstNode::FunctionBody{children} |
      AstNode::FunctionInput{children} |
      AstNode::FunctionOutput{children} |
      AstNode::FunctionArgs{children} |
      AstNode::Comment{children} |
      AstNode::Program{children, ..} |
      AstNode::Section{children, ..} |
      AstNode::Attribute{children} |
      AstNode::Transformation{children} |
      AstNode::Statement{children, ..} |
      AstNode::Fragment{children} |
      AstNode::Block{children, ..} |
      AstNode::MathExpression{children} |
      AstNode::Expression{children} |
      AstNode::TableRow{children} |
      AstNode::TableColumn{children} |
      AstNode::Binding{children} |
      AstNode::FunctionBinding{children} |
      AstNode::Root{children} => {
        let mut result = self.compile_nodes(children)?;
        tfms.append(&mut result);
      }
      AstNode::Null => (),
      x => println!("Unhandled AstNode in Compiler {:?}", x),
    }
    Ok(tfms)
  }
}

pub fn compile_text(node: &ParserNode) -> Result<String,MechError> {
  let mut ast = Ast::new();
  match &ast.build_syntax_tree(node)[0] {
    AstNode::String{text,..} => {
      Ok(MechString::from_chars(text).to_string())
    }
    x => Err(MechError{msg: "".to_string(), id: 7392, kind: MechErrorKind::GenericError(format!("Unhandled Node: {:?}", x))})
  }
}