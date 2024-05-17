
const JOKES: [&str; 10] = [
    "Dlaczego komputerowi nie warto ufać? Bo kiedy mówi, że ma 4 bajty, to kłamie. Ma 1 bajt i 3 bity.",
    "Dlaczego programista nie lubi natury? Bo ma za dużo bugów.",
    "Dlaczego programista zawsze wraca do domu o północy? Bo adres powrotny to 0x00.",
    "Jakie jest ulubione miejsce programistów na randki? GitHub.",
    "Dlaczego programiści zawsze mieszają Halloween z Bożym Narodzeniem? Bo 31 OCT = 25 DEC.",
    "Dlaczego programista nie używa łóżka? Bo nie może znaleźć śpiwora.",
    "Dlaczego programista nie używa Facebooka? Bo nie może polubić strony, która nie ma źródła.",
    "Dlaczego programista nie idzie do lekarza? Bo Google ma wszystkie odpowiedzi.",
    "Dlaczego programista nie idzie na siłownię? Bo nie lubi ciężkich obciążeń.",
    "Dlaczego programista nie idzie do kina? Bo nie lubi trailerów."
];
#[ic_cdk::query]
fn greet(name: String) -> String {
    //let (bytes,): (Vec<u8>,) = ic_cdk::api::call::call(Principal::management_canister(), "raw_rand", ()).await.unwrap();
    //let (r,) = ic_cdk::api::management_canister::main::raw_rand().await.unwrap();
    //convert Vec><u8> into integer from 0 to 9 
    let now = ic_cdk::api::time();
    let byte = now % 10;


    let index = (byte as usize) % JOKES.len();
    let string = JOKES[index];
    return string.to_string();


    //format!("Hello, {}!", name)
}


