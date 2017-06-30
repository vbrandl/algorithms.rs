extern crate sort;
extern crate rand;

use sort::bubble_sort::bubble_sort;
use sort::heap_sort::heap_sort;
use sort::insertion_sort::insertion_sort;
use sort::merge_sort::{merge_sort_bottom_up, merge_sort_top_down};
use sort::selection_sort::selection_sort;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut field: [i16; 10] = [0; 10];
    for x in &mut field {
        *x = rng.gen::<i16>();
    }

    // let mut field = [5, 3, 6, 7, 3, 2, 302];
    let mut field2 = field;
    let mut field3 = field;
    let mut field4 = field.to_vec();
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
