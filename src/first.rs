// in first.rs

/*
	In functional programming a linked list can be represented as
	List a = Empty | Elem a (List a)
	This can be translated to procedural programming using an enum (a sum type)
*/

// pub -> indicates this enum to be used outside this module
// This roughly translates to a list can be either empty or 
// an element followed by a list which holds true for the recursive functional
// definition mentioned above.
pub enum List {
    Empty,
    Elem(i32, List),
}
