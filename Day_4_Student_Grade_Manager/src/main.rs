use std::io;
fn main() {
    'main :loop {
       println!("==== Student Grade Manager ====");
    let student_name = get_name();
   let student_score = get_score();
   let student_grade = cal_grade(student_score);
   let student_remark = remark(student_score);
   display_result(student_name, student_score, student_grade,student_remark); 
   
   println!("Do you want to enter another student? (yes/no)");
   loop {
      let mut play_again_input =String::new();
   io::stdin().read_line(&mut play_again_input).unwrap();
   match play_again_input.trim().to_lowercase().as_str(){
         "yes"=> continue 'main,
         "no" => break 'main,
         _ => {
            println!("Please enter 'yes' or 'no'.");
            continue;
        }
     } 
   }
   

    }
   

    

   
}

fn get_name()-> String {
    println!("enter your name");
     let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).unwrap();
    let name = name_input.trim();
    return name.to_string();
}

fn get_score () -> i32 {
    loop {
     println!("enter your score");
     let mut score_input = String::new();
     io::stdin().read_line(&mut score_input).unwrap();
     
     let score: i32 = match score_input.trim().parse() {
        Ok(num) => num,
        Err(_) => { 
            println!("enter a number");
            continue ;
        }
        
        
    };
    if score > 100 || score < 0{
        println!("your score input is invalide, score only range from 0 - 100 ")
    }else {
       return score;  
    }
      
    }
    
}

fn cal_grade(score:i32) -> & 'static str {
    let exelence ="A";
    let very_good ="B" ;
    let good = "C" ;
    let fair ="D" ;
    let failed ="F";

  if score >= 90 {
     return exelence;
}else if score >= 80 && score <= 89 {
    return very_good;
    
}else if score >= 70 && score <= 79 {
    return good;
}else if score >= 50 && score <= 69 {
    return fair;
} else {
    return failed;
}
}

fn remark (score:i32) -> & 'static str {
    let exelence ="Excellent";
    let very_good ="Very Good" ;
    let good = "Good" ;
    let fair ="Fair" ;
    let failed ="Failed";

  if score >= 90 {
     return exelence;
}else if score >= 80 && score <= 89 {
    return very_good;
    
}else if score >= 70 && score <= 79 {
    return good;
}else if score >= 50 && score <= 69 {
    return fair;
} else {
    return failed;
}
}


fn display_result(student:String,score:i32,grade:&str,remark:&str) {

    println!("=============================");
    println!("student report");
    println!("=============================");
println!("student :{}",student);
println!("score :{}",score);
println!("grade :{}",grade);
println!("Remark :{}",remark);
}