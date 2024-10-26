fn main() {


    let mut dizi: Vec<String>=vec!["yazi".to_string(),"yazi".to_string()];
    
    let mut database: Vec<Account> = Vec::new();


    register("Zübeyde".to_string(), 3424242432, 32432325,Entries::personel_user ,&mut database);
    register("Aycan".to_string(), 2494294292, 322355,Entries::personel_user, &mut database);


    deposit(3424242432, 1000.00, &mut database);
    withdraw(3424242432, 500.0, &mut database);
    show_amount(3424242432, &mut database);

    transfer(3424242432, 300.00, 322355, &mut database);

    show_amount(2494294292, &mut database);




   
}


#[derive(Debug,Clone)]

struct Account {

    name:String,
    hesap_no :u32,
    amount : f32,
    account_id:u32,
    entry:Entries
}


#[derive(Debug,Clone,PartialEq)]

enum Entries {

    norml_user,
    personel_user
    
}



fn register(name:String,hesap_no:u32,account_id:u32,entry:Entries,database : &mut Vec<Account>) {

    let account = Account{
        name :name,
        hesap_no:hesap_no,
        amount:0.0,
        account_id:account_id,
        entry:entry

    };
    database.push(account.clone());
    println!("Kaydınız başarılı ile oldu hoş geldiniz {:?}",account.name)
    
}


fn show_amount(hesap_no:u32,database : &mut Vec<Account>) {
    for data in database  {
        if hesap_no == data.hesap_no {
            println!("Bakiyeniz {}",data.amount)
            
        }
        else {
            println!("Bankamıza kayıtlı değilsiniz ")
        }
        
    }

    
}

fn personele_özel(entry:Entries) {
    if entry == Entries::personel_user {
        
    }
    
}

fn deposit(hesap_no:u32,amount:f32,database : &mut Vec<Account>) {
    for data in database  {
        if hesap_no == data.hesap_no {
            data.amount = data.amount + amount;
            println!("{} kadar para yüklendi yeni bakiyeniz : {}",amount,data.amount)
            
        }
        else {
            println!("Bankamıza kayıtlı değilsiniz ")
        }
        let count = 0;
      }
    
}

fn withdraw(hesap_no:u32,amount:f32,database : &mut Vec<Account>) {

    for data in database  {
        if hesap_no == data.hesap_no {
            data.amount = data.amount - amount;
            println!("{} kadar para çektiniz yeni bakiyeniz : {}",amount,data.amount)
            
        }
        else {
            println!("Bankamıza kayıtlı değilsiniz ")
        }
        
    }
    
}

fn transfer(hesap_no:u32,amount:f32,account_id:u32,database : &mut Vec<Account>) {

    for data in database  {
        if hesap_no==data.hesap_no {

            data.amount = data.amount - amount;
            println!("Hesabınızdan bu kadar para yolandı {}",amount)
            
        }

        if account_id==data.account_id {

            data.amount = data.amount + amount;
            println!("Hesabınıza bu kadar para geldi {}",amount)
            
        }
        
    }

    
    



    
}




