// fn print_courtry(country_name: &String) -> String {
//     println!("{}", country_name);
//     country_name
// }

// fn print_courtry(country_name: &String) {
//     println!("{}", country_name);
// }

// fn add_hungary(country_name: &mut String) {
//     country_name.push_str("\t great!");
//     println!("{}", country_name);
// }

// fn prints_country(country_name: String ) {
//     println!("{}", country_name);
// }

// fn match_colours(rgb: (i32, i32, i32)) {
//     match rgb {
//         (r, _, _) if r < 10 => println!("Not much red"),
//         (_, r, _) if r < 10 => println!("Not much blue"),
//         (_, _, r) if r < 10 => println!("Not much green"),
//         _ => println!("Each colour has at least 10"),
//     }
// }

fn match_number(input: i32) {
    match input {
        number @ 4 => println!("4:{}", number),
        number@ 5 => println!("5:{}", number),
        _ => println!("default")
    }
}

fn main() {
    // let num = 100;
    // println!("{}", num as u8 as char)

    // let num : u8 = 100;
    // println!("{}", num as char)

    // char
    // println!("Size of string containing 'a': {}", "ß".len());
    // let slice = "ß";
    // println!("Slice is {} bytes. is {} char", slice.len(), slice.chars().count());

    // float
    // let my_float: f64 = 5.0;
    // let other_float: f32 = 5.5;
    //
    // let total_float = my_float + other_float as f64;
    // println!("{}", total_float)

    // func
    // println!("{}", number())

    // println!("{}", std::i8::MIN)

    // println!("{:X}", '행' as u32); // Cast char as u32 to get the hexadecimal value
    // println!("{:X}", 'H' as u32);
    // println!("{:X}", '居' as u32);
    // println!("{:X}", 'い' as u32);
    //
    // println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}"); // Try printing them with unicode escape \u

    // let number = 9;
    // let number_ref = &number;
    // println!("{}", number_ref);
    // println!("{:?}", number_ref);
    // println!("{:#?}", number_ref);
    // println!("{:p}", number_ref); // 返回内存地址
    // println!("Binary: {:b}", number_ref); // 打印number_ref 的值的二进制
    // println!("hexadecimal: {:x}", number_ref); // 打印number_ref 的值的十六进制
    // println!("octal: {:o}", number_ref); // 打印number_ref 的值的八进制
    // println!("{1}{0}", number_ref, number); //  更改打印顺序 从0开始
    // println!(
    //     "{c1} {c2}", c1 = "c11", c2 = "c22"
    // );
    // let title = "hahaha";
    // let bar = "ㅎ";
    // println!("{:ㅎ^30}", title); // title 在 30个ㅎ字符中间
    // println!("{:<10}", bar ); // 在 bar的右边填充15个空格
    // println!("{:>10}", bar ) // 在 bar的右边填充15个空格

    // let name = "tinalst";
    // let other_name = String::from("wananana"); // make a string from $str
    // println!("name: {} \n other_name: {}", name, other_name);


    // let name = "😂";
    // println!("{}", name);

    // let my_name = "tian";
    // let my_country = "china";
    // let my_home = "china";
    //
    // let together = format!("{} {} {},", my_name, my_country, my_home);
    // println!("{}",together)

    // let my_string: String = "hahahah".into();
    // println!("{}", my_string)

    // let my_name = "tina";
    // my_name = "ahh"; // cannot assign twice to immutable variable

    // let mut my_number = 8;
    // my_number = 10; // set success

    //
    // let mut my_number = 8;
    // // only look at value of my_number -  `my_number_ref` is a `&` reference
    // let my_number_ref = &mut my_number;
    // // let my_number_ref = &mut my_number;
    // *my_number_ref += 10;
    // println!("{}",my_number);
    //
    // let second_number = 800;
    // let triple_reference = &&&second_number;
    // println!("{}", second_number = ***triple_reference)

    // let mut number = 10;
    // let number_change = &mut number;
    // *number_change += 10;
    // let number_ref = &number;
    // println!("{}", number_ref)

    // let country = String::from("china");
    // let country = print_courtry(country);
    // print_courtry(country);

    // let country = String::from("china");
    // print_courtry(&country);
    // print_courtry(&country);


    // let mut country = String::from("china");
    // add_hungary(&mut country);
    // add_hungary(&mut country);


    // let country = String::from("china");
    // prints_country(country.clone());
    // prints_country(country);

    // let my_array = ["a"; 10];
    // println!("{:?}", my_array);

    // let array_of_ten = [1,2,3,4,5,6,7,8,9,10];
    // println!("{:?}", &array_of_ten[2..4]);
    // println!("{:?}", &array_of_ten[2..=4]);

    // 向量声明
    // let name1 = String::from("vivian");
    // let name2 = String::from("GOSIPY");
    //
    // // let mut my_vec = Vec::new();
    // let mut my_vec: Vec<String> = Vec::new();
    //
    // my_vec.push(name1);
    // my_vec.push(name2);
    // println!("{:?}", my_vec);

    // let my_vec = vec!["vivian", "GOSIPY"];
    // println!("{:?}", my_vec)

    // let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let s1 = &vec_of_ten[1..4]; // 1: [2, 3, 4]
    // let s2 = &vec_of_ten[1..]; //  [2, 3, 4, 5, 6, 7, 8, 9, 10]
    // let s3 = &vec_of_ten[..5]; //  [1, 2, 3, 4, 5]
    // let s4 = &vec_of_ten[..]; // [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    //
    // println!("s1: {:?}
    // s2: {:?}
    // s3: {:?}
    // s4: {:?}
    // ", s1, s2, s3, s4);

    // let mut num_vec = Vec::with_capacity(8);
    // num_vec.push("a");
    // println!("{}", num_vec.capacity())

    // let my_number: u8 = 5;
    // let res = match my_number {
    //     0 => {
    //         println!("it's zero");
    //         0
    //     },
    //     1 => {
    //         println!("it's one");
    //         1
    //     },
    //     2 => {
    //         println!("it's two");
    //         2
    //     },
    //     _ => {
    //         println!("It's some other number");
    //         3
    //     }
    // };
    //
    // println!("{}", res)

    // 流程控制 - match
    // let first = (200, 0, 0,);
    // match_colours(first)

    // 匹配项设置变量名
    match_number(50)
}
