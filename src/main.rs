//use std::env;
use std::collections::HashMap;
use catch_input::input;
mod db;
fn cry() -> f32{
	println!("NOOOO");
	0.0
}

fn query(string:&str){

if &string[0..1]=="q"{
let q = &string[7..];
println!("Result: {}",db::query_key(q));
}

	
}

fn main(){
let a = input!((String::from("Whadya wanna do? (no spaces between commas)\n1. Populate database (enter 1)\n2. query database (Syntax = query num,num,num,num)\n3. solve (Syntax = solve num,num,num,num)")));
	match a.as_str(){
		"1" => {populate_database();},
		_=>{query(a.as_str());}


		
	}
}

fn populate_database(){
	let mut items = HashMap::new();
// let a = input!("one => ").parse::<f32>().unwrap();
// let b = input!("two => ").parse::<f32>().unwrap();
// let c = input!("three => ").parse::<f32>().unwrap();
// let d = input!("four => ").parse::<f32>().unwrap();
// solve(a,b,c,d);

for i in 1..=13{
for p in 1..=13{
for t in 1..=13{
for v in 1..=13{
let mut nvec = [i,p,t,v];
quicksort(&mut nvec);
items.entry(nvec).or_insert(solve(nvec[0] as f32, nvec[1] as f32, nvec[2] as f32, nvec[3] as f32));
}
}
}
}

	for keys in items.keys(){
		let key = keys[0].to_string()+","+&keys[1].to_string()+","+&keys[2].to_string()+","+&keys[3].to_string()+";";
		db::save_to_db(&key, items.get(keys).unwrap_or(&0));
		
	};
}


fn solve(one:f32, two:f32, three:f32, four:f32) -> i32{
		//let args: Vec<String> = env::args().collect();
		let mut nums: Vec<f32> = Vec::<f32>::new();	
		nums.push(one);
		nums.push(two);
		nums.push(three);
		nums.push(four);
// 		for s in args{
// 		nums.push(
			
// 			match s.parse::<f32>(){
// Ok(n) => n,
// Err(_e) => cry(),
				
// 			}
// );
// 		}
    let mut val:f32;
    //println!("Enter 4 numbers: \n");
    //scanf("%d%d%d%d", &numbers[0], &numbers[1], &numbers[2], &numbers[3]);
    // printf("%lf", calc2(add, add, div, numbers));
    //sortArray(4, numbers);
	let mut count:i32 = 0;
    loop{
        // printArray(4,numbers);
			//let i = 0;

        for i in 0..4 {
						//let j=0;
            for j in 0..4 {
								//let k = 0;
                for k in 0..4 {
										//let l = 0;
                    for l in 0..4{
                        val = calc2(i, j, k, &nums, l);
                        if 24.0 - 0.0001 < val && val < 24.0 + 0.0001 {
                            //println!("{},{},{},{:#?},{},{}",i, j, k, nums, l, val);
													//PrintStuff(i,j,k,&nums, l);
													count+=1;
                            //return 1;
                        }
                    }
								}
						}
			
				}
    if !findNext(4, &mut nums){
			//println!("48");
			break;}
		}
    //println!("No solution\n");
    //return 0
	count
}

fn calc(op:i32, num:f32, mut num2:f32) -> f32{
    // i = operation,num and num2 are the numbers to operate
    let returnVal:f32 = match op {
        0=>  // add
            num + num2,
            
        1=>  // subtract
            num - num2,
            
        2=>  // multiply
            num * num2,
            
        3=>  {// divide
            if num2 == 0.0 {
                num2 = 0.0000001;
            }
						num / num2
				}
            // printf("%g %g\n", num, num2);
        _=>0.0,
    };
    returnVal
}

fn calc2(op:i32, op2:i32, op3:i32, arr:&Vec<f32>, var:i32) -> f32{
    let mut returnVal:f32;
    let returnVal2:f32;
    match var {
        0=>  // 123
					{returnVal = calc(op, arr[0], arr[1]);
            returnVal = calc(op2, returnVal, arr[2]);
            returnVal = calc(op3, returnVal, arr[3])}
						
          // 132
        4|1=>{  // 312
            returnVal = calc(op, arr[0], arr[1]);
            returnVal2 = calc(op3, arr[2], arr[3]);
            returnVal = calc(op2, returnVal, returnVal2)}
            
        2=>{  // 213
            returnVal2 = calc(op2, arr[1], arr[2]);
            returnVal = calc(op, arr[0], returnVal2);
            returnVal = calc(op3, returnVal, arr[3])}
            
        3=>{  // 231
            returnVal2 = calc(op2, arr[1], arr[2]);
            returnVal = calc(op3, returnVal2, arr[3]);
            returnVal = calc(op, arr[0], returnVal);
            }
        5=>{  // 321
            returnVal2 = calc(op3, arr[2], arr[3]);
            returnVal = calc(op2, arr[1], returnVal2);
            returnVal = calc(op, arr[0], returnVal);
				}
			_=>{
				returnVal = -1.0;
			}
    }
    returnVal
}








fn findNext(n:i32, x:&mut Vec<f32>) -> bool{
    let mut i:i32 = n;
    for w in 1..n {
				i = i-1;
        if x[i as usize - 1] < x[i as usize] {  // x[i-1] is first drop, to be increased
            for j in i..n{
                // find x[j] that is the smallest one great than x[i-1]
                if j == n - 1 || x[j as usize + 1] <= x[i as usize - 1] {
                    // swap x[i-1] and x[j] (x[i-1] is incresed to x[j])
                    let temp:f32 = x[j as usize];
                    x[j as usize] = x[i as usize - 1];
                    x[i as usize - 1] = temp;
                    // reverse elements after x[i-1] from decending to ascending
										let p = i;
										let mut q = n-1;
                    for mut p in p..q{
                        let temp = x[p as usize];
                        x[p as usize] = x[q as usize];
                        x[q as usize] = temp;
												p+=1;
												q-=1;
                    }
                    return true;  // found it
                }
            }
        }
    }
    // i==0, no drop, x in descending order, last permutation
    false  // no more
}

// Print out the array in one line
fn printArray(n:i32, a:Vec<i32>) {
    for i in 0..n {
        println!("{}", a[i as usize]);
    }
    println!("\n");
}

// Similar to strcmp
// Compare two integers, to be used in qsort to sort integers
fn compare_ints(a:i32, b:i32) ->i32 {
    return a - b;
}

// Sort the array in ascending order
fn sortArray(mut a:Vec<i32>) { quicksort(&mut a); }


fn partition(a: &mut [i32]) -> usize{
    let mut i = 0;
    let right = a.len() - 1;
 
    for j in 0..right {
        if a[j] <= a[right] {
            a.swap(j, i);
            i += 1;
        }
    }
 
    a.swap(i, right);
    i
}
 
fn quicksort(a: &mut [i32]) {
    if a.len() > 1 {
        let q = partition(a);
        quicksort(&mut a[..q]);
        quicksort(&mut a[q+1..]);
    }
}

fn PrintStuff(op:i32, op2:i32, op3:i32, arr:&Vec<f32>, var:i32) {
    let operators = ['+', '-', '*', '/'];
		let op:usize = op as usize;
		let op2 = op2 as usize;
		let op3 = op3 as usize;
    match var {
        0=>  // 123{}
					{print!("(({} {} {}) {} {}) {} {}", arr[0], operators[op], arr[1],
                   operators[op2], arr[2], operators[op3], arr[3])}
            
          // 132
        4|1=>{  // 312
            print!("({} {} {}) {} ({} {} {})", arr[0], operators[op], arr[1],
                   operators[op2], arr[2], operators[op3], arr[3])}
            
        2=> { // 213
            print!("({} {} ({} {} {})) {} {}", arr[0], operators[op], arr[1],
                   operators[op2], arr[2], operators[op3], arr[3])}
            
        3=>{  // 231
            print!("{} {} (({} {} {}) {} {})", arr[0], operators[op], arr[1],
                   operators[op2], arr[2], operators[op3], arr[3])}
            
        5=>{  // 321
            print!("{} {} ({} {} ({} {} {}))", arr[0], operators[op], arr[1],
                   operators[op2], arr[2], operators[op3], arr[3])}
        _=>{panic!("augh")}
    }
    println!(" = 24\n");
}