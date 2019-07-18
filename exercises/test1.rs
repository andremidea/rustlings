// test1.rs
// This is a test for the following sections:
// - Variables
// - Functions

// Mary is buying apples. One apple usually costs 2 dollars, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the order amount. No hints this time!

// I AM NOT DONE

// Put your function here!
fn calculateprice(units: u8) -> u8 {
    if (units > 40) {
        units
    } else {
        units * 2
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_apple_price(35);
    let price2 = calculate_apple_price(65);

    assert_eq!(70, price1);
    assert_eq!(65, price2);
    let price1 = calculateprice(55);
    let price2 = calculateprice(40);
    let price3 = calculateprice(10);


    assert_eq!(price1, 55);
    assert_eq!(price2, 80);
    assert_eq!(price3, 20);
}
