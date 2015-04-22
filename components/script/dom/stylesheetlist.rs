/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::StyleSheetListBinding;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::utils::{Reflector, reflect_dom_object};
use dom::document::Document;
use dom::stylesheet::StyleSheet;

// https://drafts.csswg.org/cssom/#the-stylesheetlist-interface
#[dom_struct]
pub struct StyleSheetList {
    reflector_: Reflector,
    owner_doc: JS<Document>,
    items: Vec<JS<StyleSheet>>,
}

impl StyleSheetList {
    pub fn new_inherited(owner_doc: &Document) -> StyleSheetList {
        StyleSheetList {
            reflector_: Reflector::new(),
            owner_doc: JS::from_ref(owner_doc),
            items: vec![],
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
