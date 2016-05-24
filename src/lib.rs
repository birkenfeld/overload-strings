#![feature(plugin_registrar, quote, rustc_private)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::registry::Registry;
use syntax::ast::{Expr, ExprKind, LitKind, Mac, MetaItem};
use syntax::fold::{self, Folder};
use syntax::ptr::P;
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt, SyntaxExtension};
use syntax::parse::token;

struct StrFolder<'a, 'cx: 'a>(
    &'a mut ExtCtxt<'cx>,
);

impl<'a, 'cx> Folder for StrFolder<'a, 'cx> {
    fn fold_expr(&mut self, e: P<Expr>) -> P<Expr> {
        if let ExprKind::Lit(ref l) = e.node {
            if let LitKind::Str(..) = l.node {
                return quote_expr!(self.0, $e.into());
            }
        }
        e.map(|e| fold::noop_fold_expr(e, self))
    }

    fn fold_mac(&mut self, mac: Mac) -> Mac {
        fold::noop_fold_mac(mac, self)
    }
}

pub fn insert_str_into(cx: &mut ExtCtxt, _span: Span, _mi: &MetaItem,
                       a: Annotatable) -> Annotatable {
    match a {
        Annotatable::Item(i) => Annotatable::Item(
            StrFolder(cx).fold_item(i).expect_one("expected one item")),
        Annotatable::TraitItem(i) => Annotatable::TraitItem(
            i.map(|i| StrFolder(cx).fold_trait_item(i).expect_one("expected one item"))),
        Annotatable::ImplItem(i) => Annotatable::ImplItem(
            i.map(|i| StrFolder(cx).fold_impl_item(i).expect_one("expected one item"))),
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern("overload_strings"),
                                  SyntaxExtension::MultiModifier(Box::new(insert_str_into)));
}
