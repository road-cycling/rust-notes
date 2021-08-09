use std::thread;

// (not my work)

pub (crate) fn do_mr() {

    let data = "12342345234523452345456345634563423452345345634563452345234534645634563456342345234523452345234654634562345456345234534534634534534534953894583945834532345346334562345239458265345063456928345203453945683542345234634563592345234935634568234523453452345";

    let mut children = vec![];
    
    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {

        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || -> u32 {

            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();

            println!("processed segment {}, result={}", i, result);

            result

        }));

    }

    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final result is: {}", final_result);

}
