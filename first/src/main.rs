fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn to_fahrenheit(temp: f64) -> f64 {
    32.0 + ((9.0/5.0) * temp)
}

fn to_celsius(fahr: f64) -> f64 {
    (fahr * (5.0/9.0)) - (160.0/9.0)
}

fn time_difference(h1: i32, m1: i32, s1: i32, h2: i32, m2: i32, s2: i32) -> (i32, i32, i32) {
    let mut total1 = h1 * 3600 + m1 * 60 + s1;
    let mut total2 = h2 * 3600 + m2 * 60 + s2;

    if total1 > total2 {
        let temp = total1;
        total1 = total2;
        total2 = temp;
    }

    let diff = total2 - total1;
    (diff / 3600, (diff % 3600) / 60, diff % 60)
}

fn factorial(n: u64) -> u64 {
    let mut fac = 1;
    for i in 1..=n {
        fac *= i;
    }
    fac
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn sum_numbers(mut numbers: i32) -> i32 {
    let mut sum = 0;
    while numbers > 0 {
        sum += numbers % 10;
        numbers /= 10;
    }
    sum
}

fn pita(limit: i32) {
    for a in 1..=limit {
        for b in a+1..=limit {
            for c in b+1..=limit {
                if a * a + b * b == c * c {
                    println!("a:{} b:{} c:{}", a, b, c);
                }
            }
        }
    }
}

// fn asci(){
//     for code in 33..=126{
//         let ch = code as u8 as char;
//         println!("{}, {}", code, ch);
//     }
// }

fn collatz(n: i32) -> i32{
    let mut cnt = 0;
    let mut colat = n;

    while colat != 1 {
        if colat % 2 == 0 {
            colat = colat / 2;
        }
        else if colat % 2 != 0 {
            colat = 3 * colat + 1;
        }
        cnt += 1;
    }
    cnt
}

fn armstrong(mut n: i32) -> bool {
    let mut sum = 0;
    let mut cnt = 0;
    let original_n = n;

    while n > 0 {
        n /= 10;
        cnt += 1;
    }
    n = original_n;

    while n > 0 {
        let digit = n % 10;
        sum += digit.pow(cnt);
        n /= 10;
    }
    sum == original_n
}

fn perfect(mut n: i32) -> bool {
    let mut original = n;
    let mut dzielnik = 0;
    for i in 1..original {
        if original % i == 0 {
            dzielnik += i;
        }
    }
    dzielnik == original
}

fn pow_mod(mut x: u128, mut n: u128, p: u128) -> u128 {
    let mut result = 1u128;
    x = x % p;
    while n > 0 {
        if n % 2 == 1 {
            result = (result * x) % p;
        }
        x = (x * x) % p;
        n /= 2;
    }
    result
}

fn swap(a: &mut i32, b: &mut i32){
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn sort(a: &mut i32, b: &mut i32, c: &mut i32) {
    if *a > *b{
        swap(a,b);
    }
    if *a > *c {
        swap(a,c);
    }
    if *b > *c {
        swap(b,c);
    }
}

fn rand(seed: &mut u32, min_rand: i32, max_rand: i32) -> i32 {
    let a: u32 = 1664525;
    let c: u32 = 1013904223;
    *seed = a.wrapping_mul(*seed).wrapping_add(c);
    let range: u32 = (max_rand - min_rand + 1) as u32;
    ((*seed) % range) as i32 + min_rand
}

use std::sync::mpsc::Receiver;
use rand::Rng;

fn swap_arr(arr: &mut [i32], i: usize, j: usize){
    arr.swap(i, j);
}

fn rand_perm(arr: &mut [i32], seed: &mut u32){
    for i in (1..arr.len()).rev() {
        let j: usize = rand(seed, 0, i as i32) as usize;
        swap_arr(arr, i, j);
    }
}

fn liczba_wystąpien(napis: &str, znak: char) -> i32{
    let mut cnt = 0;
    let iter = napis.chars();
    for c in iter {
        if c == znak {
            cnt += 1;
        }
    }
    cnt
}

fn rzymskie(napis: &str) -> i32{
    let mut res = 0;
    let mut prev = 0;
    for c in napis.chars().rev(){
        let roman = match c{
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        if roman < prev {
            res -= roman;
        } else {
            res += roman;
        }
        prev = roman;
    }
    res
}

fn co_drugi_znak(napis: &str) -> String {
    napis.chars().step_by(2).collect()
}

fn szyfruj(napis: &str, klucz: usize) -> String {
    let mut encoded = String::new();
    let _slice: &str = &napis[0..3];
    for i in (0..napis.len()).step_by(klucz) {
        if i+klucz < napis.len() {
            encoded += napis[i..i+klucz].chars().rev().collect::<String>().as_str();
        } else {
            encoded += napis[i..].chars().rev().collect::<String>().as_str();
        }
    }
    encoded
}

fn wizytowka(name: &str, surname: &str) -> String {
    let first = surname.chars().take(1).collect::<String>().to_uppercase();
    let rest = surname.chars().skip(1).collect::<String>().to_lowercase();
    let word = name.chars().take(1).collect::<String>().to_uppercase();
    word + ". " + &first + &rest

}

fn na_rzymskie(mut n: i32) -> String {
    let numbers = [
        (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"), (100, "C"),
        (90, "XC"), (50, "L"), (40, "XL"), (10, "X"), (9, "IX"), (5, "V"),
        (4, "IV"), (1, "I")
    ];
    let mut res = String::new();
    for(val, symbol) in numbers {
        res.push_str(&symbol.repeat((n/val) as usize));
        n = n % val;
    }
    res
}

fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let mut res = String::new();
    let mut i1 = a.chars().rev().peekable();
    let mut i2 = b.chars().rev().peekable();
    let mut moved = 0;
    while i1.peek() != None || i2.peek() != None || moved != 0 {
        let va = i1.next().unwrap_or('0').to_digit(10).unwrap_or(0);
        let vb = i2.next().unwrap_or('0').to_digit(10).unwrap();

        let sum = va+vb+moved;
        moved = sum / 10;
        let ch = char::from_digit(sum % 10, 10).unwrap();
        res.push(ch);
    }
    res.chars().rev().collect()
}


fn filter_vecs1(vec: Vec<String>) -> Vec<String> {
    vec.into_iter().filter(|s| s.len() < 4).collect()
}

fn filter_vecs2(vec: Vec<String>) -> Vec<String> {
    vec.into_iter().filter(|s| !s.contains('a') && !s.contains('A')).collect()
}

fn filter_vecs3(vec: Vec<String>) -> Vec<String> {
    vec.into_iter().filter(|s| s.chars().any(|c| c.is_ascii_digit())).collect()
}

fn filter_vecs4(vec: Vec<String>) -> Vec<String> {
    vec.into_iter().map(|s| s.chars().rev().collect()).collect()
}

fn filter_vecs5(vec: Vec<String>) -> Vec<String> {
    vec.into_iter().filter(|s| { s.chars().zip(s.chars().skip(1)).any(|(a,b)| a == b) }).collect()
}

fn indeksy(tablica: &[String], element: &str) -> Vec<usize> {
    tablica.iter().enumerate().filter_map(|(i,s)| if s == element {Some(i)} else {None}).collect()
}

fn email(element: &str) -> bool{
    let parts: Vec<&str> = element.split("@").collect();
    if parts.len() != 2 {
        return false;
    }
    let before = parts[0];
    let after = parts[1];

    let first = element.chars().next();
    let last = element.chars().rev().next();
    if let (Some(f), Some(l)) = (first, last) {
        if !f.is_ascii_alphanumeric() || !l.is_ascii_alphanumeric() {
            return false;
        }
    } else {
        return true;
    }

    let mut cnt = 0;
    for c in element.chars() {
        if c == '@'{
            cnt += 1;
        } else if !c.is_ascii_alphanumeric() && c != '.'{
            return false;
        }
    }
    if cnt != 1{
        return false;
    }
    if !after.contains('.') {
        return false;
    }
    true
}




struct Set{
    elements: Vec<u32>,
}

impl Set {
    pub fn new() -> Set {
        Set {
            elements: Vec::new(),
        }
    }

    fn from_slice(slice: &[u32]) -> Set {
        let mut elements: Vec<u32> = slice.to_vec();
        elements.sort_unstable();
        elements.dedup();
        Set { elements }
    }

    fn union(&self, other: &Set) -> Set {
        let mut result = self.elements.clone();
        for &elem in &other.elements {
            if !result.contains(&elem) {
                result.push(elem);
            }
        }
        result.sort_unstable();
        Set { elements: result }
    }
}

#[derive(Debug, Clone)]
struct Transaction{
    value: f32,
    from: String,
    to: String,
}

struct BankAccount{
    account: String,
    history: Vec<Transaction>,
}

impl BankAccount{
    fn new(number: &str) -> Self{
        BankAccount{
            account: number.to_string(),
            history: Vec::new(),
        }
    }

    fn transaction(&mut self, t: Transaction){
        if t.to == self.account || t.from == self.account {
            self.history.push(t);
        }
    }

    fn balance(&self) -> f32 {
        let mut balance = 0.0;
        for i in &self.history {
            if i.to == self.account{
                balance += i.value;
            } else if i.from == self.account {
                balance -= i.value;
            }
        }
        balance
    }

    fn last(&self) -> Option<Transaction>{
        self.history.last().cloned()
    }
}

    fn control(s: &str) -> u32 {
        let weights = [1, 3, 7, 9, 1, 3, 7, 9, 1, 3];
        let mut sum = 0;

        for (i, c) in s.chars().take(10).enumerate() {
            let digit = c.to_digit(10).unwrap();
            sum += digit * weights[i];
        }

        let m = sum % 10;
        if m == 0 {
            0
        } else {
            10 - m
        }
    }
    fn pesel(s: &str) -> bool{
        if s.len() != 11  {
            return false;
        }
        if !s.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        let expected = control(s);
        let last_digit = s.chars().last().unwrap().to_digit(10).unwrap();

        expected == last_digit
    }




fn main() {
    println!("{}", is_leap_year(2024));
    println!("{}", to_fahrenheit(5.0));
    println!("{}", to_celsius(41.0));
    let (h,m,s) = time_difference(14,10,20,15,10,20);
    println!("{}h {}m {}s", h, m, s );
    println!("{}",factorial(20));
    println!("{}", reverse("987654321"));
    println!("{}", sum_numbers(12345));
    pita(13);
    //asci();
    let steps = collatz(12);
    println!("{}", steps);
    println!("{}", armstrong(1634));
    println!("{}", perfect(7));
    println!("{}", pow_mod(2, 10, 5));
    let mut a = 5;
    let mut b = 0;
    let mut c = 2;
    sort(&mut a, &mut b, &mut c);
    println!("a{}, b{}, c{}", a, b, c);
    let mut seed: u32 = 44;
    println!("Wygenerowane liczby: ");
    for _ in 0..5 {
        let r = rand(&mut seed, 0, 100);
        println!("{r}");
    }
    let mut rng = rand::thread_rng();
    println!("Wygenerowane liczby: ");
    for _ in 0..5 {
        let random = rng.gen_range(0..=100);
        println!("{random}");
    }
    let mut arr: [i32; 5] = [1,2,3,4,5];
    swap_arr(&mut arr, 1, 3);
    println!("{:?}", arr);
    rand_perm(&mut arr, &mut seed);
    println!("{arr:?}");
    let s = "ala ma kota ";
    println!("'a' w '{s}' : {}", liczba_wystąpien(s, 'a'));
    println!("{}", rzymskie("MMXXV"));
    println!("{}", co_drugi_znak("adniarl"));
    println!("{}", szyfruj("Aladyn", 2));
    println!("{}", wizytowka("oskar", "sigma"));
    println!("{}", na_rzymskie(9));
    println!("{}", dodaj_pisemnie("12", "12"));

    // petla
    let mut letters = Vec::new();
    for ch in 'a'..='z'{
        letters.push(ch);
    }
    println!("{:?}", letters);

    // iterator
    let letters_: Vec<char> = ('a'..='z').collect();
    println!("{:?}", letters_);

    let mut numbers = Vec::new();
    for i in 1..=10{
        numbers.push(i*i);
    }
    println!("{:?}", numbers);

    let numbers_: Vec<i32> = (1..=10).map(|x| x * x).collect();

    println!("{:?}", numbers_);

    let mut reverse = Vec::new();
    for i in 1..=20{
        reverse.push(i);
    }
    reverse.reverse();
    println!("{:?}", reverse);

    let mut reverse_: Vec<i32> = (1..=20).collect();
    reverse_.reverse();
    println!("{:?}", reverse_);

    let mut xd:Vec<f64> = Vec::new();
    for i in 1..=20{
        xd.push(1.0/i as f64);
    }
    println!("{:?}", xd);

    let mut xd_: Vec<f64> = (1..=20).map(|x| 1.0 / x as f64).collect();
    println!("{:?}", xd_);

    let mut dzielne = Vec::new();
    for i in 1..=100{
        if (i % 3 == 0) && (i % 4 != 0) {
            dzielne.push(i);
        }
    }
    println!("{:?}", dzielne);

    let mut dzielne_: Vec<i32> = (1..=100).filter(|&n| n % 3 == 0 && n % 4 != 0).collect();
    println!("{:?}", dzielne_);

    let input = vec![
        String::from("a"),
        String::from("abc"),
        String::from("abcd"),
        String::from("hello"),
        String::from("xy"),
    ];
    let input2 = vec![
        String::from("apple"),
        String::from("Banana"),
        String::from("Cherry"),
        String::from("Dog"),
        String::from("egg"),
    ];
    let input3 = vec![
        String::from("hello"),
        String::from("test123"),
        String::from("42"),
        String::from("abc"),
        String::from("no_digits"),
    ];
    let input4 = vec![
        String::from("hello"),
        String::from("test123"),
        String::from("42"),
        String::from("abc"),
        String::from("no_digits"),
    ];
    let input5 = vec![
        String::from("inny"),
        String::from("pizza"),
        String::from("brutto"),
        String::from("lekki"),
        String::from("dzienny"),
        String::from("kot"),
        String::from("pies"),
    ];
    let short_strings = filter_vecs1(input);
    println!("{:?}", short_strings);
    let short_strings2 = filter_vecs2(input2);
    println!("{:?}", short_strings2);
    let short_strings3 = filter_vecs3(input3);
    println!("{:?}", short_strings3);
    let short_strings4 = filter_vecs4(input4);
    println!("{:?}", short_strings4);
    let short_strings5 = filter_vecs5(input5);
    println!("{:?}", short_strings5);

    let tablica = vec![
        String::from("kot"),
        String::from("pies"),
        String::from("kot"),
        String::from("ptak"),
        String::from("pies"),
    ];

    let wynik = indeksy(&tablica, "pies");

    println!("{:?}", wynik);

    let emails = [
        "test@example.com",     // true
        "a.b@domain.pl",        // true
        "@domain.com",          // false
        "user@domain",          // false
        "userdomain.com",       // false
        "user@domaincom",       // false
        "user@domain.c",        // true
        "user@.com",            // true
        "user!@example.com",    // false
    ];

    for e in emails.iter() {
        println!("{} -> {}", e, email(e));
    }
}

