pub fn arrays_m(io: &mut std::io::Stdin) {
    let a = [1, 2, 3, 5, 8];

    println!("The array is: {:?}", a);
    println!("Input the index of the array: ");

    let mut index = String::new();

    io.read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
    .trim()
    .parse().expect("Please type a number!");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}