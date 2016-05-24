#![feature(plugin_registrar, quote, rustc_private)]

extern crate rustc_plugin;
extern crate syntax;

use rustc_plugin::registry::Registry;
use syntax::ast::{Expr, ExprKind, Item, ItemKind, LitKind, Mac, MetaItem, MetaItemKind};
use syntax::codemap::Span;
use syntax::fold::{self, Folder};
use syntax::ptr::P;
use syntax::ext::base::{Annotatable, ExtCtxt, SyntaxExtension};
use syntax::parse::token;

const ATTR_NAME: &'static str = "overload_strings";

struct StrFolder<'a, 'cx: 'a>(&'a mut ExtCtxt<'cx>, Option<Span>);

impl<'a, 'cx> Folder for StrFolder<'a, 'cx> {
    fn fold_item_simple(&mut self, i: Item) -> Item {
        // don't double-overload in case of nested annotations
        if i.attrs.iter().any(|attr| {
            match attr.node.value.node {
                MetaItemKind::Word(ref name) if name == ATTR_NAME => true,
                _ => false
            }
        }) { return i; }
        // ignore statics/consts, don't automatically recurse into submodules
        match i.node {
            ItemKind::Static(..) | ItemKind::Const(..) => {
                i
            }
            ItemKind::Mod(_) => {
                if let Some(top_span) = self.1 {
                    if i.span == top_span {
                        return fold::noop_fold_item_simple(i, self);
                    }
                }
                i
            }
            _ => fold::noop_fold_item_simple(i, self)
        }
    }

    fn fold_expr(&mut self, e: P<Expr>) -> P<Expr> {
        if let ExprKind::Lit(ref l) = e.node {
            if let LitKind::Str(..) = l.node {
                return quote_expr!(self.0, $e.into());
            }
        }
        e.map(|e| fold::noop_fold_expr(e, self))
    }

    fn fold_mac(&mut self, mac: Mac) -> Mac {
        // this usually panics, but in our case it's ok
        fold::noop_fold_mac(mac, self)
    }
}

pub fn insert_str_into(cx: &mut ExtCtxt, _span: Span, _mi: &MetaItem,
                       a: Annotatable) -> Annotatable {
    match a {
        Annotatable::Item(i) => Annotatable::Item(
            StrFolder(cx, Some(i.span)).fold_item(i).expect_one("expected one item")),
        Annotatable::TraitItem(i) => Annotatable::TraitItem(
            i.map(|i| StrFolder(cx, None).fold_trait_item(i).expect_one("expected one item"))),
        Annotatable::ImplItem(i) => Annotatable::ImplItem(
            i.map(|i| StrFolder(cx, None).fold_impl_item(i).expect_one("expected one item"))),
    }
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_syntax_extension(token::intern(ATTR_NAME),
                                  SyntaxExtension::MultiModifier(Box::new(insert_str_into)));
}
