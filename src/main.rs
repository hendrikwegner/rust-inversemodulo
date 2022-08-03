fn main() {
    //let a = std::env::args().nth(1).expect("no a given").parse::<i32>().unwrap();
    //let b = std::env::args().nth(2).expect("no b given").parse::<i32>().unwrap();


    let a = 31;
    let b = 26;
    let inverse = modinverse(a, b);
    if inverse != None {
        println!("{:?}", inverse.unwrap());
    } else {
        println!("For {} exists no distinct inverse modulo {}.", a, b);
    }
}

fn modinverse(a: i32, b: i32) -> Option<i32> {
    let res = gcd(a, b);
    if res.0 == 1 {
        //inverse modulo exists
        //extended euclidian algorithm

        let mut x = 0;
        let mut y = 1;

        let len = res.1.len() - 1;

        let mut table = vec![(0, 0, 0, 0, 0, 0); len];
        for i in (0..len).rev() {
            let tmp = x;
            x = y;
            y = tmp - res.1[i].2 * y;
            // y_i = x_i+1 - q_i*y_i+1
            table[i] = (res.1[i].0, res.1[i].1, res.1[i].2, res.1[i].3, x, y);
            // println!("a: {}, b: {}, q: {}, r: {}, x: {}, y: {}", res.1[i].0, res.1[i].1, res.1[i].2, res.1[i].3, x, y);
        }
        table.push((res.1[len].0, res.1[len].1, res.1[len].2, res.1[len].3, 0, 1));
        println!("{:?}", table);
        return Some(x.rem_euclid(b));
    }
    return None; // for a no inverse modulo b exists
}

fn gcd(mut a: i32, mut b: i32) -> (i32, Vec<(i32, i32, i32, i32)>) {
    let mut history: Vec<(i32, i32, i32, i32)> = Vec::new();
    let mut r: i32 = -1;
    while r != 0 {
        let q = a / b;
        r = a % b;

        history.push((a, b, q, r));

        a = b;
        b = r;
    }

    return (a, history); // a = gcd ; history for later extended euclidian algorithm
}