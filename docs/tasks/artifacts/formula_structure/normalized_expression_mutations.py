"""Actual compiled arena/view faults must fail independent public contract assertions."""
from pathlib import Path
import subprocess
ROOT=Path(__file__).resolve().parents[4]
SOURCE=ROOT/'crates/sc-core/src/recipe/normalized.rs'
ORIGINAL=SOURCE.read_bytes()
WORK=ROOT/'target/formula_normalized_mutations'
WORK.mkdir(exist_ok=True)
SHAPE='authored_shapes_match_reference_kind_order_nodes_and_depth'
SPAN='normalized_arena_retains_source_spans_and_borrows_after_syntax_drop'
CALL='argument_views_are_ordered_exact_size_fused_and_arena_bound'
CASES=[
 ('root identity','root: self.root,','root: 0,',SHAPE),
 ('name identity','NodeData::Name(name) => NormalizedData::Name(name),','NodeData::Name(_) => NormalizedData::Name("changed"),',SHAPE),
 ('unary identity','NodeData::Negate(child) => NormalizedData::Negate(*child),','NodeData::Negate(child) => NormalizedData::Square(*child),',SHAPE),
 ('square identity','NodeData::Square(child) => NormalizedData::Square(*child),','NodeData::Square(child) => NormalizedData::Negate(*child),',SHAPE),
 ('binary operator','operator: *operator,\n                    left: *left,','operator: FormulaBinaryOperator::Add,\n                    left: *left,',SHAPE),
 ('binary child order','left: *left,\n                    right: *right,','left: *right,\n                    right: *left,',SHAPE),
 ('call child order','arguments: arguments.clone(),','arguments: arguments.iter().rev().copied().collect(),',SHAPE),
 ('call child loss','arguments: arguments.clone(),','arguments: arguments.iter().take(1).copied().collect(),',SHAPE),
 ('conditional branch order','then_branch: *then_branch,\n                    else_branch: *else_branch,','then_branch: *else_branch,\n                    else_branch: *then_branch,',SHAPE),
 ('conditional condition','condition: *condition,','condition: *then_branch,',SHAPE),
 ('depth metadata','if_depth: self.if_depth,','if_depth: 0,',SHAPE),
 ('source span','span: node.span,','span: FormulaSourceSpan::new(0, 0),',SPAN),
 ('literal unit','super::literal::normalize(number, *unit, node.span)?','super::literal::normalize(number, None, node.span)?',SHAPE),
 ('literal refusal bypass','super::literal::normalize(number, *unit, node.span)?','super::literal::normalize(number, *unit, node.span).or_else(|_| super::literal::normalize("0", None, node.span))?', 'every_literal_position_refuses_atomically_with_original_span'),
 ('argument iteration','self.indices.next().map(|index|','self.indices.nth(1).map(|index|',CALL),
 ('argument size hint','self.indices.size_hint()','(0, Some(0))',CALL),
 ('debug privacy','.field("node_count", &self.nodes.len())','.field("node_count", &self.nodes.len()).field("nodes", &self.nodes)','every_worked_expression_normalizes_and_debug_omits_customer_content'),
]
try:
 for index,(name,before,after,test) in enumerate(CASES,1):
  text=ORIGINAL.decode();assert text.count(before)==1,(name,'actual anchor not unique',text.count(before))
  SOURCE.write_text(text.replace(before,after,1))
  result=subprocess.run(['cargo','test','-p','sc-core','--test','formula_normalized_contract',test,'--','--exact'],cwd=ROOT,capture_output=True)
  output=result.stdout+result.stderr;(WORK/('mutation-%d.log'%index)).write_bytes(output)
  assert result.returncode==101 and b'test result: FAILED.' in output and b'assertion' in output and b'could not compile' not in output,(name,output.decode())
  print('normalized arena mutation %d %s: rc=101, compiled actual public assertion red'%(index,name))
  SOURCE.write_bytes(ORIGINAL)
finally:
 SOURCE.write_bytes(ORIGINAL)
assert SOURCE.read_bytes()==ORIGINAL
print('normalized arena mutations: seventeen compiled actual assertion reds; exact source restored')
