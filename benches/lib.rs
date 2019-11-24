#[macro_use]
extern crate bencher;
extern crate roaring;


use bencher::{Bencher, black_box};
use roaring::RoaringBitmap;

fn create(b: &mut Bencher) {
    b.iter(|| {
        RoaringBitmap::new();
    })
}

fn insert1(b: &mut Bencher) {
    b.iter(|| {
        let mut bitmap = RoaringBitmap::new();
        bitmap.insert(1);
        bitmap
    })
}

fn insert2(b: &mut Bencher) {
    b.iter(|| {
        let mut bitmap = RoaringBitmap::new();
        bitmap.insert(1);
        bitmap.insert(2);
        bitmap
    })
}

fn is_subset(b: &mut Bencher) {
    let bitmap: RoaringBitmap = (1..250).collect();
    b.iter(|| black_box(bitmap.is_subset(&bitmap)))
}

fn is_subset_2(b: &mut Bencher) {
    let sub: RoaringBitmap = (1000..8196).collect();
    let sup: RoaringBitmap = (0..16384).collect();
    b.iter(|| black_box(sub.is_subset(&sup)))
}

fn is_subset_3(b: &mut Bencher) {
    let sub: RoaringBitmap = (1000..4096).map(|x| x * 2).collect();
    let sup: RoaringBitmap = (0..16384).collect();
    b.iter(|| black_box(sub.is_subset(&sup)))
}

fn is_subset_4(b: &mut Bencher) {
    let sub: RoaringBitmap = (0..17).map(|x| 1 << x).collect();
    let sup: RoaringBitmap = (0..65536).collect();
    b.iter(|| black_box(sub.is_subset(&sup)))
}

fn remove_range_bitmap(b: &mut Bencher) {
    let mut sub: RoaringBitmap = (0..65536).collect();
    b.iter(|| {
        // carefully delete part of the bitmap
        // only the first iteration will actually change something
        // but the runtime remains identical afterwards
        black_box(sub.remove_range(4096 + 1..65536));
        assert_eq!(sub.len(), 4096 + 1);
    });
}

benchmark_group!(benches, create, insert1, insert2, is_subset, is_subset_2, is_subset_3, is_subset_4, remove_range_bitmap);
benchmark_main!(benches);
