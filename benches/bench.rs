#![feature(test)]
extern crate test;
use test::Bencher;

use delay_init::delay;
use std::{collections::HashMap, sync::OnceLock};

const N: usize = 10000;

delay! {
    static MAP: HashMap<u32,u32> = {
        let mut map = HashMap::new();
        map.insert(12,12);
        map
    };
}

#[bench]
fn lazy(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..N {
            assert_eq!(*MAP.get(&12).unwrap(), 12);
            assert!(MAP.get(&11).is_none());
        }
    });
}

#[bench]
fn normal(b: &mut Bencher) {
    let mut map = HashMap::new();
    map.insert(12,12);
    b.iter(|| {
        for _ in 0..N {
            assert_eq!(*map.get(&12).unwrap(), 12);
            assert!(map.get(&11).is_none());
        }
    });
}

fn get_map() -> &'static HashMap<i32,i32> {
    static CELL: OnceLock<HashMap<i32,i32>> = OnceLock::new();
    CELL.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(12,12);
        map
    })
}

#[bench]
fn stdcell(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..N {
            assert_eq!(*get_map().get(&12).unwrap(), 12);
            assert!(get_map().get(&11).is_none());
        }
    });
}
