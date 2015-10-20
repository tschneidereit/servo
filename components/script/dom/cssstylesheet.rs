/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::cell::DOMRefCell;
use dom::bindings::codegen::Bindings::CSSStyleSheetBinding;
use dom::bindings::codegen::Bindings::CSSStyleSheetBinding::CSSStyleSheetMethods;
use dom::bindings::codegen::Bindings::StyleSheetBinding::StyleSheetMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, MutNullableHeap, Root};
use dom::bindings::reflector::reflect_dom_object;
use dom::cssrule::CSSRule;
use dom::cssrulelist::CSSRuleList;
use dom::node::{Node, window_from_node};
use dom::stylesheet::StyleSheet;
use dom::window::Window;
use std::default::Default;
use std::sync::Arc;
use style::stylesheets as style;
use util::str::DOMString;

// https://drafts.csswg.org/cssom/#the-cssstylesheet-interface
#[dom_struct]
pub struct CSSStyleSheet {
    stylesheet: StyleSheet,
    owner_rule: Option<JS<CSSRule>>,
    style: DOMRefCell<Arc<style::Stylesheet>>,
    rule_list: MutNullableHeap<JS<CSSRuleList>>,
}

impl CSSStyleSheet {
    pub fn new_inherited(location: Option<DOMString>,
                         owner_node: Option<&Node>,
                         parent_stylesheet: Option<&StyleSheet>,
                         owner_rule: Option<&CSSRule>,
                         style: Arc<style::Stylesheet>)
                         -> CSSStyleSheet {
        CSSStyleSheet {
            stylesheet: StyleSheet::new_inherited(location, owner_node, parent_stylesheet),
            owner_rule: owner_rule.map(JS::from_ref),
            style: DOMRefCell::new(style),
            rule_list: Default::default(),
        }
    }

    pub fn new(global: &Window, location: Option<DOMString>,
               owner_node: Option<&Node>,
               parent_stylesheet: Option<&StyleSheet>,
               owner_rule: Option<&CSSRule>,
               style: Arc<style::Stylesheet>)
               -> Root<CSSStyleSheet> {
        reflect_dom_object(box CSSStyleSheet::new_inherited(location, owner_node,
                                                            parent_stylesheet,
                                                            owner_rule, style),
                           GlobalRef::Window(global),
                           CSSStyleSheetBinding::Wrap)
    }

    pub fn internal(&self) -> Arc<style::Stylesheet> {
        self.style.borrow().clone()
    }

    fn window(&self) -> Root<Window> {
        let mut sheet = Root::from_ref(&self.stylesheet);
        loop {
            if let Some(node) = sheet.GetOwnerNode() {
                return window_from_node(node.r());
            }
            sheet = sheet.GetParentStyleSheet().unwrap();
        }
    }
}

impl CSSStyleSheetMethods for CSSStyleSheet {
    // https://drafts.csswg.org/cssom/#dom-cssstylesheet-ownerrule
    fn GetOwnerRule(&self) -> Option<Root<CSSRule>> {
        self.owner_rule.as_ref().map(|js| Root::from_ref(&**js))
    }

    // https://drafts.csswg.org/cssom/#dom-cssstylesheet-cssrules
    fn CssRules(&self) -> Root<CSSRuleList> {
        self.rule_list.or_init(|| {
            CSSRuleList::new(&*self.window())
        })
    }
}
