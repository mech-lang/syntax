#[macro_use]
extern crate mech_syntax;
extern crate mech_core;

use mech_syntax::lexer::Lexer;
use mech_syntax::parser::{Parser, ParseStatus, Node};
use mech_syntax::compiler::Compiler;
use mech_core::{Hasher, Core};

macro_rules! compile_string {
  ($func:ident, $test:tt) => (
    #[test]
    fn $func() {
      let mut compiler = Compiler::new();
      let input = String::from($test);
      compiler.compile_string(input);
      assert_eq!(compiler.errors.is_empty(), true);
    }
  )
}

macro_rules! test_math {
  ($func:ident, $input:tt, $test:tt) => (
    #[test]
    fn $func() {
      let mut compiler = Compiler::new();
      let mut core = Core::new(10, 10);
      let input = String::from($input);
      compiler.compile_string(input);
      core.register_blocks(compiler.blocks);
      core.step();
      let table = Hasher::hash_str("test");
      let row = 1;
      let column = 1;
      let test = $test;
      assert_eq!(core.index(table, row, column).unwrap().as_u64().unwrap(), test);
    }
  )
}

compile_string!(empty, "");

// ## Constant

compile_string!(constant_digit, "1");

// ## Table

compile_string!(table, "#table");
compile_string!(table_define, "#table = [x y z]");
compile_string!(table_define_data, "#table = [x y z
                                              1 2 3]");
compile_string!(table_define_data_math, "#table = [x      y          z
                                                   1 * 2, 4 + 7 * 9, 3]");
//compile_string!(table_index_bracket_index, "#table[1]");
//compile_string!(table_index_dot_index_name, "#table.field");


test_math!(table_define_program, "# A Working Program

## Section Two

  #test = 9", 9);

test_math!(math_constant,"#test = 10", 10);
test_math!(math_add,"#test = 1 + 1", 2);
test_math!(math_multiply,"#test = 2 * 2", 4);
test_math!(math_divide,"#test = 4 / 2", 2);
test_math!(math_multiple_variable_graph,"block
  a = z * 5
  #test = d * z + a
  d = 9 * z
  z = 5", 250);
test_math!(math_multiple_variable_graph_new_ordering,"block
  #test = d * z + a
  a = z * 5
  z = 5
  d = 9 * z", 250);
test_math!(math_select_table,"  
block
  #x = 500
block
  #test = #x", 500);
test_math!(math_select_table_reverse_ordering,"  
block
  #test = #x
block
  #x = 500", 500);
test_math!(math_select_column_by_id,"  
block
  #ball = [x: 56 y: 2 vx: 3 vy: 4]
block
  #test = #ball.x", 56);
test_math!(math_multiple_rows_select,"
block
  #ball = [x: 15 y: 9 vx: 18 vy: 0]
block
  #test = #ball.x + #ball.y * #ball.vx", 177);
test_math!(math_const_and_select,"
block
  #ball = [x: 15 y: 9 vx: 18 vy: 0]
block
  #test = 9 + #ball.x", 24);
test_math!(math_select_and_const,"
block
  #ball = [x: 15 y: 9 vx: 18 vy: 0]
block
  #test = #ball.x + 9", 24);
test_math!(math_partial_bouncing_ball,"# Bouncing Balls
Define the environment
  #ball = [x: 15 y: 9 vx: 18 vy: 9]
  #system/timer = [resolution: 1000]
  #gravity = 10

Now update the block positions
  x = #ball.x + #ball.vx
  y = #ball.y + #ball.vy
  dt = #system/timer.resolution
  #test = x + y * dt", 18033);
test_math!(math_add_columns,"
block
  #ball = [x y
           1 2
           3 4
           5 6]
block
  #test = #ball.x + #ball.y", 3);
test_math!(math_add_matrices,"
block
  x = [1 2 3
       4 5 6
       7 8 9]
  y = [10 11 12
       13 14 15
       16 17 18]
  #test = x + y", 11);