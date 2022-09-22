//! [![](https://gitlab.com/andrew_ryan/doe/-/raw/master/doe.svg)](https://docs.rs/doe)

#![doc(
    html_logo_url = "https://gitlab.com/andrew_ryan/doe/-/raw/master/html_logo.svg",
    html_favicon_url = "https://gitlab.com/andrew_ryan/doe/-/raw/master/html_favicon.svg"
)]


#[allow(warnings)]
#[macro_use]
pub mod macros {
    /// as_to!() can be used to
    /// convert type
    /// ```rust
    ///  use doe::*;
    ///  let s = as_to!(5., i64);
    ///  assert_eq!(5, s);
    /// ```
    #[macro_export]
    macro_rules! as_to { 
        ( $a:expr,$type:ty ) => {
            ($a as $type)
        };
    }
    use std::any::Any;
    ///  if type is i8 return true
    /// ```rust
    /// use doe::*;
    /// assert_eq!(macros::is_i8(&5_i8),true);
    /// ```
    pub fn is_i8(s: &dyn Any) -> bool {
        if s.is::<i8>() {
            return true;
        } else {
            return false;
        }
    }
    ///  if type is i16 return true
    /// ```rust
    /// use doe::*;
    /// assert_eq!(macros::is_i16(&5_i16),true);
    /// ```
    pub fn is_i16(s: &dyn Any) -> bool {
        if s.is::<i16>() {
            return true;
        } else {
            return false;
        }
    }
    ///  if type is i32 return true
    /// ```rust
    /// use doe::*;
    /// assert_eq!(macros::is_i32(&5_i32),true);
    /// ```
    pub fn is_i32(s: &dyn Any) -> bool {
        if s.is::<i32>() {
            return true;
        } else {
            return false;
        }
    }
    ///  if type is i64 return true
    /// ```rust
    /// use doe::*;
    /// assert_eq!(macros::is_i64(&5_i64),true);
    /// ```
    pub fn is_i64(s: &dyn Any) -> bool {
        if s.is::<i64>() {
            return true;
        } else {
            return false;
        }
    }
    ///  if type is i128 return true
    /// ```rust
    /// use doe::*;
    /// assert_eq!(macros::is_i128(&5_i128),true);
    /// ```
    pub fn is_i128(s: &dyn Any) -> bool {
        if s.is::<i128>() {
            return true;
        } else {
            return false;
        }
    }
    ///  if type is f32 return true
    /// ```rust
    /// use doe::*;
    /// assert_eq!(macros::is_f32(&5.0_f32),true);
    /// ```
    pub fn is_f32(s: &dyn Any) -> bool {
        if s.is::<f32>() {
            return true;
        } else {
            return false;
        }
    }
    ///  if type is f64 return true
    /// ```rust
    /// use doe::*;
    /// assert_eq!(macros::is_f64(&5.0_f64),true);
    /// ```
    pub fn is_f64(s: &dyn Any) -> bool {
        if s.is::<f64>() {
            return true;
        } else {
            return false;
        }
    }

    pub fn pow<T>(a: T, b: T) -> T
    where
        f64: From<T>,
        T: 'static
            + std::ops::MulAssign
            + std::fmt::Display
            + std::ops::Mul<Output = T>
            + Copy
            + std::convert::From<f64>,
    {
        let mut re = a;
        let aa: f64 = a.into();
        let bb: f64 = b.into();
        re = f64::powf(aa, bb).into();
        re
    }
    /// powf!(a,b) can be used to
    /// a to the power of b
    /// ```rust
    /// use doe::*;
    /// let p = powf!(2.,2.);
    /// assert_eq!(p,4.0);
    /// ```
    #[macro_export]
    macro_rules! powf {
        ($a:expr,$b:expr) => {
            f64::powf($a, $b)
        };
    }
    /// arg!() can be used to
    ///get argument and collect to Vec\<String\>
    /// ```ignore
    /// use doe::*;
    /// //cargo run -- -n 100
    /// let arg = args!();
    /// assert_eq!(arg,vec![format!("-n"),format!("100")]);
    /// ```
    #[macro_export]
    macro_rules! args {
        () => {
            std::env::args().skip(1).collect::<Vec<String>>()
        };
    }
    /// input!() can be used to
    /// get user input from terminal
    /// ```rust
    /// use doe::*;
    /// let s = input!();
    /// println!("{:?}",s);
    /// ```
    #[macro_export]
    macro_rules! input {
        () => {{
            let mut string = String::new();
            std::io::stdin().read_line(&mut string).unwrap();
            string = string.to_string().trim().to_owned();
            string
        }};
    }
    /// split_to_vec!() can be used to
    /// a:&str split by b:&str and collect to Vec\<String\>
    /// ```rust
    /// use doe::*;
    /// let s = split_to_vec!("aa.bb",".");
    /// assert_eq!(s,vec![format!("aa"),format!("bb")]);
    /// ```
    #[macro_export]
    macro_rules! split_to_vec {
        ($a:expr,$b:expr) => {
            $a.to_string()
                .split($b)
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string().trim().to_owned())
                .collect::<Vec<String>>()
        };
    }
    /// read_csv!() can be used to
    /// read .csv file and return Vec\<Vec\<String\>\>
    /// ```rust
    /// use doe::*;
    ///let s = read_csv!("./data.csv");
    ///assert_eq!(s,vec![vec![format!("a"), format!("b"), format!("c")],vec![format!("1"), format!("2"), format!("3")],vec![format!("10"), format!("20"), format!("30")]]);
    /// ```
    #[macro_export]
    macro_rules! read_csv {
        ($path:expr) => {{
            let data = std::fs::read($path).unwrap();
            let data_vec = split_to_vec!(&String::from_utf8_lossy(&data), "\n");
            let data_each_vec = data_vec
                .iter()
                .map(|s| split_to_vec!(s, ","))
                .collect::<Vec<_>>();
            data_each_vec
        }};
    }
    /// sorted!() can be used to
    /// return sorted new Vec,
    /// type can be Vec\<i32\> Vec\<i64\> Vec\<i128\> Vec\<f32\> Vec\<f64\>   
    /// ```rust
    /// use doe::*;
    /// let s1 = sorted!(vec![1.2, 2.6, 0.2]);
    /// let s2 = sorted!(vec![8, 1_i128, 5_i128]);
    /// assert_eq!(s1,vec![0.2,1.2,2.6]);
    /// assert_eq!(s2,vec![1,5,8]);
    /// ```
    #[macro_export]
    macro_rules! sorted {
        ($vec:expr) => {{
            let mut vec = $vec.clone();
            vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
            vec
        }};
    }
    /// deduped_sorted!() can be used to
    /// return sorted and deduped new Vec,
    /// type can be Vec\<i32\> Vec\<i64\> Vec\<i128\> Vec\<f32\> Vec\<f64\>   
    /// ```rust
    /// use doe::*;
    /// let s1 = deduped_sorted!(vec![1.2, 1.2,2.6, 0.2]);
    /// let s2 = deduped_sorted!(vec![8, 1_i128,8,5_i128]);
    /// assert_eq!(s1,vec![0.2,1.2,2.6]);
    /// assert_eq!(s2,vec![1,5,8]);
    /// ```
    #[macro_export]
    macro_rules! deduped_sorted {
        ($vec:expr) => {{
            let mut vec1 = $vec.clone();
            let mut vec2 = sorted!(vec1);
            vec2.dedup();
            vec2
        }};
    }
    /// vec_element_parse!() can be used to parse a vec element to type
    /// need a Vec\<&str\>,parse type
    /// return Vec\<type\>
    /// ```rust
    /// use doe::*;
    ///let v1: Vec<f64> = vec_element_parse!(vec!["15.", "2.9"], f64);
    ///let v2: Vec<i128> = vec_element_parse!(vec!["15", "2"], i128);
    ///let v3: Vec<f32> = vec_element_parse!(vec![".15", ".2"], f32);
    ///assert_eq!(vec![15.0, 2.9], v1);
    ///assert_eq!(vec![15, 2], v2);
    ///assert_eq!(vec![0.15, 0.2], v3);
    /// ```
    #[macro_export]
    macro_rules! vec_element_parse {
        ($vec:expr,$type:ty) => {{
            let mut v2: Vec<$type> = Vec::new();
            if $vec.len() > 0 {
                match vec_element_clone!($vec, 0).parse::<$type>() {
                    Ok(_r) => {
                        let vec1 = $vec.clone();
                        v2 = vec1
                            .iter()
                            .map(|x| x.to_string().parse::<$type>().unwrap())
                            .collect::<Vec<_>>();
                    }
                    Err(_e) => {}
                }
            }
            v2
        }};
    }
    /// vec_element_to_string!() used to
    /// convert vec item to String,
    /// type can be Vec\<i32\> Vec\<i64\> Vec\<i128\> Vec\<f32\> Vec\<f64\>,Vec\<&str\>
    /// return Vec\<String\>
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_element_to_string!(vec!["15.", "2.9"]);
    ///let v2 = vec_element_to_string!(vec![15, 2]);
    ///let v3 = vec_element_to_string!(vec![0.15, 0.2]);
    ///assert_eq!(vec!["15.", "2.9"], v1);
    ///assert_eq!(vec!["15", "2"], v2);
    ///assert_eq!(vec!["0.15", "0.2"], v3);
    /// ```
    #[macro_export]
    macro_rules! vec_element_to_string {
        ($vec:expr) => {{
            let mut v2: Vec<String> = Vec::new();
            if $vec.len() > 0 {
                let vec1 = $vec.clone();
                v2 = vec1
                    .iter()
                    .map(|x| format!("{}", &x))
                    .collect::<Vec<String>>();
            }
            v2
        }};
    }
    /// snail_sort!() return the array elements arranged from outermost
    /// elements to the middle element, traveling clockwise.n x n
    /// type can be Vec\<Vec\<T\>\>
    /// return Vec\<T\>
    /// ```rust
    /// use doe::*;
    ///let v1 = snail_sort!(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    ///let v2 =  snail_sort!(vec![vec![1.1, 2.1, 3.1],vec![4.1, 5.1, 6.1],vec![7.1, 8.1, 9.1]]);
    ///assert_eq!(vec![1, 2, 3, 6, 9, 8, 7, 4, 5], v1);
    ///assert_eq!(vec![1.1, 2.1, 3.1, 6.1, 9.1, 8.1, 7.1, 4.1, 5.1], v2);
    /// ```
    #[macro_export]
    macro_rules! snail_sort {
        ($vec:expr) => {{
            fn snail<T: Copy>(matrix: &[Vec<T>]) -> Vec<T> {
                let mut ret = Vec::new();
                if matrix.len() == 0 {
                    return ret;
                }
                let mut width = matrix[0].len();
                let mut height = matrix.len();
                let mut cycle = 0;
                while width > 0 && height > 0 {
                    for x in cycle..width {
                        ret.push(matrix[cycle][x]);
                    }
                    for y in cycle + 1..height {
                        ret.push(matrix[y][width - 1]);
                    }
                    for x in (cycle..width - 1).rev() {
                        ret.push(matrix[height - 1][x]);
                    }
                    for y in (cycle + 1..height - 1).rev() {
                        ret.push(matrix[y][cycle]);
                    }
                    cycle += 1;
                    width -= 1;
                    height -= 1;
                }
                ret
            }
            let vec = $vec.clone();
            snail(&vec)
        }};
    }

    /// multiply_matrix!() return the mutiply result of two matrix
    /// take two matrix and type can be Vec\<Vec\<f64\>\>
    /// return Vec\<f64\>
    /// ```rust
    /// use doe::*;
    ///let m1: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![1.0, 2.0]];
    ///let m2: Vec<Vec<f64>> = vec![vec![0.0, 0.0], vec![0.0, 0.5]];
    ///let mul_result = multiply_matrix!(&m2, &m1);
    ///assert_eq!(mul_result, vec![[0.0, 0.0], [0.5, 1.0]]);
    /// ```
    #[macro_export]
    macro_rules! multiply_matrix {
        ($vec1:expr,$vec2:expr) => {{
            pub fn new(m: usize, n: usize) -> Vec<Vec<f64>> {
                //returb 0 created n*m matrix
                let mut mat = Vec::new();
                for i in 0..m {
                    mat.push(vec![0; n].iter().map(|s| *s as f64).collect::<Vec<_>>());
                }
                let vec = mat.clone();
                vec
            }
            pub fn multiply_matrix<'a>(
                matrix1: &'a Vec<Vec<f64>>,
                matrix2: &'a Vec<Vec<f64>>,
            ) -> Vec<Vec<f64>> {
                let m = &matrix2.len();
                let n = &matrix1[0].len();
                let mut matrix3: Vec<Vec<f64>> = new(*&matrix2.len(), *&matrix2[0].len());
                for i in 0..*&matrix2.len() {
                    for j in 0..*&matrix2[0].len() {
                        for k in 0..*m {
                            matrix3[i][j] += matrix1[i][k] * matrix2[k][j];
                        }
                    }
                }
                matrix3
            }
            let vec1 = $vec1.clone();
            let vec2 = $vec2.clone();
            multiply_matrix(&vec1, &vec2)
        }};
    }
    /// vec_element_remove!() used to
    /// find the element fist position and remove
    /// if not fond the element return original vec
    /// type can be Vec\<i32\> Vec\<i64\> Vec\<i128\> Vec\<f32\> Vec\<f64\>,Vec\<&str\>
    /// return return the same type
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_element_remove!(vec!["15.", "2.9"], "2.9");
    ///let v2 = vec_element_remove!(vec![15, 2, 3, 2], 2);
    ///let v3 = vec_element_remove!(vec![0.15, 0.2, 0.2], 0.2);
    ///assert_eq!(vec!["15."], v1);
    ///assert_eq!(vec![15, 3, 2], v2);
    ///assert_eq!(vec![0.15, 0.2], v3);
    /// ```
    #[macro_export]
    macro_rules! vec_element_remove {
        ($vec:expr,$element:expr) => {{
            let mut vec = $vec.clone();
            match &vec.binary_search_by(|probe| probe.partial_cmp(&$element).unwrap()) {
                Ok(r) => {
                    vec.remove(*r);
                }
                Err(r) => {}
            }
            vec
        };};
    }
    /// vec_element_remove_all!() used to
    /// find the element all position and remove
    /// if not fond the element return original vec
    /// type can be Vec\<i32\> Vec\<i64\> Vec\<i128\> Vec\<f32\> Vec\<f64\>,Vec\<&str\>
    /// return return the same type
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_element_remove_all!(vec!["15.", "2.9", "0.9", "2.9", "2.9"], "2.9");
    ///assert_eq!(vec!["15.", "0.9"], v1);
    ///let v2 = vec_element_remove_all!(vec![15, 2, 3, 2], 2);
    ///assert_eq!(vec![15, 3], v2);
    ///let v3 = vec_element_remove_all!(vec![0.15, 0.2, 1.0, 0.2], 0.3);
    ///assert_eq!(vec![0.15, 0.2, 1.0, 0.2], v3);
    /// ```
    #[macro_export]
    macro_rules! vec_element_remove_all {
        ($vec:expr,$element:expr) => {{
            let mut vec = $vec.clone();
            fn remove<T: Copy + PartialOrd<T>>(vec: Vec<T>, target: &T) -> Vec<T> {
                let mut v1 = vec.clone();
                for i in 1..v1.len() - 1 {
                    if v1[i - 1] == *target {
                        v1.remove(i - 1);
                    }
                }
                if v1[v1.len() - 1] == *target {
                    v1.remove(v1.len() - 1);
                }
                v1
            }
            remove(vec, &$element)
        };};
    }
    /// vec_element_position_all!() used to find element position and collect into Vec<usize>
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_element_position_all!(vec![1, 2, 5, 3, 6, 2, 2], 2);
    ///assert_eq!(v1, vec![1, 5, 6]);
    /// ```
    #[macro_export]
    macro_rules! vec_element_position_all {
        ($vec:expr,$element:expr) => {{
            let mut vec = $vec.clone();
            fn position<T: Copy + PartialOrd<T>>(vec: Vec<T>, target: &T) -> Vec<usize> {
                let mut v1 = vec.clone();
                let mut re = vec![];
                for i in 0..v1.len() {
                    if v1[i] == *target {
                        re.push(i);
                    }
                }
                re
            }
            position(vec, &$element)
        };};
    }

    /// vec_slice!() used to
    /// slice vec by range
    /// return return the same type sliced vec
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_slice!(vec![1.2, 1.5, 9.0], [..2]);
    ///let v2 = vec_slice!(vec![1, 1, 9, 90, 87, 0, 2], [4..6]);
    ///let v3 = vec_slice!(vec![1.2, 1.5, 9.0], [..]);
    ///let v4 = vec_slice!(vec![1.2, 1.5, 9.0], [1..]);
    ///let v5 = vec_slice!(vec!["1", "2", "3", "4", "5"], [2..5]);
    ///let v6 = vec_slice!(vec!["1".to_string(),"2".to_string(),"3".to_string()], [1..]);
    ///assert_eq!(v1, vec![1.2, 1.5]);
    ///assert_eq!(v2, vec![87, 0]);
    ///assert_eq!(v3, vec![1.2, 1.5, 9.0]);
    ///assert_eq!(v4, vec![1.5, 9.0]);
    ///assert_eq!(v5, vec!["3", "4", "5"]);
    ///assert_eq!(v6,vec!["2".to_string(),"3".to_string()]);
    /// ```
    #[macro_export]
    macro_rules! vec_slice {
        ($vec:expr,$slice:expr) => {{
            let vec = $vec.clone();
            let slice = $slice.clone();
            fn slice_func_1<T: Copy>(vec: Vec<T>, start: usize, stop: usize) -> Vec<T> {
                let mut re = vec![];
                for (index, item) in vec.iter().enumerate() {
                    if index >= start && index < stop {
                        re.push(vec_element_clone!(vec,index));
                    }
                }
                re
            }
            fn slice_func<T:Clone>(vec: Vec<T>, start: usize, stop: usize) -> Vec<T> {
                let mut re = vec![];
                for (index, item) in vec.iter().enumerate() {
                    if index >= start && index < stop {
                        re.push(vec_element_clone!(vec,index));
                    }
                }
                re
            }
            let s = format!("{:?}", slice);
            let mut start = 0;
            let mut stop = vec.len();
            let s_vec1 = doe::split_to_vec!(s, "[");
            let s_vec2 = doe::split_to_vec!(s_vec1[0], "]");
            let s_vec22: Vec<char> = s_vec2[0].chars().collect();
            let s_vec3 = doe::split_to_vec!(s_vec2[0], "..");
            let s_vec4 = doe::vec_element_parse!(s_vec3, f64);
            let mut left = false;
            let mut right = false;
            if s_vec22.len() == 3 {
                if s_vec22[0] == '.' {
                    left = true;
                } else if s_vec22[2] == '.' {
                    right = true;
                }
            }
            if left == false && right == false {
                if s_vec4.len() > 1 {
                    start = s_vec4[0] as usize;
                    stop = s_vec4[1] as usize;
                } else {
                    stop = vec.len();
                }
            } else if left == true {
                stop = s_vec4[0] as usize;
            } else if right == true {
                start = s_vec4[0] as usize;
            }
            slice_func(vec, start, stop)
        }};
    }

    /// vec_element_clone!() used to when occure
    /// cannot move out of index of `Vec<String>`
    /// move occurs because value has type `String`, which does not implement the `Copy` trait
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_element_clone!(vec!["15.", "2.9"], 1);
    ///let v2 = vec_element_clone!(vec![15, 2, 3, 2], 2);
    ///let v3 = vec_element_clone!(vec![0.15, 0.2, 0.2], 0);
    ///let v4 = vec_element_clone!(vec![format!("1"),format!("2"),format!("3"),format!("4"),format!("5")],4);
    ///assert_eq!("2.9", v1);
    ///assert_eq!(3, v2);
    ///assert_eq!(0.15, v3);
    ///assert_eq!(format!("5"), v4);
    /// ```
    #[macro_export]
    macro_rules! vec_element_clone {
        ($vec:expr,$index:expr) => {{
            let mut vec = $vec.clone();
            let s = &vec[$index];
            s.to_owned()
        };};
    }
    /// remove_file_or_folder!() used to remove file or folder
    /// ```ignore
    ///remove_file_or_folder!("./demo.txt");
    ///remove_file_or_folder!("./target");
    /// ```
    #[macro_export]
    macro_rules! remove_file_or_folder {
        ($path:expr) => {{
            use std::fs;
            use std::path::Path;
            pub fn rm(path: &Path) {
                if path.is_dir() {
                    fs::remove_dir_all(path).unwrap();
                } else if path.is_file() {
                    fs::remove_file(path).unwrap();
                }
            }
            let path = Path::new($path);
            rm(path);
        };};
    }
    /// vec_type!() used to get vec type
    /// return format!("{}",type);
    /// ```rust
    /// use doe::*;
    ///assert_eq!(vec_type!(vec![0.2_f64]), format!("Vec<f64>"));
    ///assert_eq!(vec_type!(vec![0.2_f32]), format!("Vec<f32>"));
    ///assert_eq!(vec_type!(vec![2_i32]), format!("Vec<i32>"));
    ///assert_eq!(vec_type!(vec![2_i128]), format!("Vec<i128>"));
    ///assert_eq!(vec_type!(vec![2_isize]), format!("Vec<isize>"));
    ///assert_eq!(vec_type!(vec![2_usize]), format!("Vec<usize>"));
    /// ```
    #[macro_export]
    macro_rules! vec_type {
        ($vec:expr) => {{
            use std::any::Any;
            pub fn vec_type(s: &dyn Any) -> String {
                let mut re = String::new();
                if s.is::<Vec<String>>() {
                    re = format!("Vec<String>");
                } else if s.is::<Vec<&str>>() {
                    re = format!("Vec<&str>");
                } else if s.is::<Vec<i128>>() {
                    re = format!("Vec<i128>");
                } else if s.is::<Vec<i64>>() {
                    re = format!("Vec<i64>");
                } else if s.is::<Vec<i32>>() {
                    re = format!("Vec<i32>");
                } else if s.is::<Vec<i16>>() {
                    re = format!("Vec<i16>");
                } else if s.is::<Vec<i8>>() {
                    re = format!("Vec<i8>");
                } else if s.is::<Vec<u128>>() {
                    re = format!("Vec<u128>");
                } else if s.is::<Vec<u64>>() {
                    re = format!("Vec<u64>");
                } else if s.is::<Vec<u32>>() {
                    re = format!("Vec<u32>");
                } else if s.is::<Vec<u16>>() {
                    re = format!("Vec<u16>");
                } else if s.is::<Vec<u8>>() {
                    re = format!("Vec<u8>");
                } else if s.is::<Vec<f64>>() {
                    re = format!("Vec<f64>");
                } else if s.is::<Vec<f32>>() {
                    re = format!("Vec<f32>");
                } else if s.is::<Vec<usize>>() {
                    re = format!("Vec<usize>");
                } else if s.is::<Vec<isize>>() {
                    re = format!("Vec<isize>");
                } else {
                    re = format!("");
                }
                re
            }
            vec_type(&$vec)
        };};
    }

    /// vec_element_convert!() used convert vec elements type
    /// ```rust
    /// use doe::*;
    ///let v1: Vec<f64> = vec_element_convert!(vec![1, 2], f64);
    ///let v2: Vec<i32> = vec_element_convert!(vec![1.0, 2.0], i32);
    ///let v3: Vec<i128> = vec_element_convert!(vec![1.0, 2.0], i128);
    ///let v4: Vec<i32> = vec_element_convert!(vec![1_usize, 2_usize], i32);
    ///let v5: Vec<i64> = vec_element_convert!(vec![0.15, 2.0], i64);
    ///assert_eq!(v1, vec![1.0, 2.0]);
    ///assert_eq!(v2, vec![1, 2]);
    ///assert_eq!(v3, vec![1, 2]);
    ///assert_eq!(v4, vec![1, 2]);
    ///assert_eq!(v5, vec![0, 2]);
    /// ```
    #[macro_export]
    macro_rules! vec_element_convert {
        ($vec:expr,$type:ty) => {{
            let vec = $vec.clone();
            let mut re: Vec<$type> = Vec::new();
            let re_type = vec_type!(re);
            if vec_type!(re) != format!("Vec<String>") {
                for item in &vec {
                    re.push(*item as $type);
                }
            }
            re
        };};
    }

    /// expr return max value
    /// ```rust
    /// use doe::*;
    ///let re_max = max!(1, 2);
    ///assert_eq!(re_max,2);
    /// ```
    #[macro_export]
    macro_rules! max {
        ($x:expr) => ($x);
        ($x:expr, $($y:expr),+) => {
            std::cmp::max($x, max!($($y),+))
        }
    }
    /// expr return min value
    /// ```rust
    /// use doe::*;
    ///let re_min = min!(1, 2, 2, 5, 4, 6);
    ///assert_eq!(re_min,1);
    /// ```
    #[macro_export]
    macro_rules! min {
        ($x:expr) => ($x);
        ($x:expr, $($y:expr),+) => {
            std::cmp::min($x, max!($($y),+))
        }
    }
    /// convert binary string to decimal
    /// ```rust
    /// use doe::*;
    ///let d1 = binary_to_decimal!("01101",i128);
    ///assert_eq!(d1,13_i128);
    ///let d2 = binary_to_decimal!("00000000000010100110001",i64);
    ///assert_eq!(d2,1329_i64);
    ///let d3 = binary_to_decimal!("000011",i32);
    ///assert_eq!(d3,3_i32);
    ///let d4 = binary_to_decimal!("000101",i16);
    ///assert_eq!(d4,5_i16);
    ///let d5 = binary_to_decimal!("001001",i8);
    ///assert_eq!(d5,9_i8);
    /// ```
    #[macro_export]
    macro_rules! binary_to_decimal {
        ($binary_string:expr,$decimal_type:ty) => {{
            fn binary_to_decimal(num: impl ToString) -> $decimal_type {
                let mut sum =  0 as $decimal_type;
                let vec = num.to_string()
                    .chars()
                    .map(|x| <$decimal_type>::from(x.to_digit(10).unwrap() as $decimal_type))
                    .collect::<Vec<$decimal_type>>();
                for (index, item) in vec.iter().rev().enumerate() {
                    sum += <$decimal_type>::pow(2, index as u32) * item;
                }
                sum
            }
            binary_to_decimal($binary_string)
        }}
    }
    /// expr return memory address
    /// ```rust
    /// use doe::*;
    ///let d1 = binary_to_decimal!("01101",i128);
    ///println!("{:?}",memory_address!(d1));//0x7ffcac734f08
    /// ```
    #[macro_export]
    macro_rules! memory_address {
        ($x:expr) => {{
            &$x as *const _
        }};
    }

    /// merge two vec return merged vec
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_merge!(vec![0, 1, 2], vec![5, 6, 7]);
    ///assert_eq!(vec![0, 1, 2, 5, 6, 7],v1);
    ///let v2 = vec_merge!(vec![0., 1., 2.], vec![5., 6., 7.]);
    ///assert_eq!(vec![0., 1., 2., 5., 6., 7.],v2);
    ///```
    #[macro_export]
    macro_rules! vec_merge {
        ($vec1:expr,$vec2:expr) => {{
            let mut re = vec![];
            for item in $vec1.iter() {
                re.push(item.to_owned());
            }
            for item in $vec2.iter() {
                re.push(item.to_owned());
            }
            re
        }};
    }

     /// take size of elements and return a new vec
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_element_take!(vec![0, 1, 2],2);
    ///assert_eq!(vec![0,1],v1);
    ///```
    #[macro_export]
    macro_rules! vec_element_take {
        ($vec:expr,$size:expr) => {{
            let mut re = vec![];
            for i in 0..$size {
                re.push(vec_element_clone!($vec, i));
            }
            re
        }};
    }

     /// zip two vec elements in tuple
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_zip!(vec![0, 1, 2],vec![0, 1, 2]);
    ///assert_eq!(vec![(0,0),(1,1),(2,2)],v1);
    ///```
    #[macro_export]
    macro_rules! vec_zip {
        ($vec1:expr,$vec2:expr) => {{
            let mut re = vec![];
            if $vec1.len() == $vec2.len() {
                for i in 0..$vec1.len() {
                    re.push((vec_element_clone!($vec1, i),vec_element_clone!($vec2, i)));
                }
            }
            re
        }};
    }

     /// enumerate all indexs and elements collect tuple of vec
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_enumerate!(vec![12, 11, 20]);
    ///assert_eq!(vec![(0,12),(1,11),(2,20)],v1);
    ///```
    #[macro_export]
    macro_rules! vec_enumerate {
        ($vec:expr) => {{
            let mut re = vec![];
            for (index, element) in $vec.iter().enumerate() {
                re.push((index, vec_element_clone!($vec, index)));
            }
            re
        }};
    }
    /// sort vec and return sorted vec
    /// ```rust
    /// use doe::*;
    ///let v1 = vec_sort!(vec![10, 2, 3]);
    ///assert_eq!(vec![2,3,10],v1);
    ///let v2 = vec_sort!(vec![1.8, 2.5, 0.3]);
    ///assert_eq!(vec![0.3,1.8,2.5],v2);
    ///```
    #[macro_export]
    macro_rules! vec_sort {
        ($vec:expr) => {{
            let mut re = $vec.clone();
            re.sort_by(|a, b| a.partial_cmp(b).unwrap());
            re
        }};
    }
    /// has stable rust nightly return bool
    /// ```rust
    /// use doe::*;
    /// let v1 = has_nightly_compiler!();
    /// assert_eq!(v1, true);
    ///```
    #[macro_export]
    macro_rules! has_nightly_compiler {
        () => {{
            fn has_nightly_compiler() -> bool {
                use std::process;
                match process::Command::new("cargo")
                    .arg("+nightly")
                    .arg("help")
                    .stdout(process::Stdio::null())
                    .stderr(process::Stdio::null())
                    .status()
                {
                    Ok(exit_status) => exit_status.success(),
                    Err(_) => false,
                }
            }
            has_nightly_compiler()
        }};
    }

    /// has stable rust compiler return bool
    /// ```rust
    /// use doe::*;
    /// let v1 = has_stable_compiler!();
    /// assert_eq!(v1, false);
    ///```
    #[macro_export]
    macro_rules! has_stable_compiler {
        () => {{
            fn has_stable_compiler() -> bool {
                use std::process;
                match process::Command::new("cargo")
                    .arg("+stable")
                    .arg("help")
                    .stdout(process::Stdio::null())
                    .stderr(process::Stdio::null())
                    .status()
                {
                    Ok(exit_status) => exit_status.success(),
                    Err(_) => false,
                }
            }
            has_stable_compiler()
        }};
    }

    ///## run command
    /// ```ignore
    /// use doe::*;
    ///command!("ls -la");            
    ///command!("dust");            
    ///command!("lsd");            
    ///```
    #[macro_export]
    macro_rules! command {
        ($cmd:expr) => {{
                let cmd = $cmd;
                use std::{io::Write, process::Command};
                let exe = vec_element_clone!(split_to_vec!(cmd," "),0);
                let other = split_to_vec!(cmd," ").get(1..).unwrap().
                into_iter().map(|s|s.to_string()).collect::<Vec<_>>();
                let output = Command::new(&exe).args(&other).stdout(std::process::Stdio::inherit())
                .output();
                match output {
                    Ok(o)=>{
                        let out = o.stdout;
                        let err = o.stderr;
                        let stat = o.status;
                        if stat.success(){
                            // std::io::stdout().write_all(&out).unwrap();
                            unsafe{
                                println!("{}",String::from_utf8_unchecked(out));
                            }
                        }else{
                            // std::io::stdout().write_all(&err).unwrap();
                            unsafe{
                                println!("{}",String::from_utf8_unchecked(err));
                            }
                        }
                    }
                    Err(e)=>{
                        println!("{}",e);
                    }
                }
        }};
    }

    
    pub fn run() {
        // command!("git add .");            
        // command!("git commit -m 'ok'");   
        // command!("git push");         
        command!("dust");
        command!("ls -la");
    }
}
