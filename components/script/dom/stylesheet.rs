/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::StyleSheetBinding::StyleSheetMethods;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::Reflector;
use dom::node::Node;
use util::str::DOMString;

use std::borrow::ToOwned;

// https://drafts.csswg.org/cssom/#the-stylesheet-interface
#[dom_struct]
pub struct StyleSheet {
    reflector_: Reflector,
    location: Option<DOMString>,
    owner_node: Option<JS<Node>>,
    parent_stylesheet: Option<JS<StyleSheet>>,
    disabled: bool,
}

impl StyleSheet {
    pub fn new_inherited(location: Option<DOMString>,
                         owner_node: Option<&Node>,
                         parent_stylesheet: Option<&StyleSheet>)
                         -> StyleSheet {
        StyleSheet {
            reflector_: Reflector::new(),
            location: location,
            owner_node: owner_node.map(JS::from_ref),
            parent_stylesheet: parent_stylesheet.map(JS::from_ref),
            disabled: false,
        }
    }
}

impl StyleSheetMethods for StyleSheet {
    // https://drafts.csswg.org/cssom/#dom-stylesheet-type
    fn Type(&self) -> DOMString {
        DOMString("text/css".to_owned())
    }

    // https://drafts.csswg.org/cssom/#dom-stylesheet-href
    fn GetHref(&self) -> Option<DOMString> {
        self.location.clone()
    }

    // https://drafts.csswg.org/cssom/#dom-stylesheet-ownernode
    fn GetOwnerNode(&self) -> Option<Root<Node>> {
        self.owner_node.as_ref().map(|js| Root::from_ref(&**js))
    }

    // https://drafts.csswg.org/cssom/#dom-stylesheet-parentstylesheet
    fn GetParentStyleSheet(&self) -> Option<Root<StyleSheet>> {
        self.parent_stylesheet.as_ref().map(|js| Root::from_ref(&**js))
    }

    // https://drafts.csswg.org/cssom/#dom-stylesheet-disabled
    fn Disabled(&self) -> bool {
        self.disabled
    }
}
