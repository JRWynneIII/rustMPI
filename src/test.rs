#![feature(globs)]
mod rustMPI;
fn main()
{
    //Won't compile. argc and argv haven't been defined. how would i get that??
    rustMPI::test();
    rustMPI::rMPI_Init();
    println!("HelloooO");
    rustMPI::rMPI_Finalize();
}
