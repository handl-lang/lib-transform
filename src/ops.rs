//! Rust `Function` impls for the `transform` module — the handls
//! (handlers) for the `op`/`handl` declarations in `index.hnl`.
//!
//! All functions are FFI atoms: they manipulate `serde_json::Value`
//! callables via `apply_fn`-family helpers from `handl-lib-ops`'s
//! `shared` module and cannot be expressed as pure handl arrows without
//! losing the exact `TypeMismatch` error messages the tests rely on. The
//! `op`/`handl` declaration form (ADR-0077 Phase 2) declares the
//! interface in `index.hnl`; these impls provide the logic and are
//! registered in an [`OpRegistry`](handl_ops::OpRegistry). Moved verbatim
//! from the retired `kernel/crates/combinators` floor-lib crate.

use handl_ops::{Op, OpContext, OpError};

use handl_lib_ops::shared::{
    arr, filter_callable_iter, fold_callable_iter, map_callable_iter, require_array,
};

use handl_ops::OpRegistry;
use std::sync::Arc;

// ===========================================================================
// Collection iteration
// ===========================================================================

// ── map ────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct Map;
impl Op for Map {
    fn call(
        &self,
        args: &[serde_json::Value],
        ctx: &mut dyn OpContext,
    ) -> Result<serde_json::Value, OpError> {
        if args.len() < 2 {
            return Err(OpError::TypeMismatch(format!(
                "expected {} args, got {}",
                2,
                args.len(),
            )));
        }
        let array = require_array(&args[1])?;
        let results = map_callable_iter(ctx, &args[0], array.iter())?;
        Ok(arr(&results))
    }
}

// ── filter ─────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct Filter;
impl Op for Filter {
    fn call(
        &self,
        args: &[serde_json::Value],
        ctx: &mut dyn OpContext,
    ) -> Result<serde_json::Value, OpError> {
        if args.len() < 2 {
            return Err(OpError::TypeMismatch(format!(
                "expected {} args, got {}",
                2,
                args.len(),
            )));
        }
        let array = require_array(&args[1])?;
        let results = filter_callable_iter(ctx, &args[0], array.iter())?;
        Ok(arr(&results))
    }
}

// ── reduce ─────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
pub struct Reduce;
impl Op for Reduce {
    fn call(
        &self,
        args: &[serde_json::Value],
        ctx: &mut dyn OpContext,
    ) -> Result<serde_json::Value, OpError> {
        if args.len() < 3 {
            return Err(OpError::TypeMismatch(format!(
                "expected {} args, got {}",
                3,
                args.len(),
            )));
        }
        let array = require_array(&args[1])?;
        let init = args[2].clone();
        let result = fold_callable_iter(ctx, &args[0], array.iter(), init)?;
        Ok(serde_json::Value::from(result))
    }
}

pub const TRANSFORM_OPS: &[&str] = &["transform.map", "transform.filter", "transform.reduce"];

pub fn transform_op_registry() -> OpRegistry {
    let mut registry = OpRegistry::new();
    registry.register("transform.map", Arc::new(Map));
    registry.register("transform.filter", Arc::new(Filter));
    registry.register("transform.reduce", Arc::new(Reduce));
    registry
}
