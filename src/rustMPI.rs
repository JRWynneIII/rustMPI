pub fn test() {
    println!("It works!");
}

extern "C" {
    pub fn MPI_Send();
}
