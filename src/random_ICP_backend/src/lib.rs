
const STRINGS: [&str; 10] = [
    "String1",
     "String2",
      "String3",
       "String4",
        "String5",
         "String6", 
         "String7",
          "String8",
           "String9",
            "String10"
            ];

#[ic_cdk::query]
fn greet(name: String) -> String {
    //let (bytes,): (Vec<u8>,) = ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ()).await.unwrap();
    //let (r,) = ic_cdk::api::management_canister::main::raw_rand().await.unwrap();
    //convert Vec><u8> into integer from 0 to 9 
    let now = ic_cdk::api::time();
    let byte = now % 10;


    let index = (byte as usize) % STRINGS.len();
    let string = STRINGS[index];
    return string.to_string();


    //format!("Hello, {}!", name)
}


