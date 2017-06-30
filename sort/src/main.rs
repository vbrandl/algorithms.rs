extern crate rand;

mod bubble_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod selection_sort;

use bubble_sort::bubble_sort;
use heap_sort::heap_sort;
use insertion_sort::insertion_sort;
use merge_sort::{merge_sort_bottom_up, merge_sort_top_down};
use selection_sort::selection_sort;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut field: [i64; 10] = [0; 10];
    for i in 0..field.len() {
        field[i] = rng.gen::<i64>();
    }

    // let mut field = [5, 3, 6, 7, 3, 2, 302];
    let mut field2 = field.clone();
    let mut field3 = field.clone();
    let mut field4 = field.clone().to_vec();
    let mut field5 = field4.clone();
    let mut field6 = field4.clone();
    println!("unsorted:             {:?}", field);
    insertion_sort(&mut field);
    println!("insertion_sort:       {:?}", field);
    selection_sort(&mut field2);
    println!("selection_sort:       {:?}", field2);
    bubble_sort(&mut field3);
    println!("bubble_sort:          {:?}", field3);
    merge_sort_top_down(&mut field4);
    println!("merge_sort_top_down:  {:?}", field4);
    merge_sort_bottom_up(&mut field5);
    println!("merge_sort_bottom_up: {:?}", field5);
    heap_sort(&mut field6);
    println!("heap_sort:            {:?}", field6);
}
