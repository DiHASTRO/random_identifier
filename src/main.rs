use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic;
use std::sync::atomic::Ordering;
use std::env;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    
    let args: Vec<String> = env::args().collect();

    let two: u64 = 2;
    let time = SystemTime::now().duration_since(UNIX_EPOCH).expect("").as_secs() as u64;
    let proc_num = &args[1].parse().unwrap();
    let total: u64 = time * two.pow(33) + two.pow(23) * proc_num + 3;
    let total_a = atomic::AtomicU64::new(total);
    app.at("/").get(move |_| {
        let id = total_a.fetch_add(1, Ordering::Relaxed);
        let mut res_str = id.to_string();
        if id % two.pow(22) < 3 {
            res_str = "Error".to_string();
        }
        async move {
            Ok(format!("{}", res_str))
        }
    });
    app.listen(format!("127.0.0.1:{}", &args[2])).await?;
    Ok(())
}
