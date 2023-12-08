pub fn sample1() -> i32 {
    let mut hoge = 100;
    for _ in 0..10{
        hoge = 200;
        
    }

    return hoge
}

pub fn sample2() ->  &'static str {
    let mut hoge_s = "";
    for _ in 0..10{      
        if hoge_s == ""{
            hoge_s = "100";
        }
    }

    return hoge_s
}

pub fn sample3() ->  &'static str {
    let mut hoge_s = "";
    let hoge_x = "x";
    for _ in 0..10{     
        if hoge_s == ""{
            hoge_s = hoge_x;
        }
    }

    return hoge_s
}


pub fn sample4() ->  String {
    let mut s_string = String::from("abc");
    let s_str = "XYZ";
    
    s_string.push_str(s_str);

    s_string
}

pub fn sample5() ->  String {
    let mut s_string = String::from("abc");
    let s_str = String::from("xyz");
    

    for _ in 0..10{     
        if s_string == "abc"{
            s_string.push_str(&s_str);
        }
    }

    s_string
}

pub fn sample6() ->  String {
    let mut s_string = String::from("abc");
    let s_str = String::from("xyz");
    

    for _ in 0..10{     
        if s_string == "abc"{
            s_string.push_str(&s_str);
        }
    }

    s_string
}

pub fn sample7() ->  String {
    let mut s_string = String::from("abc");
    let s_str = String::from("xyz");
    

    for _ in 0..10{     
        if s_string == "abc"{
            s_string = s_string + &s_str;
        }
    }

    s_string
}

pub fn sample8() ->  String {
    let mut s_string = String::from("abc");
    let s_str = String::from("xyz");
    

    for _ in 0..10{     
        if s_string == "abc"{
            s_string = s_string.clone() + &s_str;
        }
    }

    s_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(sample1(), 200);
        assert_eq!(sample2(), "100");
        assert_eq!(sample3(), "x");
        assert_eq!(sample4(), "abcXYZ");
        assert_eq!(sample5(), "abcxyz");
        assert_eq!(sample6(), "abcxyz");
        assert_eq!(sample7(), "abcxyz");
        assert_eq!(sample8(), "abcxyz");
    }
}
