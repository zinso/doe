# Macros for efficient development
[![](https://gitlab.com/andrew_ryan/doe/-/raw/master/doe.svg)](https://docs.rs/doe)

## Examples

```rust
use doe::*;
//get argument and into Vec<String>
//cargo run -- -n 100
let arg = args!();
assert_eq!(arg,vec![format!("-n"),format!("100")]);

//get input and into String
let s = input!();
println!("{:?}",s);

//power of two f64 number
let p = powf!(2.,2.);
assert_eq!(p,4.0);

//split a:&str by b:&str and collect into Vec<String>  
let s = split_to_vec!("aa.bb",".");
assert_eq!(s,vec![format!("aa"),format!("bb")]);

// read .csv file and return Vec<Vec<String>>
let s = read_csv!("./data.csv");
assert_eq!(s,vec![vec![format!("a"), format!("b"), format!("c")],vec![format!("1"), format!("2"), format!("3")],vec![format!("10"), format!("20"), format!("30")]]);

//remove_file_or_folder!() used to remove file or folder
//type &str path
remove_file_or_folder!("./demo.txt");
remove_file_or_folder!("./target");

//return sorted vec
// type can be Vec<i8> Vec<i16> Vec<i32> Vec<i64> Vec<i128> Vec<f32> Vec<f64>
let s1 = sorted!(vec![1.2, 2.6, 0.2]);
let s2 = sorted!(vec![8, 1_i128, 5_i128]);
assert_eq!(s1,vec![0.2,1.2,2.6]);
assert_eq!(s2,vec![1,5,8]);

// return sorted and deduped new Vec
// type can be Vec<i8> Vec<i16> Vec<i32> Vec<i64> Vec<i128> Vec<f32> Vec<f64>
let s1 = deduped_sorted!(vec![1.2, 1.2,2.6, 0.2]);
let s2 = deduped_sorted!(vec![8, 1_i128, ,8,5_i128]);
assert_eq!(s1,vec![0.2,1.2,2.6]);
assert_eq!(s2,vec![1,5,8]);

//parse Vec element to f64,
//need a Vec<&str>,and parse type
//return Vec<type>
let v1: Vec<f64> = vec_element_parse!(vec!["15.", "2.9"], f64);
let v2: Vec<i128> = vec_element_parse!(vec!["15", "2"], i128);
let v3: Vec<f32> = vec_element_parse!(vec![".15", ".2"], f32);
assert_eq!(vec![15.0, 2.9], v1);
assert_eq!(vec![15, 2], v2);
assert_eq!(vec![0.15, 0.2], v3);

 //vec_element_to_string!() used to
// convert vec element to String,
// type can be Vec<i32> Vec<i64> Vec<i128> Vec<f32> Vec<f64>,Vec<&str>
// return Vec<String>
let v1 = vec_element_to_string!(vec!["15.", "2.9"]);
let v2 = vec_element_to_string!(vec![15, 2]);
let v3 = vec_element_to_string!(vec![0.15, 0.2]);
assert_eq!(vec!["15.", "2.9"], v1);
assert_eq!(vec!["15", "2"], v2);
assert_eq!(vec!["0.15", "0.2"], v3);
```

```rust
//snail_sort!() return the array elements arranged from outermost 
//elements to the middle element, traveling clockwise.n x n
//type can be Vec<Vec<T>>
//return Vec<T>
```

<div align="center">
<img src="https://www.haan.lu/files/2513/8347/2456/snail.png"/>
</div>

```rust
let v1 = snail_sort!(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
let v2 =  snail_sort!(vec![vec![1.1, 2.1, 3.1],vec![4.1, 5.1, 6.1],vec![7.1, 8.1, 9.1]]);
assert_eq!(vec![1, 2, 3, 6, 9, 8, 7, 4, 5], v1);
assert_eq!(vec![1.1, 2.1, 3.1, 6.1, 9.1, 8.1, 7.1, 4.1, 5.1], v2);

// multiply_matrix!() return the mutiply result of two matrix
// take two matrix and type can be Vec<Vec<f64>>
// return Vec<Vec<f64>>
let m1: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![1.0, 2.0]];
let m2: Vec<Vec<f64>> = vec![vec![0.0, 0.0], vec![0.0, 0.5]];
let mul_result = multiply_matrix!(&m2, &m1);
assert_eq!(mul_result, vec![[0.0, 0.0], [0.5, 1.0]]);

// vec_element_remove!() used to
// find the element fist position and remove
// if not fond the element return original vec
// type can be Vec<i32> Vec<i64> Vec<i128> Vec<f32> Vec<f64>,Vec<&str>
// return return the same type
let v1 = vec_element_remove!(vec!["15.", "2.9"], "2.9");
let v2 = vec_element_remove!(vec![15, 2, 3, 2], 2);
let v3 = vec_element_remove!(vec![0.15, 0.2, 0.2], 0.2);
assert_eq!(vec!["15."], v1);
assert_eq!(vec![15, 3, 2], v2);
assert_eq!(vec![0.15, 0.2], v3);

// vec_element_remove_all!() used to
// find the element all position and remove
// if not fond the element return original vec
// type can be Vec<i32> Vec<i64> Vec<i128> Vec<f32> Vec<f64>,Vec<&str>
// return return the same type
let v1 = vec_element_remove_all!(vec!["15.", "2.9", "0.9", "2.9", "2.9"], "2.9");
assert_eq!(vec!["15.", "0.9"], v1);
let v2 = vec_element_remove_all!(vec![15, 2, 3, 2], 2);
assert_eq!(vec![15, 3], v2);
let v3 = vec_element_remove_all!(vec![0.15, 0.2, 1.0, 0.2], 0.3);
assert_eq!(vec![0.15, 0.2, 1.0, 0.2], v3);

//vec_element_position_all!() used to find element position and collect into Vec<usize>
let v1 = vec_element_position_all!(vec![1, 2, 5, 3, 6, 2, 2], 2);
assert_eq!(v1, vec![1, 5, 6]);

// vec_slice!() used to
// slice vec by range
// type can be  Vec<i32> Vec<i64> Vec<i128> Vec<f32> Vec<f64>,Vec<&str>
// and need a range [a..b]=> a to b ,[..]=>all
// return return the same type
let v1 = vec_slice!(vec![1.2, 1.5, 9.0], [..2]);
let v2 = vec_slice!(vec![1, 1, 9, 90, 87, 0, 2], [4..6]);
let v3 = vec_slice!(vec![1.2, 1.5, 9.0], [..]);
let v4 = vec_slice!(vec![1.2, 1.5, 9.0], [1..]);
let v5 = vec_slice!(vec!["1", "2", "3", "4", "5"], [2..5]);
let v6 = vec_slice!(vec!["1".to_string(),"2".to_string(),"3".to_string()], [1..]);
assert_eq!(v1, vec![1.2, 1.5]);
assert_eq!(v2, vec![87, 0]);
assert_eq!(v3, vec![1.2, 1.5, 9.0]);
assert_eq!(v4, vec![1.5, 9.0]);
assert_eq!(v5, vec!["3", "4", "5"]);
assert_eq!(v6,vec!["2".to_string(),"3".to_string()]);

//vec_element_clone!() used to when occure
//cannot move out of index of `Vec<String>`
//move occurs because value has type `String`, which does not implement the `Copy` trait
let v1 = vec_element_clone!(vec!["15.", "2.9"], 1);
let v2 = vec_element_clone!(vec![15, 2, 3, 2], 2);
let v3 = vec_element_clone!(vec![0.15, 0.2, 0.2], 0);
let v4 = vec_element_clone!(vec![format!("1"),format!("2"),format!("3"),format!("4"),format!("5")],4);
assert_eq!("2.9", v1);
assert_eq!(3, v2);
assert_eq!(0.15, v3);
assert_eq!(format!("5"), v4);

//vec_element_convert!() used convert vec elements type
let v1: Vec<f64> = vec_element_convert!(vec![1, 2], f64);
let v2: Vec<i32> = vec_element_convert!(vec![1.0, 2.0], i32);
let v3: Vec<i128> = vec_element_convert!(vec![1.0, 2.0], i128);
let v4: Vec<i32> = vec_element_convert!(vec![1_usize, 2_usize], i32);
let v5: Vec<i64> = vec_element_convert!(vec![0.15, 2.0], i64);
assert_eq!(v1, vec![1.0, 2.0]);
assert_eq!(v2, vec![1, 2]);
assert_eq!(v3, vec![1, 2]);
assert_eq!(v4, vec![1, 2]);
assert_eq!(v5, vec![0, 2]);

// vec_type!() used to get vec type
// return format!("{}",type);
assert_eq!(vec_type!(vec![0.2_f64]), format!("Vec<f64>"));
assert_eq!(vec_type!(vec![0.2_f32]), format!("Vec<f32>"));
assert_eq!(vec_type!(vec![2_i32]), format!("Vec<i32>"));
assert_eq!(vec_type!(vec![2_i128]), format!("Vec<i128>"));
assert_eq!(vec_type!(vec![2_isize]), format!("Vec<isize>"));
assert_eq!(vec_type!(vec![2_usize]), format!("Vec<usize>"));

// expr return max value
let re_max = max!(1, 2);
assert_eq!(re_max,2);

// expr return min value
let re_min = min!(1, 2, 2, 5, 4, 6);
assert_eq!(re_max,1);

//convert binary string to decimal
let d1 = binary_to_decimal!("01101",i128);
assert_eq!(d1,13_i128);
let d2 = binary_to_decimal!("00000000000010100110001",i64);
assert_eq!(d2,1329_i64);
let d3 = binary_to_decimal!("000011",i32);
assert_eq!(d3,3_i32);
let d4 = binary_to_decimal!("000101",i16);
assert_eq!(d4,5_i16);
let d5 = binary_to_decimal!("001001",i8);
assert_eq!(d5,9_i8);

// get memory_address
let d1 = binary_to_decimal!("01101",i128);
println!("{:?}",memory_address!(d1));//0x7ffcac734f08

// merge two vec return merged vec
let v1 = vec_merge!(vec![0, 1, 2], vec![5, 6, 7]);
assert_eq!(vec![0, 1, 2, 5, 6, 7],v1);
let v2 = vec_merge!(vec![0., 1., 2.], vec![5., 6., 7.]);
assert_eq!(vec![0., 1., 2., 5., 6., 7.],v2);

// take size of elements and return a new vec
let v1 = vec_element_take!(vec![0, 1, 2],2);
assert_eq!(vec![0,1],v1);

// zip two vec elements in tuple
let v1 = vec_zip!(vec![0, 1, 2],vec![0, 1, 2]);
assert_eq!(vec![(0,0),(1,1),(2,2)],v1);

// enumerate all indexs and elements collect tuple of vec
let v1 = vec_enumerate!(vec![12, 11, 20]);
assert_eq!(vec![(0,12),(1,11),(2,20)],v1);

// sort vec and return sorted vec
let v1 = vec_sort!(vec![10, 2, 3]);
assert_eq!(vec![2,3,10],v1);
let v2 = vec_sort!(vec![1.8, 2.5, 0.3]);
assert_eq!(vec![0.3,1.8,2.5],v2);

// has stable rust compiler return bool
let v1 = has_stable_compiler!();
assert_eq!(v1, false);

// has nightly rust compiler return bool
let v1 = has_nightly_compiler!();
assert_eq!(v1, true);

// run command
command!("ls -la");            
command!("dust");            
command!("lsd");  
```


