use anyhow::Result;
use flowistry::mir::{borrowck_facts::get_body_with_borrowck_facts, utils::BodyExt};
use log::debug;
use rustc_data_structures::fx::FxHashSet as HashSet;
use rustc_hir::BodyId;
use rustc_macros::Encodable;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::{
  analysis::{FlowistryAnalysis, FlowistryOutput, FlowistryResult},
  range::Range,
};

struct Playground {
  range: Range,
}

#[derive(Debug, Clone, Encodable, Default)]
pub struct PlaygroundOutput {
  outlives: HashSet<(String, String)>,
}

impl FlowistryOutput for PlaygroundOutput {
  fn merge(&mut self, other: PlaygroundOutput) {
    self.outlives.extend(other.outlives);
  }
}

impl FlowistryAnalysis for Playground {
  type Output = PlaygroundOutput;

  fn locations(&self, tcx: TyCtxt) -> Result<Vec<Span>> {
    Ok(vec![self.range.to_span(tcx.sess.source_map())?])
  }

  fn analyze_function(&mut self, tcx: TyCtxt, body_id: BodyId) -> Result<Self::Output> {
    let def_id = tcx.hir().body_owner_def_id(body_id);
    let body_with_facts = get_body_with_borrowck_facts(tcx, def_id);
    let body = &body_with_facts.body;
    debug!("{}", body.to_string(tcx).unwrap());

    let outlives = body_with_facts
      .input_facts
      .subset_base
      .iter()
      .map(|(sup, sub, _)| (format!("{:?}", sup), format!("{:?}", sub)))
      .collect::<HashSet<_>>();

    Ok(PlaygroundOutput { outlives })
  }
}

pub fn playground(
  range: Range,
  compiler_args: &[String],
) -> FlowistryResult<PlaygroundOutput> {
  Playground { range }.run(compiler_args)
}
