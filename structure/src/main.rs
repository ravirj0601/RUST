struct Emp{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let ravi = Emp{
        active: true,
        username: String::from("Bhola"),
        email: String::from("Bhola.ji@gmail.com"),
        sign_in_count: 1,
    }; 
    println!("EmailId: {}\nStatus: {}\nUserName: {}\nSignInNo: {}",ravi.email,
     ravi.username, ravi.active, ravi.sign_in_count);
    let sonal = Emp{
        active: true,
        username: String::from("Dost"),
        email: String::from("Dosti@gmail.com"),
        sign_in_count: ravi.sign_in_count,
    };
    println!("EmailId: {}\nStatus: {}\nUserName: {}\nSignInNo: {}",sonal.email,
     sonal.username, sonal.active, sonal.sign_in_count);
    
}
