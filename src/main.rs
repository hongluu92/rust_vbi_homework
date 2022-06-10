use std::io;

fn main() {
    let mut org_arr = [1,2,3,4,5,6,7,8,9];
    println!("=== CHECK CONTAINT ==="); 
    println!("sub_arr_1 :{}",check_containt(&mut org_arr, &mut [1,2,3]));
    println!("sub_arr_2 :{}",check_containt(&mut org_arr, &mut [2,3,4]));
    println!("sub_arr_3 :{}",check_containt(&mut org_arr, &mut [7,8,9]));
    println!("sub_arr_4 :{}",check_containt(&mut org_arr, &mut [1,2]));
    println!("sub_arr_5 :{}",check_containt(&mut org_arr, &mut [2,3,4,6]));
    
    println!("=== COUNT ME ==="); 
    let input_text = String::from("asdasd3asd32Ds14gfsgfdgdfgdfgf");
    loop {
        let mut buff = String::new();
        println!("Enter a character (x)");
        io::stdin().read_line(&mut buff).unwrap();
        let buff = buff.trim();
        if buff == "1" {
            break;
        }
        println!("count (x)  {}", count_me_in(&buff, &input_text));
        println!("count (x) ignore case {}", count_me_in_ingore_case(&buff, &input_text));      
    }
}
fn check_containt( org_arr: &mut [i32], sub_arr: &mut [i32]) -> bool{
    let org_length = org_arr.len();
    let sub_length = sub_arr.len();
    
    for i in 0..(org_length - sub_length + 1) {
        for j in 0..(sub_length ) {
            if org_arr[i+j] != sub_arr[j] {
                break;
            }
            if j == sub_length -1 {
                return true;
            }     
        }
    }
    return false;
}

fn count_me_in( me: &str, input_text: &String ) -> usize {
    return input_text.matches(me).count();
}

fn count_me_in_ingore_case( me: &str, input_text: &String ) -> usize {
    return input_text.to_lowercase().matches(&me.to_lowercase()).count();
}