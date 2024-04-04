use std::{mem, thread};

use rand::random;
use roaring::{RoaringBitmap, RoaringTreemap};

#[test]
fn roaring() {
    let mut list = Vec::with_capacity(10000);
    let mut map = RoaringBitmap::new();
    let mut tree = RoaringTreemap::new();

    thread::scope(|s| {
        s.spawn(|| {
            for _ in 1..100000 {
                list.push(random::<u64>());
            }
        });

        s.spawn(|| {
            for i in 1..100000 {
                map.push(i);
            }
        });

        s.spawn(|| {
            for i in 1..100000 {
                tree.push(i);
            }
        });
    });

    let rank = dbg!(tree.rank(10));
    dbg!(tree.select(rank - 1));

    dbg!(list.len(), mem::size_of_val(&*list));
    dbg!(map.len(), map.serialized_size());
    dbg!(tree.len(), tree.serialized_size());
}
