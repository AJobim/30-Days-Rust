//Bubble sort algorithm


fn bubble_sort(mut array: [u32; 5]) {
    let size = array.len();
    let mut swapped: bool;

    for _i in 0..size - 1 {        
        swapped = false;
        
        for j in 0..size - 1 {            
            if array[j] > array[j + 1]{
                array.swap(j, j + 1);
                swapped = true;
            }
        }

        if swapped == false {
            break;
        }
    }

    println!("Finished sorting!");
    println!("The array post-sorting = {:?}", array);
}

fn main(){
    let numbers: [u32; 5] = [9, 7, 15, 5, 2];
    println!("The array pre-sorting = {:?}", numbers);
    bubble_sort(numbers);
}

