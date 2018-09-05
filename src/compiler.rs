// # Mech Syntax Compiler

// ## Preamble

use mech_core::{Block, Constraint};
use mech_core::{Function, Plan, Comparator};
use mech_core::Hasher;
use parser;
use lexer::Lexer;
use parser::{Parser, ParseStatus};
use lexer::Token;
use alloc::{String, Vec, fmt};

// ## Compiler Nodes

#[derive(Clone, PartialEq)]
pub enum Node {
  Root{ children: Vec<Node> },
  Fragment{ children: Vec<Node> },
  Program{ children: Vec<Node> },
  Head{ children: Vec<Node> },
  Body{ children: Vec<Node> },
  Section{ children: Vec<Node> },
  Block{ children: Vec<Node> },
  Statement{ children: Vec<Node> },
  Expression{ children: Vec<Node> },
  MathExpression{ children: Vec<Node> },
  FilterExpression{ children: Vec<Node> },
  SelectExpression{ children: Vec<Node> },
  Data{ children: Vec<Node> },
  DataWatch{ children: Vec<Node> },
  SelectData{ children: Vec<Node> },
  SetData{ children: Vec<Node> },
  Column{ children: Vec<Node> },
  Binding{ children: Vec<Node> },
  Function{ name: String, children: Vec<Node> },
  Define { name: String, id: u64},
  DotIndex { rows: Vec<Node>, columns: Vec<Node>},
  BracketIndex { rows: Vec<Node>, columns: Vec<Node>},
  VariableDefine {children: Vec<Node> },
  TableDefine {children: Vec<Node> },
  AnonymousTableDefine {children: Vec<Node> },
  TableRow {children: Vec<Node> },
  AddRow {children: Vec<Node> },
  Constraint{ children: Vec<Node> },
  Title{ text: String },
  Identifier{ name: String, id: u64 },
  Table{ name: String, id: u64 },
  Paragraph{ text: String },
  Constant {value: i64},
  String{ text: String },
  Token{ token: Token, byte: u8 },
  Null,
}

impl fmt::Debug for Node {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    print_recurse(self, 0);
    Ok(())
  }
}

pub fn print_recurse(node: &Node, level: usize) {
  spacer(level);
  let children: Option<&Vec<Node>> = match node {
    Node::Root{children} => {print!("Root\n"); Some(children)},
    Node::Fragment{children} => {print!("Fragment\n"); Some(children)},
    Node::Program{children} => {print!("Program\n"); Some(children)},
    Node::Head{children} => {print!("Head\n"); Some(children)},
    Node::Body{children} => {print!("Body\n"); Some(children)},
    Node::VariableDefine{children} => {print!("VariableDefine\n"); Some(children)},
    Node::Column{children} => {print!("Column\n"); Some(children)},
    Node::Binding{children} => {print!("Binding\n"); Some(children)},
    Node::TableDefine{children} => {print!("TableDefine\n"); Some(children)},
    Node::AnonymousTableDefine{children} => {print!("AnonymousTableDefine\n"); Some(children)},
    Node::TableRow{children} => {print!("TableRow\n"); Some(children)},
    Node::AddRow{children} => {print!("AddRow\n"); Some(children)},
    Node::Section{children} => {print!("Section\n"); Some(children)},
    Node::Block{children} => {print!("Block\n"); Some(children)},
    Node::Statement{children} => {print!("Statement\n"); Some(children)},
    Node::SetData{children} => {print!("SetData\n"); Some(children)},
    Node::Data{children} => {print!("Data\n"); Some(children)},
    Node::DataWatch{children} => {print!("DataWatch\n"); Some(children)},
    Node::SelectData{children} => {print!("SelectData\n"); Some(children)},
    Node::DotIndex{rows, columns} => {print!("DotIndex[rows: {:?}, columns: {:?}]\n", rows, columns); None},
    Node::BracketIndex{rows, columns} => {print!("BracketIndex[rows: {:?}, columns: {:?}]\n", rows, columns); None},
    Node::Expression{children} => {print!("Expression\n"); Some(children)},
    Node::Function{name, children} => {print!("Function({:?})\n", name); Some(children)},
    Node::MathExpression{children} => {print!("MathExpression\n"); Some(children)},
    Node::SelectExpression{children} => {print!("SelectExpression\n"); Some(children)},
    Node::FilterExpression{children} => {print!("FilterExpression\n"); Some(children)},
    Node::Constraint{children} => {print!("Constraint\n"); Some(children)},
    Node::Identifier{name, id} => {print!("{}({:#x})\n", name, id); None},
    Node::String{text} => {print!("String({:?})\n", text); None},
    Node::Title{text} => {print!("Title({:?})\n", text); None},
    Node::Constant{value} => {print!("{:?}\n", value); None},
    Node::Paragraph{text} => {print!("Paragraph({:?})\n", text); None},
    Node::Table{name,id} => {print!("#{}({:?})\n", name, id); None},
    Node::Define{name,id} => {print!("Define #{}({:?})\n", name, id); None},
    Node::Token{token, byte} => {print!("Token({:?})\n", token); None},
    Node::Null => {print!("Null\n"); None},
    _ => {print!("Unhandled Node"); None},
  };  
  match children {
    Some(childs) => {
      for child in childs {
        print_recurse(child, level + 1)
      }
    },
    _ => (),
  }    
}

pub fn spacer(width: usize) {
  let limit = if width > 0 {
    width - 1
  } else {
    width
  };
  for _ in 0..limit {
    print!("│");
  }
  print!("├");
}

// ## Compiler

#[derive(Debug)]
pub struct Compiler {
  pub blocks: Vec<Block>,
  pub constraints: Vec<Constraint>,
  depth: usize,
  row: usize,
  column: usize,
  table: u64,
  pub parse_tree: parser::Node,
  pub syntax_tree: Node,
  pub node_stack: Vec<Node>, 
  pub section: usize,
  pub block: usize,
}

impl Compiler {

  pub fn new() -> Compiler {
    Compiler {
      blocks: Vec::new(),
      constraints: Vec::new(),
      node_stack: Vec::new(),
      depth: 0,
      column: 0,
      row: 0,
      table: 0,
      section: 1,
      block: 1,
      parse_tree: parser::Node::Root{ children: Vec::new() },
      syntax_tree: Node::Root{ children: Vec::new() },
    }
  }

  pub fn compile_string(&mut self, input: String) -> &Vec<Block> {
    let mut lexer = Lexer::new();
    let mut parser = Parser::new();
    lexer.add_string(input.clone());
    let tokens = lexer.get_tokens();
    parser.text = input;
    parser.add_tokens(&mut tokens.clone());
    parser.build_parse_tree();
    self.parse_tree = parser.parse_tree.clone();
    self.build_syntax_tree(parser.parse_tree);
    let ast = self.syntax_tree.clone();
    self.compile_blocks(ast);
    &self.blocks
  }

  pub fn compile_blocks(&mut self, node: Node) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    match node {
      Node::Block{children} => {
        let mut block = Block::new();
        block.name = format!("{:?},{:?}", self.section, self.block);
        block.id = Hasher::hash_string(block.name.clone()) as usize;
        self.block += 1;
        let constraints = self.compile_constraints(&children);
        block.add_constraints(constraints);
        block.plan();
        blocks.push(block);
      },
      Node::Root{children} => {
        let result = self.compile_children(children);
        self.blocks = result;
      },
      Node::Program{children} => {blocks.append(&mut self.compile_children(children));},
      Node::Fragment{children} => {
        let mut block = Block::new();
        block.name = format!("{:?},{:?}", self.section, self.block);
        block.id = Hasher::hash_string(block.name.clone()) as usize;
        self.block += 1;
        let constraints = self.compile_constraints(&children);
        block.add_constraints(constraints);
        block.plan();
        blocks.push(block);
      },
      Node::Body{children} => {blocks.append(&mut self.compile_children(children));},
      Node::Section{children} => {
        blocks.append(&mut self.compile_children(children));
        self.section += 1;
        self.block = 1;
      },
      _ => (),
    }
    blocks
  }

  pub fn compile_constraint(&mut self, node: &Node) -> Vec<Constraint> {
    
    let mut constraints: Vec<Constraint> = Vec::new();
    match node {
      Node::Statement{children} => {
        constraints.append(&mut self.compile_constraints(children));
      },
      Node::Constraint{children} => {
        self.row = 0;
        self.column = 0;
        constraints.append(&mut self.compile_constraints(children));
      },
      Node::Expression{children} => {
        constraints.append(&mut self.compile_constraints(children));
      },
      Node::AnonymousTableDefine{children} => {
        constraints.append(&mut self.compile_constraints(children));
      },
      Node::VariableDefine{children} => {
        let mut result = self.compile_constraint(&children[0]);
        match result[0] {
          Constraint::Identifier{id} => self.table = Hasher::hash_string(format!("{:?},{:?}-{:?}", self.section, self.block, id)),
          _ => (),
        }
        let mut result = self.compile_constraint(&children[1]);
        constraints.push(Constraint::NewBlockTable{id: self.table, rows: self.row as u64, columns: self.column as u64});
        constraints.append(&mut result);
      },
      Node::MathExpression{children} => {
        constraints.append(&mut self.compile_constraints(children));
      },
      Node::TableRow{children} => {
        self.row += 1;
        self.column = 0;
        constraints.append(&mut self.compile_constraints(children));
      },
      Node::Column{children} => {
        self.column += 1;
        constraints.append(&mut self.compile_constraints(children));
      },
      Node::Identifier{name, id} => {
        constraints.push(Constraint::Identifier{id: *id});
      },
      Node::Constant{value} => {
        constraints.push(Constraint::Constant{table: self.table, row: self.row as u64, column: self.column as u64, value: *value as i64});
      },
      _ => (),
    }
    constraints
  }

  pub fn compile_constraints(&mut self, nodes: &Vec<Node>) -> Vec<Constraint> {
    let mut compiled = Vec::new();
    for node in nodes {
      compiled.append(&mut self.compile_constraint(node));
    }
    compiled
  }

  pub fn compile_children(&mut self, nodes: Vec<Node>) -> Vec<Block> {
    let mut compiled = Vec::new();
    for node in nodes {
      compiled.append(&mut self.compile_blocks(node));
    }
    compiled
  }

  pub fn build_syntax_tree(&mut self, node: parser::Node) -> Vec<Node> {
    let mut compiled = Vec::new();
    self.depth += 1;
    match node {
      parser::Node::Root{children} => {
        let result = self.compile_nodes(children);
        self.syntax_tree = Node::Root{children: result};        
      },
      parser::Node::Fragment{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::Fragment{children: result});
      },
      parser::Node::Program{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::Program{children: result});
      },
      parser::Node::Head{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::Head{children: result});
      },
      parser::Node::Body{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::Body{children: result});
      },
      parser::Node::Section{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::Section{children: result});
      },
      parser::Node::Block{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::Block{children: result});
      },
      parser::Node::Data{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::Data{children: result});
      },
      parser::Node::SelectData{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::SelectData{children: result});
      },
      parser::Node::Statement{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::Statement{children: result});
      },
      parser::Node::Expression{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::Expression{children: result});
      },
      parser::Node::DataWatch{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::DataWatch{children: result});
      },
      parser::Node::SetData{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{..} => (), 
            _ => children.push(node),
          }
        }
        compiled.push(Node::SetData{children});
      },
      parser::Node::Column{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{..} => (),
            _ => children.push(node),
          }
        }
        compiled.push(Node::Column{children});
      },
      parser::Node::Binding{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{..} => (), 
            _ => children.push(node),
          }
        }
        compiled.push(Node::Binding{children});
      },
      parser::Node::Constraint{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            // Ignore irrelevant nodes like spaces and operators
            Node::Token{..} => (), 
            _ => children.push(node),
          }
        }
        compiled.push(Node::Constraint{children});
      },
      parser::Node::SelectExpression{children} => {
        let result = self.compile_nodes(children);
        compiled.push(Node::SelectExpression{children: result});
      },
      parser::Node::FilterExpression{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{token: Token::Space, ..} => (), 
            _ => children.push(node),
          }
        }
        compiled.push(Node::FilterExpression{children});
      },
      parser::Node::AnonymousTable{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{..} => (), 
            _ => children.push(node),
          }
        }
        compiled.push(Node::AnonymousTableDefine{children});
      },
      parser::Node::TableRow{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{..} => (), 
            _ => children.push(node),
          }
        }
        compiled.push(Node::TableRow{children});
      },
      parser::Node::MathExpression{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            // Ignore irrelevant nodes like spaces and operators
            Node::Token{..} => (), 
            _ => children.push(node),
          }
        }
        compiled.push(Node::MathExpression{children});
      },
      parser::Node::Infix{children} => {
        let result = self.compile_nodes(children);
        let operator = &result[0];
        let name: String = match operator {
          Node::Token{token, byte} => byte_to_char(*byte).unwrap().to_string(),
          _ => String::from(""),
        };
        compiled.push(Node::Function{name, children: vec![]});
      },
      parser::Node::VariableDefine{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{..} => (),
            _ => children.push(node),
          }
        }
        compiled.push(Node::VariableDefine{children});
      },
      parser::Node::TableDefine{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{..} => (),
            _ => children.push(node),
          }
        }
        compiled.push(Node::TableDefine{children});
      },
      parser::Node::AddRow{children} => {
        let result = self.compile_nodes(children);
        let mut children: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{..} => (),
            _ => children.push(node),
          }
        }
        compiled.push(Node::AddRow{children});
      },
      parser::Node::Index{children} => {
        compiled.append(&mut self.compile_nodes(children));
      },
      parser::Node::DotIndex{children} => {
        let result = self.compile_nodes(children);
        let mut columns: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{token, byte} => (),
            _ => columns.push(node),
          };
        }
        compiled.push(Node::DotIndex{rows: vec![], columns});
      },
      parser::Node::BracketIndex{children} => {
        let result = self.compile_nodes(children);
        let mut columns: Vec<Node> = Vec::new();
        for node in result {
          match node {
            Node::Token{token, byte} => (),
            _ => columns.push(node),
          };
        }
        compiled.push(Node::BracketIndex{rows: vec![], columns});
      },
      parser::Node::Table{children} => {
        let result = self.compile_nodes(children);
        match &result[1] {
          Node::Identifier{name, id} => compiled.push(Node::Table{name: name.to_string(), id: *id}),
          _ => (),
        };
      },  
      // Quantities
      parser::Node::Number{children} => {
        let mut value = 0;
        let mut result = self.compile_nodes(children);
        let mut place = result.len();
        for node in result {
          match node {
            Node::Token{token, byte} => {
              let digit = byte_to_digit(byte).unwrap();
              let q = digit * magnitude(place);
              place -= 1;
              value += q;
            },
            _ => (),
          }
        }
        compiled.push(Node::Constant{value: value as i64});
      },
      // String-like nodes
      parser::Node::Paragraph{children} => {
        let mut result = self.compile_nodes(children);
        let node = match &result[0] {
          Node::String{text} => Node::Paragraph{text: text.clone()},
          _ => Node::Null,
        };
        compiled.push(node);
      },
      parser::Node::Title{children} => {
        let mut result = self.compile_nodes(children);
        // space space #
        let node = match &result[2] {
          Node::String{text} => Node::Title{text: text.clone()},
          _ => Node::Null,
        };
        compiled.push(node);
      },
      parser::Node::Subtitle{children} => {
        let mut result = self.compile_nodes(children);
        // space space # #
        let node = match &result[3] {
          Node::String{text} => Node::Title{text: text.clone()},
          _ => Node::Null,
        };
        compiled.push(node);
      },
      parser::Node::Text{children} => {
        let mut result = self.compile_nodes(children);
        let mut text_node = String::new();
        for node in result {
          match node {
            Node::String{text} => text_node.push_str(&text),
            Node::Token{token: Token::Space, ..} => text_node.push(' '),
            _ => (),
          }
        }
        compiled.push(Node::String{text: text_node});
      },
      parser::Node::Word{children} => {
        let mut word = String::new();
        let mut result = self.compile_nodes(children);
        for node in result {
          match node {
            Node::Token{token, byte} => {
              let character = byte_to_char(byte).unwrap();
              word.push(character);
            },
            _ => (),
          }
        }
        compiled.push(Node::String{text: word});
      },
      parser::Node::TableIdentifier{children} |
      parser::Node::Identifier{children} => {
        let mut word = String::new();
        let mut result = self.compile_nodes(children);
        for node in result {
          match node {
            Node::Token{token, byte} => {
              let character = byte_to_char(byte).unwrap();
              word.push(character);
            },
            _ => compiled.push(node),
          }
        }
        let id = Hasher::hash_string(word.clone());
        compiled.push(Node::Identifier{name: word, id});
      },
      // Math
      parser::Node::L1{children} |
      parser::Node::L2{children} |
      parser::Node::L3{children} |
      parser::Node::L4{children} => {
        let result = self.compile_nodes(children);
        let mut last = Node::Null;
        for node in result {
          match last {
            Node::Null => last = node,
            _ => {
              let (name, mut children) = match node {
                Node::Function{name, mut children} => (name.clone(), children.clone()),
                _ => (String::from(""), vec![]),
              };
              children.push(last);
              children.reverse();
              last = Node::Function{name, children};
            },
          };
        }
        compiled.push(last);
      },
      parser::Node::L1Infix{children} |
      parser::Node::L2Infix{children} |
      parser::Node::L3Infix{children} => {
        let result = self.compile_nodes(children);
        let operator = &result[1].clone();
        let input = &result[3].clone();
        let name: String = match operator {
          Node::Token{token, byte} => byte_to_char(*byte).unwrap().to_string(),
          _ => String::from(""),
        };        
        compiled.push(Node::Function{name, children: vec![input.clone()]});
      },
      // Pass through nodes. These will just be omitted
      parser::Node::Comparator{children} |
      parser::Node::IdentifierOrConstant{children} |
      parser::Node::ProseOrCode{children}|
      parser::Node::StatementOrExpression{children} |
      parser::Node::DataWatch{children} |
      parser::Node::Constant{children} |
      parser::Node::SetOperator{children} |
      parser::Node::Repeat{children} |
      parser::Node::Alphanumeric{children} |
      parser::Node::IdentifierCharacter{children} => {
        compiled.append(&mut self.compile_nodes(children));
      },
      parser::Node::Token{token, byte} => {
        compiled.push(Node::Token{token, byte});
      },
      _ => (),
    }
    
    //self.constraints = constraints.clone();
    compiled
  }

  pub fn compile_nodes(&mut self, nodes: Vec<parser::Node>) -> Vec<Node> {
    let mut compiled = Vec::new();
    for node in nodes {
      compiled.append(&mut self.build_syntax_tree(node));
    }
    compiled
  }

}

fn get_destination_register(constraint: &Constraint) -> Option<usize> {
  match constraint {
    Constraint::CopyInput{input, memory} => Some(*memory as usize),
    _ => None,
  }
}

// ## Appendix 

// ### Encodings

fn byte_to_digit(byte: u8) -> Option<u64> {
  match byte {
    48 => Some(0),
    49 => Some(1),
    50 => Some(2),
    51 => Some(3),
    52 => Some(4),
    53 => Some(5),
    54 => Some(6),
    55 => Some(7),
    56 => Some(8),
    57 => Some(9),
    _ => None,
  }
}

fn byte_to_char(byte: u8) -> Option<char> {
  match byte {
    33 => Some('!'),
    42 => Some('*'),
    43 => Some('+'),
    45 => Some('-'),
    47 => Some('/'),
    48 => Some('0'),
    49 => Some('1'),
    50 => Some('2'),
    51 => Some('3'),
    52 => Some('4'),
    53 => Some('5'),
    54 => Some('6'),
    55 => Some('7'),
    56 => Some('8'),
    57 => Some('9'),
    60 => Some('<'),
    62 => Some('>'),
    97 => Some('a'),
    98 => Some('b'),
    99 => Some('c'),
    100 => Some('d'),
    101 => Some('e'),
    102 => Some('f'),
    103 => Some('g'),
    104 => Some('h'),
    105 => Some('i'),
    106 => Some('j'),
    107 => Some('k'),
    108 => Some('l'),
    109 => Some('m'),
    110 => Some('n'),
    111 => Some('o'),
    112 => Some('p'),
    113 => Some('q'),
    114 => Some('r'),
    115 => Some('s'),
    116 => Some('t'),
    117 => Some('u'),
    118 => Some('v'),
    119 => Some('w'),
    120 => Some('x'),
    121 => Some('y'),    
    122 => Some('z'),
    126 => Some('~'),
    65 => Some('A'),
    66 => Some('B'),
    67 => Some('C'),
    68 => Some('D'),
    69 => Some('E'),
    70 => Some('F'),
    71 => Some('G'),
    72 => Some('H'),
    73 => Some('I'),
    74 => Some('J'),
    75 => Some('K'),
    76 => Some('L'),
    77 => Some('M'),
    78 => Some('N'),
    79 => Some('O'),
    80 => Some('P'),
    81 => Some('Q'),
    82 => Some('R'),
    83 => Some('S'),
    84 => Some('T'),
    85 => Some('U'),
    86 => Some('V'),
    87 => Some('W'),
    88 => Some('X'),
    89 => Some('Y'),
    90 => Some('Z'),
    94 => Some('^'),
    _ => {
      println!("Unhandled Byte {:?}", byte);
      None
    },
  }
}

// ### Utility Functions

fn magnitude(n: usize) -> u64 {
  let mut m = 1;
  for i in 1 .. n {
    m = m * 10;
  }
  m
}