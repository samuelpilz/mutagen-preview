use quote::quote;
use syn::fold::Fold;
use syn::{parse_quote, Expr, ExprLit, Lit, LitBool};

use super::default_folds::fold_expr_default;
use super::MutagenTransformer;
use crate::transform_info::register_global_mutation;

pub struct MutagenTransformerLitBool();

// transforms bool literals to mutator expressions
impl Fold for MutagenTransformerLitBool {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Lit(ExprLit {
                lit: Lit::Bool(LitBool { value, span: _ }),
                attrs: _,
            }) => {
                let mutator_id =
                    register_global_mutation(format!("LitBool {} -> {}", value, !value));
                parse_quote! {
                    ::mutagen_preview::mutator::MutatorLitBool::new(#mutator_id, #value)
                        .run_mutator(
                            &mutagen_preview::MutagenRuntimeConfig::get_default()
                        )
                }
            }
            _ => fold_expr_default(self, e),
        }
    }
}

impl MutagenTransformer for MutagenTransformerLitBool {}