// ## Compiler Nodes

#[derive(Clone, PartialEq)]
pub enum Node {
  Root{ children: Vec<Node> },
  Fragment{ children: Vec<Node> },
  Program{title: Option<String>, children: Vec<Node> },
  Head{ children: Vec<Node> },
  Body{ children: Vec<Node> },
  Section{title: Option<String>, children: Vec<Node> },
  Block{ children: Vec<Node> },
  Statement{ children: Vec<Node> },
  Expression{ children: Vec<Node> },
  MathExpression{ children: Vec<Node> },
  SelectExpression{ children: Vec<Node> },
  Data{ children: Vec<Node> },
  Whenever{ children: Vec<Node> },
  WheneverIndex{ children: Vec<Node> },
  Wait{ children: Vec<Node> },
  Until{ children: Vec<Node> },
  SelectData{name: String, id: TableId, children: Vec<Node> },
  SetData{ children: Vec<Node> },
  SplitData{ children: Vec<Node> },
  TableColumn{ children: Vec<Node> },
  Binding{ children: Vec<Node> },
  FunctionBinding{ children: Vec<Node> },
  Function{ name: String, children: Vec<Node> },
  Define { name: String, id: u64},
  DotIndex { children: Vec<Node>},
  SubscriptIndex { children: Vec<Node> },
  Range,
  VariableDefine {children: Vec<Node> },
  TableDefine {children: Vec<Node> },
  AnonymousTableDefine {children: Vec<Node> },
  AnonymousMatrixDefine {children: Vec<Node> },
  InlineTable {children: Vec<Node> },
  TableHeader {children: Vec<Node> },
  Attribute {children: Vec<Node> },
  TableRow {children: Vec<Node> },
  Comment {children: Vec<Node> },
  AddRow {children: Vec<Node> },
  Transformation{ children: Vec<Node> },
  Identifier{ name: String, id: u64 },
  Table{ name: String, id: u64 },
  Quantity {value: Quantity, unit: Option<String>},
  String{ text: String },
  Token{ token: Token, byte: u8 },
  Add,
  Subtract,
  Multiply,
  Divide,
  Exponent,
  LessThan,
  GreaterThan,
  GreaterThanEqual,
  LessThanEqual,
  Equal,
  NotEqual,
  And,
  Or,
  Xor,
  SelectAll,
  Empty,
  True,
  False,
  NumberLiteral{kind: NumberLiteralKind, bytes: Vec<u8> },
  RationalNumber{children: Vec<Node> },
  // Markdown
  SectionTitle{ text: String },
  Title{ text: String },
  ParagraphText{ text: String },
  Paragraph{ children: Vec<Node> },
  UnorderedList{ children: Vec<Node> },
  ListItem{ children: Vec<Node> },
  InlineCode{ children: Vec<Node> },
  CodeBlock{ children: Vec<Node> },
  // Mechdown
  InlineMechCode{ children: Vec<Node> },
  MechCodeBlock{ children: Vec<Node> },
  Null,
}

impl fmt::Debug for Node {
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    print_recurse(self, 0, f);
    Ok(())
  }
}

pub fn print_recurse(node: &Node, level: usize, f: &mut fmt::Formatter) {
  spacer(level,f);
  let children: Option<&Vec<Node>> = match node {
    Node::Root{children} => {write!(f,"Root\n").ok(); Some(children)},
    Node::Fragment{children, ..} => {write!(f,"Fragment\n").ok(); Some(children)},
    Node::Program{title, children} => {write!(f,"Program({:?})\n", title).ok(); Some(children)},
    Node::Head{children} => {write!(f,"Head\n").ok(); Some(children)},
    Node::Body{children} => {write!(f,"Body\n").ok(); Some(children)},
    Node::VariableDefine{children} => {write!(f,"VariableDefine\n").ok(); Some(children)},
    Node::TableColumn{children} => {write!(f,"TableColumn\n").ok(); Some(children)},
    Node::Binding{children} => {write!(f,"Binding\n").ok(); Some(children)},
    Node::FunctionBinding{children} => {write!(f,"FunctionBinding\n").ok(); Some(children)},
    Node::TableDefine{children} => {write!(f,"TableDefine\n").ok(); Some(children)},
    Node::AnonymousTableDefine{children} => {write!(f,"AnonymousTableDefine\n").ok(); Some(children)},
    Node::InlineTable{children} => {write!(f,"InlineTable\n").ok(); Some(children)},
    Node::TableHeader{children} => {write!(f,"TableHeader\n").ok(); Some(children)},
    Node::Attribute{children} => {write!(f,"Attribute\n").ok(); Some(children)},
    Node::TableRow{children} => {write!(f,"TableRow\n").ok(); Some(children)},
    Node::AddRow{children} => {write!(f,"AddRow\n").ok(); Some(children)},
    Node::Section{title, children} => {write!(f,"Section({:?})\n", title).ok(); Some(children)},
    Node::Block{children, ..} => {write!(f,"Block\n").ok(); Some(children)},
    Node::Statement{children} => {write!(f,"Statement\n").ok(); Some(children)},
    Node::SetData{children} => {write!(f,"SetData\n").ok(); Some(children)},
    Node::SplitData{children} => {write!(f,"SplitData\n").ok(); Some(children)},
    Node::Data{children} => {write!(f,"Data\n").ok(); Some(children)},
    Node::Whenever{children} => {write!(f,"Whenever\n").ok(); Some(children)},
    Node::WheneverIndex{children} => {write!(f,"WheneverIndex\n").ok(); Some(children)},
    Node::Wait{children} => {write!(f,"Wait\n").ok(); Some(children)},
    Node::Until{children} => {write!(f,"Until\n").ok(); Some(children)},
    Node::SelectData{name, id, children} => {write!(f,"SelectData({:?} {:?}))\n", name, id).ok(); Some(children)},
    Node::DotIndex{children} => {write!(f,"DotIndex\n").ok(); Some(children)},
    Node::SubscriptIndex{children} => {write!(f,"SubscriptIndex\n").ok(); Some(children)},
    Node::Range => {write!(f,"Range\n").ok(); None},
    Node::Expression{children} => {write!(f,"Expression\n").ok(); Some(children)},
    Node::Function{name, children} => {write!(f,"Function({:?})\n", name).ok(); Some(children)},
    Node::MathExpression{children} => {write!(f,"MathExpression\n").ok(); Some(children)},
    Node::Comment{children} => {write!(f,"Comment\n").ok(); Some(children)},
    Node::SelectExpression{children} => {write!(f,"SelectExpression\n").ok(); Some(children)},
    Node::Transformation{children, ..} => {write!(f,"Transformation\n").ok(); Some(children)},
    Node::Identifier{name, id} => {write!(f,"Identifier({}({:#x}))\n", name, id).ok(); None},
    Node::String{text} => {write!(f,"String({:?})\n", text).ok(); None},
    Node::RationalNumber{children} => {write!(f,"RationalNumber\n").ok(); Some(children)},
    Node::NumberLiteral{kind, bytes} => {write!(f,"NumberLiteral({:?})\n", bytes).ok(); None},
    Node::Quantity{value, unit} => {write!(f,"Quantity({}{:?})\n", value.to_f32(), unit).ok(); None},
    Node::Table{name,id} => {write!(f,"Table(#{}({:#x}))\n", name, id).ok(); None},
    Node::Define{name,id} => {write!(f,"Define #{}({:?})\n", name, id).ok(); None},
    Node::Token{token, byte} => {write!(f,"Token({:?})\n", token).ok(); None},
    Node::SelectAll => {write!(f,"SelectAll\n").ok(); None},
    Node::LessThan => {write!(f,"LessThan\n").ok(); None},
    Node::GreaterThan => {write!(f,"GreaterThan\n").ok(); None},
    Node::GreaterThanEqual => {write!(f,"GreaterThanEqual\n").ok(); None},
    Node::LessThanEqual => {write!(f,"LessThanEqual\n").ok(); None},
    Node::Equal => {write!(f,"Equal\n").ok(); None},
    Node::NotEqual => {write!(f,"NotEqual\n").ok(); None},
    Node::Empty => {write!(f,"Empty\n").ok(); None},
    Node::True => {write!(f,"True\n").ok(); None},
    Node::False => {write!(f,"False\n").ok(); None},
    Node::Null => {write!(f,"Null\n").ok(); None},
    Node::Add => {write!(f,"Add\n").ok(); None},
    Node::Subtract => {write!(f,"Subtract\n").ok(); None},
    Node::Multiply => {write!(f,"Multiply\n").ok(); None},
    Node::Divide => {write!(f,"Divide\n").ok(); None},
    Node::Exponent => {write!(f,"Exponent\n").ok(); None},
    // Markdown Nodes
    Node::Title{text} => {write!(f,"Title({:?})\n", text).ok(); None},
    Node::ParagraphText{text} => {write!(f,"ParagraphText({:?})\n", text).ok(); None},
    Node::UnorderedList{children} => {write!(f,"UnorderedList\n").ok(); Some(children)},
    Node::ListItem{children} => {write!(f,"ListItem\n").ok(); Some(children)},
    Node::Paragraph{children} => {write!(f,"Paragraph\n").ok(); Some(children)},
    Node::InlineCode{children} => {write!(f,"InlineCode\n").ok(); Some(children)},
    Node::CodeBlock{children} => {write!(f,"CodeBlock\n").ok(); Some(children)},
    // Extended Mechdown
    Node::InlineMechCode{children} => {write!(f,"InlineMechCode\n").ok(); Some(children)},
    Node::MechCodeBlock{children} => {write!(f,"MechCodeBlock\n").ok(); Some(children)},
    _ => {write!(f,"Unhandled Compiler Node").ok(); None},
  };
  match children {
    Some(childs) => {
      for child in childs {
        print_recurse(child, level + 1,f)
      }
    },
    _ => (),
  }
}

pub fn spacer(width: usize, f: &mut fmt::Formatter) {
  let limit = if width > 0 {
    width - 1
  } else {
    width
  };
  for _ in 0..limit {
    write!(f,"│").ok();
  }
  write!(f,"├").ok();
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
      let mut children = vec![];
      let mut title = None;
      for node in result {
        match node {
          Node::Title{text} => title = Some(text),
          _ => children.push(node),
        }
      }
      compiled.push(Node::Program{title, children});
    },
    parser::Node::Head{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::Head{children: result});
    },
    parser::Node::Section{children} => {
      let result = self.compile_nodes(children);
      let mut children = vec![];
      let mut title = None;
      for node in result {
        match node {
          Node::Title{text} => title = Some(text),
          _ => children.push(node),
        }
      }
      compiled.push(Node::Section{title, children});
    },
    parser::Node::Block{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::Block{children: result});
    },
    parser::Node::Data{children} => {
      let result = self.compile_nodes(children);
      let mut reversed = result.clone();
      reversed.reverse();
      let mut select_data_children: Vec<Node> = vec![];

      for node in reversed {
        match node {
          Node::Table{name, id} => {
            if select_data_children.is_empty() {
              select_data_children = vec![Node::Null; 1];
            }
            select_data_children.reverse();
            compiled.push(Node::SelectData{name, id: TableId::Global(id), children: select_data_children.clone()});
          },
          Node::Identifier{name, id} => {
            if select_data_children.is_empty() {
              select_data_children = vec![Node::Null; 1];
            }
            //select_data_children.reverse();
            compiled.push(Node::SelectData{name, id: TableId::Local(id), children: select_data_children.clone()});
          },
          Node::DotIndex{children} => {
            let mut reversed = children.clone();
            if children.len() == 1 {
              reversed.push(Node::Null);
              reversed.reverse();
            }
            select_data_children.push(Node::DotIndex{children: reversed});
          },
          Node::SubscriptIndex{..} => {
            /*let mut reversed = children.clone();
            reversed.reverse();*/
            select_data_children.push(node.clone());
          }
          _ => (),
        }
      }
    },
    parser::Node::Statement{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::Statement{children: result});
    },
    parser::Node::Expression{children} => {
      let result = self.compile_nodes(children);
      for node in result {
        match node {
          // If the node is a naked expression, modify the graph
          // TODO this is hacky... maybe change the parser?
          Node::SelectData{..} => {
            compiled.push(node);
          },
          _ => compiled.push(Node::Expression{children: vec![node]}),
        }
      }
    },
    parser::Node::Attribute{children} => {
      let result = self.compile_nodes(children);
      let mut children: Vec<Node> = Vec::new();
      for node in result {
        match node {
          Node::Token{..} => (),
          _ => children.push(node),
        }
      }
      compiled.push(Node::Attribute{children});
    },
    parser::Node::Whenever{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::Whenever{children: result});
    },
    parser::Node::Wait{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::Wait{children: result});
    },
    parser::Node::Until{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::Until{children: result});
    },
    parser::Node::SelectAll => {
      compiled.push(Node::SelectAll);
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
    parser::Node::SplitData{children} => {
      let result = self.compile_nodes(children);
      let mut children: Vec<Node> = Vec::new();
      for node in result {
        match node {
          Node::Token{..} => (),
          _ => children.push(node),
        }
      }
      compiled.push(Node::SplitData{children});
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
      compiled.push(Node::TableColumn{children});
    },
    parser::Node::Empty => {
      compiled.push(Node::Empty);
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
    parser::Node::FunctionBinding{children} => {
      let result = self.compile_nodes(children);
      let mut children: Vec<Node> = Vec::new();
      for node in result {
        match node {
          Node::Token{..} => (),
          _ => children.push(node),
        }
      }
      compiled.push(Node::FunctionBinding{children});
    },
    parser::Node::Transformation{children} => {
      let result = self.compile_nodes(children);
      let mut children: Vec<Node> = Vec::new();
      for node in result {
        match node {
          // Ignore irrelevant nodes like spaces and operators
          Node::Token{..} => (),
          _ => children.push(node),
        }
      }
      if !children.is_empty() {
        compiled.push(Node::Transformation{children});
      }
    },
    parser::Node::SelectExpression{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::SelectExpression{children: result});
    },
    parser::Node::InlineTable{children} => {
      let result = self.compile_nodes(children);
      let mut children: Vec<Node> = Vec::new();
      for node in result {
        match node {
          Node::Token{..} => (),
          _ => children.push(node),
        }
      }
      compiled.push(Node::InlineTable{children});
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
    parser::Node::TableHeader{children} => {
      let result = self.compile_nodes(children);
      let mut children: Vec<Node> = Vec::new();
      for node in result {
        match node {
          Node::Token{..} => (),
          _ => children.push(node),
        }
      }
      compiled.push(Node::TableHeader{children});
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
      let mut new_node = false;
      for node in result {
        match node {
          // Ignore irrelevant nodes like spaces and operators
          Node::Token{..} => (),
          Node::Function{..} => {
            new_node = true;
            children.push(node);
          },
          Node::Quantity{..} => {
            new_node = false;
            children.push(node);
          }
          _ => children.push(node),
        }
      }
      // TODO I might be able to simplify this now. This doesn't seem to be necessary
      if new_node {
        compiled.push(Node::MathExpression{children});
      } else {
        compiled.append(&mut children);
      }
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
        // If the node is a naked expression, modify the
        // graph to put it into an anonymous table
        match node {
          Node::Token{..} => (),
          Node::SelectData{..} => {
            children.push(Node::Expression{
              children: vec![Node::AnonymousTableDefine{
                children: vec![Node::TableRow{
                  children: vec![Node::TableColumn{
                    children: vec![node]}]}]}]});
          },
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
          Node::SelectData{..} => {
            children.push(Node::Expression{
              children: vec![Node::AnonymousTableDefine{
                children: vec![Node::TableRow{
                  children: vec![Node::TableColumn{
                    children: vec![node]}]}]}]});
          },
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
      let mut result = self.compile_nodes(children);
      compiled.push(Node::DotIndex{children: result});
    },
    parser::Node::SubscriptIndex{children} => {
      let result = self.compile_nodes(children);
      let mut children: Vec<Node> = Vec::new();
      for node in result {
        match node {
          Node::Token{token, byte} => {
            match token {
              Token::Tilde => {
                children.push(Node::WheneverIndex{children: vec![Node::Null]});
              }
              _ => (),
            }
          },
          _ => children.push(node),
        }
      }
      compiled.push(Node::SubscriptIndex{children});
    },
    parser::Node::Table{children} => {
      let result = self.compile_nodes(children);
      match &result[0] {
        Node::Identifier{name, id} => {
          compiled.push(Node::Table{name: name.to_string(), id: *id});
        },
        _ => (),
      };
    },
    // Quantities
    parser::Node::Quantity{children} => {
      let mut result = self.compile_nodes(children);
      let mut unit = None;
      let mut str_result = "".to_string();
      for node in result {
        match node {
          Node::Identifier{name: word, id} => unit = Some(word),
          Node::String{text} => {
            str_result = format!("{}{}",str_result,text);
          }
          _ => (),
        }
      }
      let float_value: f32 = str_result.parse::<f32>().unwrap();
      compiled.push(Node::Quantity{value: Value::from_f32(float_value), unit});
    },
    parser::Node::Number{children} => {
      let mut result = self.compile_nodes(children);
      let mut str_result = "".to_string();
      for node in result {
        match node {
          Node::Token{token: Token::Comma, byte} => (),
          Node::Token{token, byte} => {
            let digit = byte_to_digit(byte).unwrap();
            str_result = format!("{}{}",str_result,digit);
          },
          _ => (),
        }
      }
      compiled.push(Node::String{text: str_result});
    },
    parser::Node::FloatingPoint{children} => {
      let mut result = self.compile_nodes(children);
      let mut str_result = ".".to_string();
      for node in result {
        match node {
          Node::Token{token: Token::Period, byte} => (),
          Node::Token{token, byte} => {
            let digit = byte_to_digit(byte).unwrap();
            str_result = format!("{}{}",str_result,digit);
          },
          _ => (),
        }
      }
      compiled.push(Node::String{text: str_result});
    },
    // String-like nodes
    parser::Node::ParagraphText{children} => {
      let mut result = self.compile_nodes(children);
      let mut paragraph = "".to_string();
      for node in result {
        match &node {
          Node::String{text} => {paragraph = paragraph + text;},
          _ => (),
        };
      }

      let node = Node::ParagraphText{text: paragraph.clone()};
      compiled.push(node);
    },
    parser::Node::InlineCode{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::InlineCode{children: result});
    },
    parser::Node::CodeBlock{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::CodeBlock{children: result});
    },
    parser::Node::MechCodeBlock{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::MechCodeBlock{children: result});
    },
    parser::Node::Comment{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::Comment{children: result});
    },
    parser::Node::InlineMechCode{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::InlineMechCode{children: result});
    },
    parser::Node::Paragraph{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::Paragraph{children: result});
    },
    parser::Node::UnorderedList{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::UnorderedList{children: result});
    },
    parser::Node::ListItem{children} => {
      let result = self.compile_nodes(children);
      compiled.push(Node::ListItem{children: result});
    },
    parser::Node::Title{children} => {
      let mut result = self.compile_nodes(children);
      let node = match &result[0] {
        Node::String{text} => Node::Title{text: text.clone()},
        _ => Node::Null,
      };
      compiled.push(node);
    },
    parser::Node::Subtitle{children} => {
      let mut result = self.compile_nodes(children);
      let node = match &result[0] {
        Node::String{text} => Node::Title{text: text.clone()},
        _ => Node::Null,
      };
      compiled.push(node);
    },
    parser::Node::SectionTitle{children} => {
      let mut result = self.compile_nodes(children);
      let node = match &result[0] {
        Node::String{text} => Node::SectionTitle{text: text.clone()},
        _ => Node::Null,
      };
      compiled.push(node);
    },
    parser::Node::FormattedText{children} |
    parser::Node::Text{children} => {
      let mut result = self.compile_nodes(children);
      let mut text_node = String::new();
      for node in result {
        match node {
          Node::String{text} => text_node.push_str(&text),
          Node::Token{token, byte} => text_node.push_str(&format!("{}",byte_to_char(byte).unwrap())),
          Node::Quantity{value, unit} => text_node.push_str(&format!("{}", value.to_f32())),
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
          Node::String{text} => word.push_str(&text),
          Node::Quantity{value, unit} => word.push_str(&format!("{}", value.to_f32())),
          _ => compiled.push(node),
        }
      }
      let id = hash_string(&word);
      compiled.push(Node::Identifier{name: word, id});
    },
    // Math
    parser::Node::L0{children} |
    parser::Node::L1{children} |
    parser::Node::L2{children} |
    parser::Node::L3{children} |
    parser::Node::L4{children} |
    parser::Node::L5{children} |
    parser::Node::L6{children} => {
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
    parser::Node::L0Infix{children} |
    parser::Node::L1Infix{children} |
    parser::Node::L2Infix{children} |
    parser::Node::L3Infix{children} |
    parser::Node::L4Infix{children} |
    parser::Node::L5Infix{children} => {
      let result = self.compile_nodes(children);
      let operator = &result[0].clone();
      let input = &result[1].clone();
      let name: String = match operator {
        Node::Add => "math/add".to_string(),
        Node::Subtract => "math/subtract".to_string(),
        Node::Multiply => "math/multiply".to_string(),
        Node::Divide => "math/divide".to_string(),
        Node::Exponent => "math/exponent".to_string(),
        Node::GreaterThan => "compare/greater-than".to_string(),
        Node::GreaterThanEqual => "compare/greater-than-equal".to_string(),
        Node::LessThanEqual => "compare/less-than-equal".to_string(),
        Node::LessThan => "compare/less-than".to_string(),
        Node::Equal => "compare/equal".to_string(),
        Node::NotEqual => "compare/not-equal".to_string(),
        Node::Range => "table/range".to_string(),
        Node::And => "logic/and".to_string(),
        Node::Or => "logic/or".to_string(),
        Node::Xor => "logic/xor".to_string(),
        Node::Token{token, byte} => byte_to_char(*byte).unwrap().to_string(),
        _ => String::from(""),
      };
      compiled.push(Node::Function{name, children: vec![input.clone()]});
    },
    parser::Node::Function{children} => {
      let mut result = self.compile_nodes(children);
      let mut children: Vec<Node> = Vec::new();
      let mut function_name: String = "".to_string();
      for node in result {
        match node {
          Node::Token{..} => (),
          Node::Identifier{name, id} => function_name = name,
          _ => children.push(node),
        }
      }
      compiled.push(Node::Function{name: function_name, children: children.clone()});
    },
    parser::Node::Negation{children} => {
      let mut result = self.compile_nodes(children);
      let mut input = vec![Node::Quantity{value: 0, unit: None}];
      input.push(result[0].clone());
      compiled.push(Node::Function{ name: "math/subtract".to_string(), children: input });
    },
    parser::Node::Not{children} => {
      let mut result = self.compile_nodes(children);
      let mut input = vec![Node::Quantity{value: Value::from_bool(true), unit: None}];
      input.push(result[0].clone());
      compiled.push(Node::Function{ name: "logic/xor".to_string(), children: input });
    },
    parser::Node::String{children} => {
      let mut result = self.compile_nodes(children);
      let string = if result.len() > 0 {
        result[0].clone()
      } else {
        Node::String{text: String::new()}
      };
      compiled.push(string);
    },
    parser::Node::NumberLiteral{children} => {
      let mut result = self.compile_nodes(children);
      compiled.push(result[0].clone());
    },
    parser::Node::RationalNumber{children} => {
      let mut result = self.compile_nodes(children);
      compiled.push(Node::RationalNumber{children: result});
    },
    parser::Node::DecimalLiteral{bytes} => {
      let dec_bytes: Vec<u8> = bytes.iter().map(|b| {
        match b {
          48 => 0,
          49 => 1,
          50 => 2,
          51 => 3,
          52 => 4,
          53 => 5,
          54 => 6,
          55 => 7,
          56 => 8,
          57 => 9,
          _ => 0,        // TODO: ERROR
        }
      }).collect::<Vec<u8>>();
      compiled.push(Node::NumberLiteral{kind: NumberLiteralKind::Decimal, bytes: dec_bytes});
    },
    parser::Node::BinaryLiteral{bytes} => {
      let bin_bytes: Vec<u8> = bytes.iter().map(|b| {
        match b {
          48 => 0,
          49 => 1,
          _ => 0,        // TODO: ERROR
        }
      }).collect::<Vec<u8>>();
      compiled.push(Node::NumberLiteral{kind: NumberLiteralKind::Binary, bytes: bin_bytes});
    }
    parser::Node::OctalLiteral{bytes} => {
      let oct_bytes: Vec<u8> = bytes.iter().map(|b| {
        match b {
          48 => 0,
          49 => 1,
          50 => 2,
          51 => 3,
          52 => 4,
          53 => 5,
          54 => 6,
          55 => 7,
          _ => 0,        // TODO: ERROR
        }
      }).collect::<Vec<u8>>();
      compiled.push(Node::NumberLiteral{kind: NumberLiteralKind::Octal, bytes: oct_bytes});
    }
    parser::Node::HexadecimalLiteral{bytes} => {
      let hex_bytes: Vec<u8> = bytes.iter().map(|b| {
        match b {
          48 => 0,
          49 => 1,
          50 => 2,
          51 => 3,
          52 => 4,
          53 => 5,
          54 => 6,
          55 => 7,
          56 => 8,
          57 => 9,
          65 | 97 => 10, // A
          66 | 98 => 11, // B
          67 | 99 => 12, // C
          68 | 100 => 13,// D
          69 | 101 => 14,// E
          70 | 102 => 15,// F
          _ => 0,        // TODO: ERROR
        }
      }).collect::<Vec<u8>>();
      compiled.push(Node::NumberLiteral{kind: NumberLiteralKind::Hexadecimal, bytes: hex_bytes});
    },
    parser::Node::True => {
      compiled.push(Node::True);
    },
    parser::Node::Null => {
    },
    parser::Node::False => {
      compiled.push(Node::False);
    },
    parser::Node::ParentheticalExpression{children} => {
      let mut result = self.compile_nodes(children);
      compiled.push(result[0].clone());
    },
    parser::Node::GreaterThan => compiled.push(Node::GreaterThan),
    parser::Node::LessThan => compiled.push(Node::LessThan),
    parser::Node::GreaterThanEqual => compiled.push(Node::GreaterThanEqual),
    parser::Node::LessThanEqual => compiled.push(Node::LessThanEqual),
    parser::Node::Equal => compiled.push(Node::Equal),
    parser::Node::NotEqual => compiled.push(Node::NotEqual),
    parser::Node::Add => compiled.push(Node::Add),
    parser::Node::Range => compiled.push(Node::Range),
    parser::Node::Subtract => compiled.push(Node::Subtract),
    parser::Node::Multiply => compiled.push(Node::Multiply),
    parser::Node::Divide => compiled.push(Node::Divide),
    parser::Node::Exponent => compiled.push(Node::Exponent),
    parser::Node::And => compiled.push(Node::And),
    parser::Node::Or => compiled.push(Node::Or),
    parser::Node::Xor => compiled.push(Node::Xor),
    parser::Node::Comparator{children} => {
      match children[0] {
        parser::Node::LessThan => compiled.push(Node::LessThan),
        parser::Node::LessThanEqual => compiled.push(Node::LessThanEqual),
        parser::Node::GreaterThanEqual => compiled.push(Node::GreaterThanEqual),
        parser::Node::Equal => compiled.push(Node::Equal),
        parser::Node::NotEqual => compiled.push(Node::NotEqual),
        parser::Node::GreaterThan => compiled.push(Node::GreaterThan),
        _ => (),
      }
    },
    parser::Node::LogicOperator{children} => {
      match children[0] {
        parser::Node::And => compiled.push(Node::And),
        parser::Node::Or => compiled.push(Node::Or),
        parser::Node::Xor => compiled.push(Node::Xor),
        _ => (),
      }
    },
    // Pass through nodes. These will just be omitted
    parser::Node::Constant{children} |
    parser::Node::StateMachine{children} |
    parser::Node::StateTransition{children} |
    parser::Node::Body{children} |
    parser::Node::Punctuation{children} |
    parser::Node::DigitOrComma{children} |
    parser::Node::Comment{children} |
    parser::Node::Any{children} |
    parser::Node::Symbol{children} |
    parser::Node::AddOperator{children} |
    parser::Node::LogicOperator{children} |
    parser::Node::Subscript{children} |
    parser::Node::DataOrConstant{children} |
    parser::Node::SpaceOrTab{children} |
    parser::Node::Whitespace{children} |
    parser::Node::NewLine{children} |
    parser::Node::Attribute{children} |
    parser::Node::Comparator{children} |
    parser::Node::IdentifierOrConstant{children} |
    parser::Node::ProseOrCode{children}|
    parser::Node::StatementOrExpression{children} |
    parser::Node::WatchOperator{children} |
    parser::Node::Quantity{children} |
    parser::Node::SetOperator{children} |
    parser::Node::Repeat{children} |
    parser::Node::Alphanumeric{children} |
    parser::Node::IdentifierCharacter{children} => {
      compiled.append(&mut self.compile_nodes(children));
    },
    parser::Node::Token{token, byte} => {
      match token {
        Token::Newline => {
          self.current_line += 1;
          self.current_col = 1;
          self.current_char += 1;
        },
        Token::EndOfStream => (),
        _ => {
          self.current_char += 1;
          self.current_col += 1;
        }
      }
      compiled.push(Node::Token{token, byte});
    },
    _ => println!("Unhandled Parser Node in Compiler: {:?}", node),
  }

  //self.constraints = constraints.clone();
  compiled
}