// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


struct foo(Box<uint>);

impl Add<foo, foo> for foo {
    fn add(&self, f: &foo) -> foo {
        let foo(box i) = *self;
        let foo(box j) = *f;
        foo(box() (i + j))
    }
}

fn main() {
    let x = foo(box 3);
    let _y = x + {x}; // the `{x}` forces a move to occur
    //~^ ERROR cannot move out of `x`
}
