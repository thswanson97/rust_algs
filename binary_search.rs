fn find_index( arr: &[usize], find: usize) -> Option<usize> {
   let length = arr.len();
   let mut half = length / 2;
   let mut hind = length - 1;
   let mut lind = 0;
   let mut current = arr[half];

   while  lind <= hind {
       if current == find {
           return Some(half);
       }
       else if current < find{
           lind = half + 1;
       }
       else if current > find {
           hind = half - 1;
       }
        half = (hind + lind) / 2;
        current = arr[half];
   }
   return None;
}

fn grab_array_index(my_arr : &[usize], find: usize) -> Option<usize>{ 
    return my_arr.iter()
    .position(|&x| x == find)
}

fn main() 
{
    let mut arr = vec![1,2,3,4,5,6];
    let mut find = 3;

    match grab_array_index(&arr, find) {
        Some(p) => println!("index is {}", p),
        None => println!("not in the array >:("),
    }
    match find_index(&arr, find) {
        Some(p) => println!("Here ya go! {}", p),
        None => println!("not in the array >:("),
    }

    arr = vec![1,3,5,7,9,11,13,15,17,19];
    find = 17;

    match grab_array_index(&arr, find) {
        Some(p) => println!("index is {}", p),
        None => println!("not in the array >:("),
    }
    match find_index(&arr, find) {
        Some(p) => println!("Here ya go! {}", p),
        None => println!("not in the array >:("),
    }

    arr = vec![1,3,5,7,9,11,13,15,17,19];
    find = 18;

    match grab_array_index(&arr, find) {
        Some(p) => println!("index is {}", p),
        None => println!("not in the array >:("),
    }
    match find_index(&arr, find) {
        Some(p) => println!("Here ya go! {}", p),
        None => println!("not in the array >:("),
    }
}