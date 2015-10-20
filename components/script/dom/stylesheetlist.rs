/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::StyleSheetListBinding;
use dom::bindings::codegen::Bindings::StyleSheetListBinding::StyleSheetListMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::Castable;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::document::Document;
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
        reflect_dom_object(box StyleSheetList::new_inherited(owner_doc),
                           GlobalRef::Window(owner_doc.window()),
                           StyleSheetListBinding::Wrap)
    }
}

impl StyleSheetListMethods for StyleSheetList {
    // https://drafts.csswg.org/cssom/#dom-stylesheetlist-item
    fn Item(&self, index: u32) -> Option<Root<StyleSheet>> {
        let index = index as usize;
        let sheets = self.owner_doc.stylesheets();
        if index >= sheets.len() {
            return None;
        }
        let sheet = &*sheets[index];
        Some(Root::from_ref(sheet.upcast()))
    }
    // https://drafts.csswg.org/cssom/#dom-stylesheetlist-length
    fn Length(&self) -> u32 {
        let sheets = self.owner_doc.stylesheets();
        sheets.len() as u32
    }
    // https://drafts.csswg.org/cssom/#stylesheetlist
    fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<Root<StyleSheet>> {
        let rval = self.Item(index);
        *found = index < self.Length();
        rval
    }
}
