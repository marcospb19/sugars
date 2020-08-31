//! This crate provides a collection of macros to make some tasks easier to use
//! on Rust ecosystem.
//!
//! ## What it has to offer?
//!  * **Collections:**
//!     * **bheap**: Create a [`BinaryHeap`] from a list of key-value pairs
//!     * **btmap**: Create a [`BTreeMap`] from a list of key-value pairs
//!     * **btset**: Create a [`BTreeSet`] from a list of elements
//!     * **deque**: Create a [`VecDeque`] from a list of elements
//!     * **flkl**: Create a [`LinkedList`] from a list of elements, adding each element
//!       to the start of the list.
//!     * **hmap**: Create a [`HashMap`] from a list of key-value pairs
//!     * **hset**: Create a [`HashSet`] from a list of elements
//!     * **lkl**: Create a [`LinkedList`] from a list of elements, adding each element to
//!       the end of the list.
//!  * **Comprehensions:**
//!     * **c**: Macro to make a lazy Iterator collection comprehensions, all other are
//!       base on this one.
//!     * **cbheap**: Macro to [`BinaryHeap`] collection comprehensions
//!     * **cbtmap**: Macro to [`BTreeMap`] collection comprehensions
//!     * **cbtset**: Macro to [`BTreeSet`] collection comprehensions
//!     * **cdeque**: Macro to [`VecDeque`] collection comprehensions
//!     * **cmap**: Macro to [`HashMap`] collection comprehensions
//!     * **cset**: Macro to [`HashSet`] collection comprehensions
//!     * **cvec**: Macro to [`Vec`] collection comprehensions
//!  * **Smart Pointers:**
//!     * **arc**: A simple macro to make a new [`Arc`] value.²
//!     * **boxed**: A simple macro to make a new [`Box`] value.²
//!     * **cell**: A simple macro to make a new [`Cell`] value.²
//!     * **cow**: A simple macro to make a new `Cow::Owned` value.
//!     * **mutex**: A simple macro to make a new [`Mutex`] value.²
//!     * **refcell**: A simple macro to make a new [`RefCell`] value.²
//!     * **rc**: A simple macro to make a new [`Rc`] value.²
//!     * **rwlock**: A simple macro to make a new [`RwLock`] value.²
//!  * **Time/Duration:**
//!     * **dur**: Creates a [`Duration`] object following a time pattern¹
//!     * **sleep**: Makes a thread sleep a amount following a time pattern¹
//!     * **time**: Print out the time it took to execute a given expression in seconds
//!
//!  1. A time pattern can be: mim, sec, nano, micro, milli
//!  2. Also can return a tuple if is given more than one parameter
//!
//! ## Examples
//! **arc**, **boxed**, **cell**, **cow**, **mutex** and **refcell**: Usage are mostly the
//! same, just change de Types and macros
//! ```rust
//! use sugars::boxed;
//! assert_eq!(Box::new(10), boxed!(10));
//! ```
//!
//! **hmap** and **btmap**: Usage are the same, just change de Types and macros
//! ```rust
//! use std::collections::HashMap;
//! use sugars::hmap;
//!
//! let mut map = HashMap::new();
//! map.insert("1", 1);
//! map.insert("2", 2);
//!
//! let map2 = hmap! {"1" => 1, "2" => 2};
//!
//! assert_eq!(map, map2);
//! ```
//!
//! **bheap**, **hset**, **btset** and **deque**: Usage are the same, just change de Types
//! and macros
//! ```rust
//! use std::collections::HashSet;
//! use sugars::hset;
//!
//! let mut set = HashSet::new();
//! set.insert(1);
//! set.insert(2);
//!
//! let set2 = hset! {1, 2};
//!
//! assert_eq!(set, set2);
//! ```
//!
//! **dur** and **sleep**
//! ```rust
//! use sugars::{dur, sleep};
//!
//! let d1 = dur!(10 sec);
//! let d2 = std::time::Duration::from_secs(10);
//!
//! assert_eq!(d1, d2);
//!
//! // Sleeps uses the same syntax, but makes the thread sleep for the given time
//! sleep!(10 sec)
//! ```
//!
//! **c**: Notice that it generates a lazy Iterator, so the user has to deal with that
//!
//! This has the following syntax: `c![<expr>; <<pattern> in <iterator>, >...[, if
//! <condition>]]`
//! ```rust
//! use std::collections::HashSet;
//! use sugars::c;
//!
//! let vec = c![x; x in 0..10].collect::<Vec<_>>();
//! let set = c![i*2; &i in vec.iter()].collect::<HashSet<_>>();
//! let vec2 = c![i+j; i in vec.into_iter(), j in set.iter(), if i%2 == 0 && j%2 !=
//! 0].collect::<Vec<_>>();
//! ```
//!
//! **cvec**, **cdeque**, **clkl** and **cbheap**: Usage are the same, just change de
//! Types and macros
//! ```rust
//! use sugars::cvec;
//!
//! // Normal comprehension
//! cvec![x; x in 0..10];
//!
//! // You can filter as well
//! cvec![x; x in 0..10, if x % 2 == 0];
//! ```
//!
//! **cset** and **cbtset**: Usage are the same, just change de Types and macros
//! ```
//! use sugars::cset;
//!
//! // Normal comprehension
//! cset! {x; x in 0..10};
//!
//! // You can filter as well
//! cset! {x; x in 0..10, if x % 2 == 0};
//! ```
//!
//! **cmap** and **cbtmap**: Usage are the same, just change de Types and macros
//! ```rust
//! use sugars::cmap;
//!
//! // Normal comprehension
//! cmap! {x => x*2; x in 1..10};
//!
//! // You can filter as well
//! cmap! {x => x*2; x in 1..10, if x%2 == 0};
//! ```
//!
//! **time**
//! ```rust
//! use sugars::{time, sleep};
//!
//! // Should print to stderr ≈ 2.0000 seconds
//! time!( sleep!(2 sec) );
//!
//! // It also return the evaluated expression, like dbg! macro
//! let x = time!( 100 + 20 );
//! ```
//!
//! [`BinaryHeap`]: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html
//! [`BTreeMap`]: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html
//! [`BTreeSet`]: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html
//! [`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
//! [`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
//! [`VecDeque`]: https://doc.rust-lang.org/std/collections/struct.VecDeque.html
//! [`LinkedList`]: https://doc.rust-lang.org/std/collections/struct.LinkedList.html
//!
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [`Mutex`]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
//! [`Rc`]: https://doc.rust-lang.org/std/rc/struct.Rc.html
//! [`RefCell`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html
//! [`RwLock`]: https://doc.rust-lang.org/std/sync/struct.RwLock.html
//!
//! [`Duration`]: https://doc.rust-lang.org/std/time/struct.Duration.html

mod collections;
mod comprehension;
mod hash;
mod pointer;
mod times;
