use ic_cdk::update;

const JOKES: [&str; 10] = [
    "Dlaczego programista zawsze wraca do domu o północy? Bo adres powrotny to 0x00.",
    "Jakie jest ulubione miejsce programistów na randki? GitHub.",
    "Szef do pracownika: - W tym tygodniu to już czwarte spóźnienie. Jaki z tego należy wyciągnąć wniosek? - Że dziś czwartek.",
    "Co mówi leniwa osoba, gdy słyszy coś niedorzecznego? Nie chce mi się w to wierzyć.",
    "Dlaczego koza wydaje się tania w utrzymaniu? Bo może zeżreć cokolwiek. A dlaczego koza jest droga w utrzymaniu? Bo może zeżreć cokolwiek.",
    "Jaka jest najbardziej królewska cześć trójkąta? Jego wysokość",
    "Pamiętajcie, Titanica zbudowali zawodowcy, a Arkę Noego amatorzy.",
    "Jakie kwiaty zabijają żony? Żonkile.",
    "Dlaczego w kosmosie nie ma dobrych imprez? Bo nie ma atmosfery.",
    "Jaka jest różnica między młotkiem a kukurydzą? Młotek się dłużej gotuje."
];

#[update]
async fn random_joke() -> String {
    let (r,) = ic_cdk::api::management_canister::main::raw_rand().await.unwrap();
    let byte = r[0] as usize;
    let index = byte % 10;
    let joke = JOKES[index];
    return joke.to_string();
}

