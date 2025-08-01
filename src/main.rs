use std::{thread::sleep, time::Duration};
use std::time::SystemTime;
use std::io::{self, Write};
fn main() {
    println!("{} \x1b[34m[INFO]\x1b[0m (1) Starting Pumpkin 0.1.0-dev+1.21.7 for Minecraft 1.21.7 (Protocol 772)", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[33m[WARN]\x1b[0m Pumpkin is currently under heavy development!", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[34m[INFO]\x1b[0m (1) Report issues on https://github.com/Pumpkin-MC/Pumpkin/issues", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[34m[INFO]\x1b[0m (1) Join our Discord for community support: https://discord.com/invite/wT8XjrjKkf", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[34m[INFO]\x1b[0m (1) Loading Overworld: -1404760116303608497", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[34m[INFO]\x1b[0m (1) Loading Nether: -1404760116303608497", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[34m[INFO]\x1b[0m (1) Loading End: -1404760116303608497", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[33m[WARN]\x1b[0m Failed to load favicon from 'icon.png': not found; using default.", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[34m[INFO]\x1b[0m (1) Query protocol is enabled. Starting...", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[34m[INFO]\x1b[0m (1) Started server; took 830ms", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[34m[INFO]\x1b[0m (7) Server query running on 0.0.0.0:25565", get_local_time_string());
    delay_ms(500, 2000);
    println!("{} \x1b[34m[INFO]\x1b[0m (1) Server is now running. Connect using: Java Edition: 0.0.0.0:25565 | Bedrock/Pocket Edition: 0.0.0.0:19132", get_local_time_string());
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        delay_ms(50, 100);
        
        if input.trim() == "stop" {
            println!("{} \x1b[34m[INFO]\x1b[0m (1) Stopping server...", get_local_time_string());
            delay_ms(500, 2000);
            println!("{} \x1b[34m[INFO]\x1b[0m (1) Server stopped successfully.", get_local_time_string());
            break;
        } else {
            println!("{} \x1b[31m[ERROR]\x1b[0m Unknown command: {}", get_local_time_string(), input.trim());
        } }
}



fn delay_ms(min_ms: u64, max_ms: u64) {
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .subsec_nanos();

    let range = max_ms - min_ms + 1;
    let delay_ms = min_ms + (nanos as u64 % range);
    sleep(Duration::from_millis(delay_ms));
}

#[cfg(windows)]
fn get_local_time_string() -> String {
    use std::mem::MaybeUninit;
    use libc::{time, time_t, localtime_s, tm};

    unsafe {
        let mut raw_time: time_t = 0;
        time(&mut raw_time);

        let mut result_tm: MaybeUninit<tm> = MaybeUninit::uninit();
        localtime_s(result_tm.as_mut_ptr(), &raw_time);
        let tm = result_tm.assume_init();
        format!(
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            tm.tm_year + 1900,
            tm.tm_mon + 1,
            tm.tm_mday,
            tm.tm_hour,
            tm.tm_min,
            tm.tm_sec
        )
    }
}
