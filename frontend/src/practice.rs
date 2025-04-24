




#[async_std::main]
async fn main() -> Result<(),anyhow::Error> {


    let rows = vec![
        ("Sohail","Uddin",25,"a"),
        ("Shafi","Uddin",27,"b"),
        ("Akhtar","Parveen",26,"c"),
        ("Mama","Adil",31,"d")
    ];


    for (i,first_name,last_name,age,grade) in rows.iter().enumerate() {
        println!("{} {} {} {} {}");
    }
}