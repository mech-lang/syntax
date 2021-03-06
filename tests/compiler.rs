#[macro_use]
extern crate mech_syntax;
extern crate mech_core;

use mech_syntax::parser::{Parser, Node};
use mech_syntax::compiler::Compiler;
use mech_core::{hash_string, Core, Index, Value, ValueMethods, make_quantity};

macro_rules! test_mech {
  ($func:ident, $input:tt, $test:expr) => (
    #[test]
    fn $func() {
      let mut compiler = Compiler::new();
      let mut core = Core::new(100);
      core.load_standard_library();
      let input = String::from($input);
      compiler.compile_string(input);
      core.runtime.register_blocks(compiler.blocks);
      let table = hash_string("test");
      let row = Index::Index(1);
      let column = Index::Index(1);
      let test: Value = $test;
      let actual = core.get_cell_in_table(table, &row, &column);
      match actual {
        Some(value) => {
          assert_eq!(value, test);
        },
        None => assert_eq!(0,1),
      }
    }
  )
}

// ## Constant

test_mech!(constant_empty, "
block
  x = [1 2
       4 _
       _ 7]
  #test = stats/sum(column: x{:,1})",Value::from_i64(5));

  test_mech!(constant_empty_table, "
block
  #test = _", Value::empty());

test_mech!(constant_inline_empty, "#test = [first: 123, second: _, third: 456]",Value::from_i64(123));

// ## Table

test_mech!(table_define_inline_expressions, "
block
  #x = [x: 1 + 2, y: 2 + 2]
block
  #test = #x.x + #x.y", Value::from_i64(7));

test_mech!(table_anonymous_table_trailing_whitespace, "
block
  #test = [|d|
            5  ]", Value::from_i64(5));

test_mech!(table_define_program, "# A Working Program

## Section Two

  #test = 9", Value::from_i64(9));

// ## Select

test_mech!(select_table,"  
block
  #x = 500
block
  #test = #x", Value::from_i64(500));

test_mech!(select_table_reverse_ordering,"  
block
  #test = #x
block
  #x = 500", Value::from_i64(500));

// ## Math

test_mech!(math_constant,"#test = 10", Value::from_i64(10));

test_mech!(math_add,"#test = 1 + 1", Value::from_i64(2));

test_mech!(math_subtract,"#test = 1 - 1", Value::from_i64(0));

test_mech!(math_multiply,"#test = 2 * 2", Value::from_i64(4));

test_mech!(math_divide,"#test = 4 / 2", Value::from_quantity(make_quantity(20000,-4,0)));

test_mech!(math_two_terms,"#test = 1 + 2 * 9", Value::from_i64(19));

test_mech!(math_constant_collision,"#test = 10000 + 1", Value::from_i64(10001));

test_mech!(math_multiple_variable_graph,"block
  a = z * 5
  #test = d * z + a
  d = 9 * z
  z = 5", Value::from_i64(250));

test_mech!(math_multiple_variable_graph_new_ordering,"block
  #test = d * z + a
  a = z * 5
  z = 5
  d = 9 * z", Value::from_i64(250));

  test_mech!(math_add_columns_alias,"
block
  #test = stats/sum(column: #ball.x + #ball.y)

block
  x = 1:10
  y = 1:10
  #ball = [|x y|
            x y]", Value::from_i64(110));

  test_mech!(math_add_columns_indices,"
block
  #test = stats/sum(column: #ball{:,1} + #ball{:,2})

block
  x = 1:10
  y = 1:10
  #ball = [|x y|
            x y]", Value::from_i64(110));

test_mech!(math_on_whole_table,"
block
  #x = 500
block
  #test = #x + 5", Value::from_i64(505));

test_mech!(select_column_by_id,"  
block
  #ball = [x: 56 y: 2 vx: 3 vy: 4]
block
  #test = #ball.y", Value::from_i64(2));

test_mech!(math_multiple_rows_select,"
block
  #ball = [x: 15 y: 9 vx: 18 vy: 0]
block
  #test = #ball.x + #ball.y * #ball.vx", Value::from_i64(177));

test_mech!(math_const_and_select,"
block
  #ball = [x: 15 y: 9 vx: 18 vy: 0]
block
  #test = 9 + #ball.x", Value::from_i64(24));

test_mech!(math_select_and_const,"
block
  #ball = [x: 15 y: 9 vx: 18 vy: 0]
block
  #test = #ball.x + 9", Value::from_i64(24));

test_mech!(partial_bouncing_ball,"# Bouncing Balls
Define the environment
  #ball = [x: 15 y: 9 vx: 18 vy: 9]
  #time/timer = [period: 1000]
  #gravity = 10

Now update the block positions
  x = #ball.x + #ball.vx
  y = #ball.y + #ball.vy
  dt = #time/timer.period
  #test = x + y * dt", Value::from_i64(18033));

test_mech!(math_add_columns,"
block
  #ball = [|x y|
            1 2
            3 4
            5 6]
block
  #test = #ball.x + #ball.y", Value::from_i64(3));

test_mech!(math_add_matrices,"
block
  x = [1 2
       4 5]
  y = [10 11
       13 14]
  z = x + y
  #test = z{1,1} + z{1,2} + z{2,1} + z{2,2}", Value::from_i64(60));

test_mech!(math_scalar_plus_vector,"
block
  x = 3:6
  y = 5 + x
  #test = y{1,1} + y{2,1} + y{3,1} + y{4,1}", Value::from_i64(38));

test_mech!(math_vector_plus_scalar_inline,"
block
  #x = [1 2 3] + 1
  
block
  #test = stats/sum(row: #x)", Value::from_i64(9));

test_mech!(math_vector_plus_scalar_inline_reverse,"
block
  #x = 1 + [1 2 3]
    
block
  #test = stats/sum(row: #x)", Value::from_i64(9));

test_mech!(math_vector_plus_scalar,"
block
  x = [1 2 3]
  #x = x + 1

block
  #test = stats/sum(row: #x)", Value::from_i64(9));

test_mech!(math_negation_double_negative,"
block
  y = -13
  #test = -y", Value::from_i64(13));

test_mech!(math_parenthetical_expression_constants,"
block
  #test = (1 + 2) * 3", Value::from_i64(9));

// ## Units

test_mech!(units_basic_math,"#test = 35g + 10g", Value::from_i64(45));

test_mech!(units_scaling,"#test = 35g + 10kg", Value::from_i64(10035));

// ## Ranges

test_mech!(range_basic,r#"
block
  #test = stats/sum(column: #range)
block
  #range = 5 : 14"#, Value::from_i64(95));

test_mech!(range_and_cat,r#"
block
  #test = stats/sum(table: #ball)

block
  x = 1:4000
  y = 1:4000
  #ball = [x y]"#, Value::from_i64(16004000));

// ## Subscripts

test_mech!(subscript_scalar_math,"
block
  x = 3:6
  y = 10:12
  #test = x{1,1} + y{3,1}", Value::from_i64(15));

test_mech!(subscript_scan,"
block
  x = 10:20
  z = 3:5
  #test = x{z, :}", Value::from_i64(12));

test_mech!(subscript_single_horz,"
block
  x = [1 2 3]
  #test = x{2}", Value::from_i64(2));

test_mech!(subscript_single_vert,"
block
  x = [1; 2; 3]
  #test = x{2}", Value::from_i64(2));

// ## Comparators

test_mech!(comparator_greater,"
block
  x = 10:20
  z = x > 15
  #test = x{z, :}", Value::from_i64(16));

test_mech!(comparator_less,"
block
  x = 10:20
  z = x < 15
  #test = x{z, :}", Value::from_i64(10));

test_mech!(comparator_greater_than_equal,"
block
  #x = [1; 2; 3]
  #y = [2; 1; 3]
  
block
  ix = #x >= #y
  #test = stats/sum(column: #x{ix,:})", Value::from_i64(5)); 

test_mech!(comparator_less_than_equal,"
block
  #x = [1; 2; 3]
  #y = [2; 1; 3]
  
block
  ix = #x <= #y
  #test = stats/sum(column: #x{ix,:})", Value::from_i64(4)); 


test_mech!(comparator_equal,"
block
  #x = [1; 2; 3; 2]
  #y = [2; 1; 3; 2]
  
block
  ix = #x == #y
  #test = stats/sum(column: #x{ix,:})", Value::from_i64(5)); 

test_mech!(comparator_equal_string,r#"
block
  #x = [1; 2; 3; 4]
  #y = ["a"; "b"; "a"; "b"]
  
block
  ix = #y == "a"
  #test = stats/sum(column: #x{ix,:})"#, Value::from_i64(4)); 

test_mech!(comparator_not_equal,"
block
  #x = [1; 2; 3; 2]
  #y = [2; 1; 3; 2]
  
block
  ix = #x != #y
  #test = stats/sum(column: #x{ix,:})", Value::from_i64(3)); 

// ## Set

test_mech!(set_column_simple,"
block
  #test.x := 77

block
  #test = [|x|
            9]", Value::from_i64(77));

test_mech!(set_column_alias,"
block
  #test = #ball.x

block
  #ball = [x: 0 y: 0]

block
  ~ #foo.x
  #ball.x := #foo.x

block
  #foo = [x: 100 y: 120]
  #z = 100

block
  ~ #z
  #foo.x := 200", Value::from_i64(200));


test_mech!(set_single_index,"
block
  #x = [400; 0; 0]
 
block 
  #x{3} := 7

block
  #test = stats/sum(column: #x)", Value::from_i64(407));

test_mech!(set_column_logical,"
block
  ix = x > 0
  x = #q.x
  #q.x{ix} := -1

block
  #test = #q.x{1} + #q.x{2} + #q.x{3}

block
  #q = [|x y z|
         1 2 3
         4 5 6
         7 8 9]", Value::from_i64(-3));

test_mech!(set_second_column_logical,"
block
  #test = #ball.y

block
  ix = x > 0
  x = #ball.y
  #ball.y{ix} := 3

block
  #ball = [|x y z|
            1 2 3
            4 5 6
            7 8 9]", Value::from_i64(3));

test_mech!(set_second_omit_row_subscript,"
block
  #ball = [x: 15 y: 9 vx: 40 vy: 9]
  #time/timer = [period: 15 tick: 0]
  #gravity = 2

block
  ~ #time/timer.tick
  #ball.y := #ball.vy + #gravity

block
  #test = #ball.y", Value::from_i64(11));

test_mech!(set_rhs_math_filters_logic,"
block
  #ball = [|x y vx vy|
            1 2 3 4
            5 6 7 8
            9 10 11 12]
  #time/timer = [period: 15 tick: 0]
  #gravity = 2

block
  ix = #ball.vy > 10
  iy = #ball.vy < 5
  ixx = ix | iy
  #ball.y{ixx} := #ball.vy * 9099

block
  #test = #ball{1,2} + #ball{3,2}", Value::from_i64(145584));

test_mech!(set_implicit_logic,"
block
  #ball = [|x y vx vy|
            1 2 3 4
            5 6 7 8
            9 10 11 12]
  #time/timer = [period: 15 tick: 0]
  #gravity = 2

block
  ix = #ball.vy > 10
  iy = #ball.vy < 5
  #ball.y{ix | iy} := #ball.vy * 9099

block
  #test = #ball{1,2} + #ball{3,2}", Value::from_i64(145584));

test_mech!(set_inline_row,"
block
  #test = stats/sum(row: #launch-point)

block
  #launch-point = [x: 0 y: 0]

block
  #launch-point := [x: 10 y: 20]", Value::from_i64(30));

// ## Concat

test_mech!(concat_horzcat_data,"
block
  x = 1:10
  y = 11:20
  #z = [x y]
  
block
  #test = #z{1,1} + #z{1,2} + #z{2,1} + #z{1,1}", Value::from_i64(15));

test_mech!(concat_horzcat_autofill,r#"
block
  #test = stats/sum(column: #y.type)

block
  x = ["a"; "b"; "c"; "d"]
  #y = [type: 1 class: "table" result: x]"#, Value::from_i64(4));

// ## Append

test_mech!(append_row_inline,"
block
  ix = #foo.x > 50
  #test = #foo{ix, :}

block
  ~ #z.x
  y = #z
  #foo += [x: 100 y: 110 z: 120]

block
  x = #ball.y
  #z = [x: 123 y: 456]
  #foo = [|x y z|
           5 6 7
           8 9 10
           11 12 13]

block
  #ball = [|x y z|
            1 2 3]", Value::from_i64(100));

// ## Logic

test_mech!(logic_and,"
block
  ix1 = #foo.x > 5
  ix2 = #foo.x < 11
  ix3 = ix1 & ix2
  #test = #foo{ix3, 1}

block
  #foo = [|x y z|
           5 6 7
           8 9 10
           11 12 13]", Value::from_i64(8));

test_mech!(logic_and_filter_inline,"
block
  ix = #foo.x > 5 & #foo.x < 11
  #test = #foo{ix, 1}

block
  #foo = [|x y z|
           5 6 7
           8 9 10
           11 12 13]", Value::from_i64(8));

test_mech!(logic_and_composed,"
block
  ix = #foo.x > 5 & #foo.x < 11 & #foo.y > 9
  #test = #foo{ix, 1}

block
  #foo = [|x y z|
           5 6 7
           8 9 10
           9 10 11
           11 12 13]", Value::from_i64(9));

test_mech!(logic_or,"
block
  ix1 = #foo.x < 7
  ix2 = #foo.x > 9
  ix3 = ix1 | ix2
  q = #foo{ix3, 1}
  #test = q{1,1} + q{2,1}

block
  #foo = [|x y z|
           5 6 7
           8 9 10
           11 12 13]", Value::from_i64(16));

// ## Change scan

test_mech!(change_scan_column,"block
  #time/timer = [tick: 0]

block
  ~ #time/timer.tick
  #test = 3", Value::from_i64(3));

test_mech!(change_scan_simple2,"block
  #i = 2
  #x = [400; 0; 0]
 
block
  #test = stats/sum(column: #x)

block 
  ~ #i
  i = #i
  #x{i,:} := #x{i - 1,:} + 1", Value::from_i64(801));

test_mech!(change_scan_simple,"block
  #i = 2
  #test = 10

block 
  ~ #i
  #test := 20", Value::from_i64(20));


test_mech!(change_scan_equality,"block
  #test = #q * 3
  ~ #q == 10

block
  #q = 10", Value::from_i64(30));

test_mech!(change_scan_inequality,"block
  #test := #q * 3
  ~ #q < 20

block
  #test = 10
  #q = 10", Value::from_i64(30));

test_mech!(change_scan_recursive,r#"
block
  ~ #html/event/keydown.key == "ArrowUp"
  #explorer.y := #explorer.y - 1"

block
  #explorer = [x: 10, y: 10]"

block
  ~ #explorer.x
  #html/event/keydown = [key: "ArrowUp"]

block
  ~ #explorer
  #test = #explorer.y"#, Value::from_i64(9));

// ## Full programs

/*test_mech!(program_bouncing_balls,"# Bouncing Balls

Define the environment
  #html/event/click = [|x y|]
  #ball = [x: 50 y: 9 vx: 40 vy: 9]
  #time/timer = [period: 15, tick: 0]
  #gravity = 2
  #boundary = [x: 60 y: 60]

## Update condition

Now update the block positions
  ~ #time/timer.tick
  #ball.x := #ball.x + #ball.vx
  #ball.y := #ball.y + #ball.vy
  #ball.vy := #ball.vy + #gravity

## Boundary Condition

Keep the balls within the y boundary
  ~ #ball.y
  iy = #ball.y > #boundary.y
  #ball.y{iy} := #boundary.y
  #ball.vy{iy} := -#ball.vy * 80 / 100

Keep the balls within the x boundary
  ~ #ball.x
  ix = #ball.x > #boundary.x
  ixx = #ball.x < 0
  #ball.x{ix} := #boundary.x
  #ball.x{ixx} := 0
  #ball.vx{ix | ixx} := -#ball.vx * 80 / 100

## Create More Balls

Create ball on click
  ~ #html/event/click.x
  #ball += [x: 10 y: 10 vx: 40 vy: 0]

test
  x = #ball.x + #ball.y
  #test = stats/sum(column: x)", Value::from_quantity(make_quantity(98,0,0)));*/

// ## Strings

test_mech!(string_basic,r#"
block
  #test = "Hello World""#, Value::from_str("Hello World"));

test_mech!(string_table,r#"
block
  #test = ["Hello" "World"]"#, Value::from_str("Hello"));

test_mech!(string_empty,r#"
block
  #test = ["" "World"]"#, Value::from_str(""));

test_mech!(string_named_attributes, r#"#test = [type: "h1" text: "An App"]"#, Value::from_str("h1"));

// ## Nesting

test_mech!(nesting_basic,r#"
block
  #test = #app{1,2}{1,2}

block
  #app = [2 [5 7]]"#, Value::from_u64(7));


test_mech!(nesting_triple,r#"
block
  #test = [#app{2,2}{1,2}{1,1}]

block
  x = 314
  container = [|type text| 
                123   [x]]
  #app = [|direction contains| 
           "column"  [container]
           "row"     [container]]"#, Value::from_u64(314));

test_mech!(deep_nesting,r#"
block
  #test = stats/sum(row: #app/main{1}{1,2}{1,:})

block
  #ball = [x: 10 y: 10]

block
  ball = [shape: "circle" parameters: [cx: 123 cy: 456]]
  line = [shape: "line" parameters: [x1: #ball.x, x2: #ball.y]]
  canvas = [contains: [ball; line]]
  #app/main = [contains: [canvas]]"#, Value::from_u64(579));

test_mech!(nesting_math,r#"
block
  #test = #app{2,2}{1,2}{1,1} * 10

block
  x = 314
  container = [|type text| 
                123   [x]]
  #app = [|direction contains| 
           "column"  [container]
           "row"     [container]]"#, Value::from_u64(3140));

test_mech!(nesting_math_select_range,r#"
block
  x = #app{2,2}{1,2}{:,1} * 10
  y = x{2,1}
  z = x{3,1}
  #test = y + z

block
  x = 1:10
  container = [|type text| 
                123   [x]]
  #app = [|direction contains| 
           "column"  [container]
           "row"     [container]]"#, Value::from_u64(50));

// ## Functions

test_mech!(function_stats_sum,r#"
block
  x = [1;2;3;4;5]
  #test = stats/sum(column: x)"#, Value::from_quantity(make_quantity(15,0,0)));

test_mech!(function_stats_sum_row,r#"
block
  x = [1 2 3 4 5]
  #test = stats/sum(row: x)"#, Value::from_quantity(make_quantity(15,0,0)));

test_mech!(function_stats_sum_row_col,r#"
block
  x = [1;2;3;4;5]
  y = stats/sum(row: x)
  #test = y{1} + y{2} + y{3} + y{4} + y{5}"#, Value::from_quantity(make_quantity(15,0,0)));

test_mech!(function_stats_sum_table,r#"
block
  x = [1 2 3; 4 5 6]
  #test = stats/sum(table: x)"#, Value::from_quantity(make_quantity(21,0,0)));

test_mech!(function_add_functions,r#"
block
  x = [1 2
       4 _
       _ 7]
  #test = stats/sum(column: x{:,1}) + stats/sum(column: x{:,2})"#, Value::from_i64(14));

test_mech!(function_set_any,r#"
block
  x = [1; 2; 3; 4; 5]
  y = x > 4
  #test = set/any(column: y)"#, Value::from_bool(true));

test_mech!(function_set_any_false,r#"
block
  x = [1; 2; 3; 4; 5]
  y = x > 5
  #test = set/any(column: y)"#, Value::from_bool(false));

test_mech!(function_inline_args,r#"
block
  #test = stats/sum(row: [1 2 3 4])"#, Value::from_u64(10));

test_mech!(function_inline_colum_args,r#"
block
  #test = stats/sum(column: [1; 2; 3; 4])"#, Value::from_u64(10));
  
// ## Errors

test_mech!(error_duplicate_alias, r#"
block
  #test = 5

block
  x = 1
  x = 3
  #test := 7"#, Value::from_i64(5));

// ## Markdown

test_mech!(markdown_program_title, r#"# Title
  #test = 123"#, Value::from_i64(123));

test_mech!(markdown_no_program_title, r#"paragraph
  #test = 123"#, Value::from_i64(123));

test_mech!(markdown_section_title, r#"# Title

Paragraph

## Section

  #test = 123"#, Value::from_i64(123));

test_mech!(markdown_inline_code, r#"# Title

Paragraph including `inline code`

## Section

  #test = 123"#, Value::from_i64(123));

test_mech!(markdown_list, r#"# Title

Paragraph including `inline code`

## Section

- Item 1
- Item 2
- Item 3

  #test = 123"#, Value::from_i64(123));

test_mech!(markdown_list_inline_code, r#"# Title

Paragraph including `inline code`

## Section

- Item `some code`
- Item `some code`
- Item `some code`

  #test = 123"#, Value::from_i64(123));

test_mech!(markdown_code_block, r#"# Title

Paragraph including `inline code`

## Section

```
A regular code block
```

  #test = 123"#, Value::from_i64(123));

// ## Mechdown (Markdown extensions for Mech)

test_mech!(mechdown_inline_mech_code, r#"# Title

Paragraph including `inline mech code` is [[#test]]

## Section

  #test = 123"#, Value::from_i64(123));

test_mech!(mechdown_block_directives, r#"
block
  #test = 1

```mech:disabled
  #test := 2
```
"#, Value::from_i64(1));

// ## Comments

test_mech!(comment_line, r#"
block
  // This is a comment
  #test = 123"#, Value::from_i64(123));

// ## Recursion

test_mech!(recursive_blocks, r#"
block
  #test = #i

block
  #i = [x: 2]

block
  #i.x{#i <= 6} := #i.x + 1"#, Value::from_u64(7));

// ## Table split

test_mech!(table_split, r#"
block
  #test = #z{1}{1} + #z{2}{1} + #z{3}{1}
block
  x = [7;8;9]
  y >- x
  #z = y"#, Value::from_u64(24));

test_mech!(table_split_global, r#"
block
  #test = #name-tag{1}{2} + #name-tag{2}{2}

Messages
  #messages = [|name content|
                1    "One"
                2    "Two"]
block
  #name-tag >- [type: "div" content: #messages.name]"#, Value::from_u64(3));


// ## Boolean values

test_mech!(boolean_anonymous_table, r#"
block
  #y = [1 2 3]

block
  #x = [true false true]
  
block
  #z = #y{#x}
  
block
  #test = #z{1} + #z{2}"#, Value::from_u64(4));

  test_mech!(boolean_literal_true, r#"
block
  #test = true"#, Value::from_bool(true));

  test_mech!(boolean_literal_false, r#"
block
  #test = false"#, Value::from_bool(false));

  test_mech!(boolean_literals_and_operator, r#"
block
  x = true
  y = false
  #test = x & y"#, Value::from_bool(false));

// ## Number Literals

  test_mech!(number_literal_decimal, r#"
  #test = 0d1234567890"#, 13902651193305449173);