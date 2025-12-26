Analyzing rust code at: /home/sam/Projects/hec22

Running cargo... found 47 issue(s)
Running clippy... found 59 issue(s)
Running semgrep... ✓
Running cargo-udeps... ✓

Cache: 0 hits, 4 misses

╭──────────────────────────────────────────────────────────────────────────────╮
│ Static Analysis Report - RUST                                                │
╰──────────────────────────────────────────────────────────────────────────────╯
          Summary          
 Total Issues        71    
   Errors            3     
   Warnings          68    
 Duplicates Removed  35    
 Tools Run           4     
 Execution Time      6.33s 


Errors (3):
  ✗ examples/inlet_capacity.rs:91:24 - [E0061] this method takes 5 arguments but
2 arguments were supplied
  ✗ examples/inlet_capacity.rs:99:14 - [E0061] this function takes 4 arguments 
but 2 arguments were supplied
  ✗ examples/inlet_capacity.rs:141:30 - [E0061] this method takes 5 arguments 
but 2 arguments were supplied

Warnings (68):
  ⚠ examples/hydraulic_solver.rs:144:55 - [clippy::cloned_ref_to_slice_refs] 
this call to `clone` can be replaced with `std::slice::from_ref`
  ⚠ examples/inlet_bypass_workflow.rs:12:5 - [unused_imports] unused import: 
`std::collections::HashMap`
  ⚠ examples/inlet_capacity.rs:6:21 - [unused_imports] unused import: 
`GutterFlowResult`
  ⚠ src/csv.rs:23:31 - [unused_imports] unused import: `ConduitType`
  ⚠ src/csv.rs:25:120 - [unused_imports] unused import: `NodeType`
  ⚠ src/csv.rs:26:11 - [unused_imports] unused import: `Reader`
  ⚠ src/csv.rs:397:34 - [clippy::unwrap_or_default] use of `or_insert_with` to 
construct default value
  ⚠ src/gutter.rs:33:5 - [unused_imports] unused import: `std::f64::consts::PI`
  ⚠ src/gutter.rs:272:8 - [dead_code] method `width_ratio` is never used
  ⚠ src/hydraulics.rs:209:33 - [unused_variables] unused variable: `diameter`
  ⚠ src/hydraulics.rs:653:5 - [clippy::too_many_arguments] this function has too
many arguments (10/7)
  ⚠ src/hydraulics.rs:681:9 - [clippy::let_and_return] returning the result of a
`let` binding from a block
  ⚠ src/hydraulics.rs:1038:32 - [clippy::if_same_then_else] this `if` has 
identical blocks
  ⚠ src/hydraulics.rs:1040:39 - [clippy::if_same_then_else] this `if` has 
identical blocks
  ⚠ src/hydraulics.rs:1346:5 - [clippy::too_many_arguments] this function has 
too many arguments (10/7)
  ⚠ src/hydraulics.rs:1958:17 - [clippy::manual_range_contains] manual 
`RangeInclusive::contains` implementation
  ⚠ src/hydraulics.rs:1968:13 - [unused_variables] unused variable: `ratio`
  ⚠ src/inlet.rs:18:21 - [unused_imports] unused imports: `CompositeGutter`, 
`GUTTER_K_US`, and `UniformGutter`
  ⚠ src/inlet.rs:549:9 - [clippy::doc_overindented_list_items] doc list item 
overindented
  ⚠ src/inlet.rs:550:9 - [clippy::doc_overindented_list_items] doc list item 
overindented
  ⚠ src/inlet.rs:624:9 - [clippy::doc_overindented_list_items] doc list item 
overindented
  ⚠ src/main.rs:341:5 - [clippy::single_char_add_str] calling `push_str()` using
a single-character string literal
  ⚠ src/main.rs:411:16 - [clippy::ptr_arg] writing `&PathBuf` instead of `&Path`
involves a new object where a slice will do
  ⚠ src/main.rs:528:22 - [clippy::needless_lifetimes] the following explicit 
lifetimes could be elided: 'a
  ⚠ src/solver.rs:11:5 - [clippy::doc_lazy_continuation] doc list item without 
indentation
  ⚠ src/solver.rs:17:46 - [unused_imports] unused imports: `DrainageAreaResult` 
and `ViolationType`
  ⚠ src/solver.rs:24:17 - [unused_imports] unused imports: `AccessHoleResult`, 
`FlowRegime`, and `PipeFlowResult`
  ⚠ src/solver.rs:32:44 - [unused_imports] unused import: `NodeType`
  ⚠ src/solver.rs:124:13 - [unused_mut] variable does not need to be mutable
  ⚠ src/solver.rs:125:13 - [unused_mut] variable does not need to be mutable
  ⚠ src/solver.rs:283:35 - [clippy::len_zero] length comparison to one
  ⚠ src/solver.rs:289:21 - [clippy::needless_borrow] this expression creates a 
reference which is immediately dereferenced by the compiler
  ⚠ src/solver.rs:291:21 - [clippy::needless_borrow] this expression creates a 
reference which is immediately dereferenced by the compiler
  ⚠ src/solver.rs:302:21 - [clippy::needless_borrow] this expression creates a 
reference which is immediately dereferenced by the compiler
  ⚠ src/solver.rs:439:5 - [clippy::too_many_arguments] this function has too 
many arguments (9/7)
  ⚠ src/solver.rs:647:13 - [unused_variables] unused variable: `upstream_invert`
  ⚠ src/solver.rs:788:9 - [clippy::let_and_return] returning the result of a 
`let` binding from a block
  ⚠ src/solver.rs:818:10 - [clippy::only_used_in_recursion] parameter is only 
used in recursion
  ⚠ src/solver.rs:1249:26 - [unused_imports] unused imports: `PipeMaterial`, 
`PipeProperties`, and `PipeShape`
  ⚠ src/solver.rs:1250:9 - [unused_imports] unused import: 
`crate::node::OutfallProperties`
  ⚠ src/visualization/network_plan.rs:10:19 - [unused_imports] unused import: 
`Node`
  ⚠ src/visualization/profile.rs:71:5 - [dead_code] field `node_path` is never 
read
  ⚠ src/visualization/profile.rs:141:28 - [clippy::type_complexity] very complex
type used. Consider factoring parts into `type` definitions
  ⚠ src/visualization/profile.rs:298:13 - [unused_variables] unused variable: 
`plot_height`
  ⚠ src/visualization/profile.rs:369:14 - [unused_variables] unused variable: 
`i`
  ⚠ src/visualization/profile.rs:530:35 - [unused_imports] unused import: 
`ConduitType`
  ⚠ src/visualization/svg.rs:46:5 - [clippy::too_many_arguments] this function 
has too many arguments (8/7)
  ⚠ tests/chapter5_verification.rs:13:7 - [dead_code] constant `TOLERANCE` is 
never used
  ⚠ tests/grate_sizing_test.rs:118:9 - [clippy::manual_range_contains] manual 
`RangeInclusive::contains` implementation
  ⚠ tests/grate_sizing_test.rs:345:9 - [clippy::manual_range_contains] manual 
`RangeInclusive::contains` implementation
  ⚠ tests/hec22_chapter5_examples.rs:33:9 - [non_snake_case] variable `q_partA` 
should have a snake case name
  ⚠ tests/hec22_chapter5_examples.rs:51:9 - [non_snake_case] variable `t_partB` 
should have a snake case name
  ⚠ tests/hec22_chapter5_examples.rs:92:9 - [non_snake_case] variable `t_partA` 
should have a snake case name
  ⚠ tests/hec22_chapter5_examples.rs:112:9 - [non_snake_case] variable `q_partB`
should have a snake case name
  ⚠ tests/json_schema_tests.rs:86:13 - [clippy::len_zero] length comparison to 
zero
  ⚠ tests/json_schema_tests.rs:104:13 - [clippy::len_zero] length comparison to 
zero
  ⚠ tests/json_schema_tests.rs:105:13 - [clippy::len_zero] length comparison to 
zero
  ⚠ tests/json_schema_tests.rs:106:13 - [clippy::len_zero] length comparison to 
zero
  ⚠ tests/json_schema_tests.rs:114:13 - [clippy::len_zero] length comparison to 
zero
  ⚠ tests/json_schema_tests.rs:277:13 - [clippy::len_zero] length comparison to 
zero
  ⚠ tests/json_schema_tests.rs:278:13 - [clippy::len_zero] length comparison to 
zero
  ⚠ tests/json_schema_tests.rs:335:13 - [clippy::len_zero] length comparison to 
zero
  ⚠ tests/json_schema_tests.rs:339:13 - [clippy::len_zero] length comparison to 
zero
  ⚠ tests/multi_level_tributary_flow_test.rs:26:9 - [unused_variables] unused 
variable: `project`
  ⚠ tests/network_integration_test.rs:426:13 - [clippy::len_zero] length 
comparison to zero
  ⚠ tests/outfall_egl_test.rs:21:9 - [unused_variables] unused variable: 
`project`
  ⚠ tests/outfall_egl_test.rs:113:55 - [clippy::useless_vec] useless use of 
`vec!`
  ⚠ tests/tributary_flow_test.rs:24:9 - [unused_variables] unused variable: 
`project`

Analysis completed with 3 error(s).
