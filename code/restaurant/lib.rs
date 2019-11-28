/*
Module tree

crate
    front_of_house
        hosting
        hosting

like a unix directory tree
one can write the directory and bodies with {}
one can move the code to their modules creating directories and files without changing paths!

Summary

Rust lets you organize your packages into crates and your crates into modules so you can refer to
items defined in one module from another module. You can do this by specifying absolute or relative
paths. These paths can be brought into scope with a use statement so you can use a shorter path for
multiple uses of the item in that scope. Module code is private by default, but you can make 
definitions public by adding the pub keyword.
*/

fn serve_order() {}

// super refers to parent (..)
mod back_of_house {
    // here fields are public if enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // struct fields follow the general rule of everything being private by default
    // unless annotated with pub
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

mod front_of_house;

// with pub use, external code can call functions defined in the scope of module hosting
pub use crate::front_of_house::hosting;

use std::fmt::Result;
use std::io::Result as IOResult;
use std::collections::HashMap;
use std::{cmp::Ordering, cmp};
use std::io::{self, Write};

pub fn eat_at_restaurant() {
    // absolute path (preffered)
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // use to the rescue
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("New");
    println!("{} toast please", meal.toast);

    // enums
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}