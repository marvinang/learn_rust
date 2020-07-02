// Tip: 由于需要声明 compare 函数的第二参数必须与实现该特性的类型相同，
// 所以 Self （注意大小写）关键字就代表了当前类型（不是实例）本身。
trait Comparable {
    fn compare(&self, object: &Self) -> i8;
}

fn max<T: Comparable>(array: &[T]) -> &T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i].compare(&array[max_index]) > 0 {
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

impl Comparable for f64 {
    fn compare(&self, object: &f64) -> i8 {
        if &self > &object {
            1
        } else if &self == &object {
            0
        } else {
            -1
        }
    }
}

//================================= 官网 =========================
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

// 返回&T
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for &item in list {
        if item > *largest {
            largest = &item;
        }
    }
    largest
}

fn main() {
    let arr = [1.0, 3.0, 5.0, 4.0, 2.0];
    println!("maximum of arr is {}", max(&arr));

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
