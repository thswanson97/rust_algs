

// fn find_index( arr: &[usize], find: usize) -> Option<usize>
// {
//     let length = arr.len();
//     let half = length / 2;
//     println!("half at top {}", half);
//     if arr[half] == find
//     {
//         println! ("yay, found {}", half);
//         return half;
//     }
//     else if length ==1 && find != arr[half] 
//     {
//         return None;
//     }
//     else if arr[half] < find 
//     {
//         println!("in upper half {}",arr[half]);
//         let upper_half = &arr[half .. length];
//         find_index(upper_half, find)
//     }
//     else 
//     {
//         println!("in lower half {}",arr[half]);
//         let lower_half = &arr[0 .. half ];
//         find_index(lower_half, find)
//     }
// }

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

fn main() 
{
    fn grab_array_index(my_arr : &[usize], find: usize) -> usize{ 
        return my_arr.iter()
        .position(|&x| x == find)
        .unwrap();
    }

    println!("sup bwords");
    
    let mut arr = vec![1,2,3,4,5,6];
    let mut find = 3;
    println!("Index of {} is {}", find , grab_array_index(&arr, find));
    match find_index(&arr, find) {
        Some(p) => println!("has value {}", p),
        None => println!("nothing")
    }

    arr = vec![1,3,5,7,9,11,13,15,17,19];
    find = 17;
    println!("Index of {} is {}", find , grab_array_index(&arr, find));
    match find_index(&arr, find) {
        Some(p) => println!("has value {}", p),
        None => println!("nothing")
    }
    arr = vec![1,3,5,7,9,11,13,15,17,19];
    find = 17;
    println!("Index of {} is {}", find , grab_array_index(&arr, find));
    match find_index(&arr, find) {
        Some(p) => println!("has value {}", p),
        None => println!("nothing")
    }
}