/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::StyleSheetListBinding;
use dom::bindings::codegen::Bindings::StyleSheetListBinding::StyleSheetListMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::utils::{Reflector, reflect_dom_object};
use dom::document::Document;
use dom::node::window_from_node;
use dom::stylesheet::StyleSheet;

// https://drafts.csswg.org/cssom/#the-stylesheetlist-interface
#[dom_struct]
pub struct StyleSheetList {
    reflector_: Reflector,
    owner_doc: JS<Document>,
}

impl StyleSheetList {
    pub fn new_inherited(owner_doc: &Document) -> StyleSheetList {
        StyleSheetList {
            reflector_: Reflector::new(),
            owner_doc: JS::from_ref(owner_doc),
        }
    }

    pub fn new(owner_doc: &Document)
               -> Root<StyleSheetList> {
        let win = owner_doc.window();
        reflect_dom_object(box StyleSheetList::new_inherited(owner_doc),
                           GlobalRef::Window(win.r()),
                           StyleSheetListBinding::Wrap)
    }
}

impl StyleSheetListMethods for StyleSheetList {
    // https://drafts.csswg.org/cssom/#dom-stylesheetlist-length
    fn Length(&self) -> u32 {
        let doc = self.owner_doc.root();
        let sheets = doc.r().get_stylesheets();
        sheets.len() as u32
    }
}
