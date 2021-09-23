use mech_syntax::parser::Parser;
use mech_syntax::ast::Ast;
use mech_syntax::compiler::Compiler;
use mech_core::Core;

fn main() {

  let mut parser = Parser::new();
  let mut ast = Ast::new();
  let mut compiler = Compiler::new();
  let mut core = Core::new();

  parser.parse("block 
  😃 = 1
  🤦🏼‍♂️ = 2
  y̆és = 🤦🏼‍♂️ + 😃
  #test = y̆és");

  println!("{:#?}", parser.parse_tree);

  ast.build_syntax_tree(&parser.parse_tree);

  println!("{:?}", ast.syntax_tree);

  let blocks = compiler.compile_blocks(&vec![ast.syntax_tree.clone()]);

  core.insert_block(blocks[0].clone());
  
  for t in blocks {
    println!("{:#?}", t);
  }

  println!("{:#?}", core);

  println!("{:#?}", core.get_table("test").unwrap().borrow().get(0, 0));

}