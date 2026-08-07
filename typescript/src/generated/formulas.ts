// GENERATED FILE - do not edit by hand.
//
// Rendered from the published `no single dataset file`. Edits are overwritten on the next refresh.

import type { Confidence, Grade, Validation } from "../types.js";

/** A damage-composition formula, described but not yet evaluable. */
export interface Formula {
  /** A stable identifier for this formula, scoped to this repository. */
  readonly id: string;
  /** What this formula computes, in prose — not the expression itself. */
  readonly description: string;
  /** An opaque handle for the evidence behind this formula. It resolves only against the internal ledger and states nothing about where the value was read from. Absent means no ledger entry exists yet — never a stand-in for one. */
  readonly evidence_id: string | null;
  /** How much standing the extraction behind this formula has. A separate axis from `validation`: nothing having graded it and an oracle having disagreed are different failures, and one number cannot carry both. */
  readonly confidence: Confidence;
  /** Whether this description has been checked against a capture of the game. Every record here is unvalidated until one exists. */
  readonly validation: Validation;
  /** The build this description was derived from. A formula's constants are specific to one build, so a consumer must not evaluate this description against another build's numbers. */
  readonly build_id: string;
}

/** Every recovered formula description, sorted by `id`. */
export const FORMULAS: readonly Formula[] = [
];
