use std::fs;
use std::fmt::Display;
use std::fmt;

pub struct LogTimeStamp {
    pub hours: u64, 
    pub minutes: u64, 
    pub seconds: u64,
    pub days: u64, 
    pub months: u64, 
    pub years: u64,
    pub raw: u64,
}

impl Display for LogTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{} - {}:{}:{}", self.days, self.months, self.years, self.hours, self.minutes, self.seconds)
    }
}

pub fn get_current_time() -> LogTimeStamp {
    const YEAR: u64 = 365 * 24 * 60 * 60;
    const DAY: u64 = 24 * 60 * 60;
    const HOUR: u64 = 60 * 60;
    const MINUTE: u64 = 60;
    
    let time = std::time::SystemTime::now();

    let x = time
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut days = (x%YEAR - ((x / YEAR)/4) * DAY) / DAY + 1;
    let months: u64;
    if days <= 31 { months = 1;}
    else if days > 31 && days <= 59 {months = 2; days -= 31}
    else if days > 59 && days <= 90 {months = 3; days -= 59}
    else if days > 90 && days <= 120 {months = 4; days -= 90}
    else if days > 120 && days <= 151 {months = 5; days -= 120}
    else if days > 151 && days <= 181 {months = 6; days -= 151}
    else if days > 181 && days <= 212 {months = 7; days -= 181}
    else if days > 212 && days <= 243 {months = 8; days -= 212}
    else if days > 243 && days <= 273 {months = 9; days -= 243}
    else if days > 273 && days <= 304 {months = 10; days -= 273}
    else if days > 304 && days <= 334 {months = 11; days -= 304}
    else if days > 334 && days <= 366 {months = 12; days -= 334}
    else {months = 12;}

    LogTimeStamp {
        hours: (x%DAY)/HOUR, 
        minutes: (x%HOUR)/MINUTE, 
        seconds: x%MINUTE,
        days: days, 
        months: months, 
        years: 1970 + x / YEAR,
        raw: x,
    }

}


pub struct Logger {
    log: String,
    path: String,
    pub log_size: usize,
    logs_made: u16,
    loghead: LogTimeStamp,
}

impl Drop for Logger {
    fn drop(&mut self) {
        let stamp = get_current_time();
        let log_path = format!("{}/{}.txt", self.path, stamp);
            match fs::write(log_path, &self.log) {
                Ok(_) => self.logs_made += 1,
                Err(_) => panic!()

            };
    }
}

impl Logger {

    pub fn new(log_folder: String) -> Logger{
        Logger {
            log: "".to_owned(),
            path: log_folder,
            log_size: 0,
            logs_made: 0,
            loghead: get_current_time(),
        }
    }

    pub fn add(&mut self, s: &str) {
        if s.len() > 1024 {
            self.add("Attempted to log an entry larger than 1024 bytes");
        }
        let stamp = get_current_time();
        self.log.push_str(&format!("{}", &stamp));
        self.log.push(';');
        self.log.push_str(s);
        self.log_size = self.log.len();
        if self.log_size > 1024 {
            let log_path = format!("{}/{}.txt", self.path, self.loghead);
            match fs::write(log_path, &self.log) {
                Ok(_) => self.logs_made += 1,
                Err(_) => panic!()

            };
            self.log = "".to_owned();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Logger, get_current_time};
    #[test]
    fn test_destructor() {
        let mut logger = Logger::new("logs".to_owned());
        logger.add("Here is a log");
        if true {
            let mut logg = Logger::new("logs1".to_owned());
            logg.add("Here is a different log");
        }
        println!("Here something is happening");
    }

    #[test]
    fn test_too_big_log() {
        let mut logger = Logger::new("logs".to_owned());
        let mut s = "".to_owned();
        let mut i = 0;
        while i < 2000 {
            s.push('a');
            i += 1;
        }
        logger.add(&s);
    }

    #[test]
    fn test_timestamp() {
        println!("{}", get_current_time());
    }
}