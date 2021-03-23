fn boundary_max_page(x: i64) -> [i64; 2] {
    let ans_float: f64 = ((x * 2) as f64).sqrt();

    return [ans_float.floor() as i64, ans_float.ceil() as i64];
}

fn total(n: i64) -> i64 {
    return (n*(n+1)/2) as i64;
}


fn main() {
    let n: i64 = 300;

    let max_pages = boundary_max_page(n);

    for max_page in max_pages.iter() {
        let mut total_page: i64 = total(*max_page);
        let mut taked_max_page = *max_page;
        let diff: f64 = (total_page - n) as f64;
        let diff_dived: f64 = diff / 2.0_f64;
        let mut candidates: [i64; 2] = [diff_dived.floor() as i64, diff_dived.ceil() as i64];
        if total_page == n {
            total_page += max_page+1 + max_page+2;
            candidates = [max_page+1, max_page+2];
            taked_max_page += 2;
        }
        if candidates[0] % 2 == 0 {
            continue;
        }
        println!("max page: {}", taked_max_page);
        println!("total page: {}", total_page);
        println!("diff: {}", diff);
        println!("diff_dived: {}", diff_dived);
        println!("candidates: {:?}", candidates);
    }
}
