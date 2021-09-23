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

fn main() {
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

    let array_of_ten = [1,2,3,4,5,6,7,8,9,10];
    println!("{:?}", &array_of_ten[2..4]);
    println!("{:?}", &array_of_ten[2..=4]);

}
