fn main() {
    panic_in_code();
}


fn exec_panic() {
    panic!("Deu ruim...");
}

fn panic_in_code() {
    let vet = [1,3,4,5];

    vet[99];
}