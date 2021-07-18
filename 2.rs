//这个例子要求实现外部特性，检查流量并发送消息
#![allow(unused_variables)]
fn main() {
pub trait Messenger {
    fn send(&self, msg: &str);  //外部特性，不能改动
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
//这样的语法哪里都没讲，拿来就用
//<'a，表示本结构的生命期，结构存在的时候，所有引用元素都得活着
//结构的元素有泛型，所以得声明，该元素有生命期，所以泛型也得声明生命期，T: 'a

//该泛型要实施Messenger, 只好用个加号，T:'a + Messenger
//附录B, table B-1, Operators, "+", Table B-5: Trait Bound Constraints
//trait + trait, 'a + trai: Compound type constraint
//差评差评差评

//全部加起来<'a, T: 'a + Messenger>
//应该等价于，<'a, T: 'a> where T: Messenger

    messenger: &'a T, //'a&T不好吗？
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>  //没有<'a, T: 'a>
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> { 
    //不返回指针， &T的生命期隐含在定义中, 参数只有一个指针，没有歧义，所以不需要<'a, T: 'a>。猜测。
    //总之，如果不是明确需要生命期先不要管，反正编译器会告诉你
        LimitTracker {
            messenger,  //messenger=messenger, 域名与参数同名，简化赋值
            value: 0,
            max,  //max=max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger { sent_messages: vec![] }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();  //sent_messages==vec![]
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);  

        limit_tracker.set_value(80);  
        //调用MockMessenger.send(), 改变sent_messages大小，不可变，编译不通过

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}

