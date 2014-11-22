#![feature(globs)]
mod rustMPI;
fn main()
{
    rustMPI::rMPI_Init();
    println!("Testing rustMPI...");
    let rank: int = 0;
    rustMPI::rMPI_Comm_rank(rustMPI::MPI_COMM_WORLD,&rank);
    println!("{}",rank);
    rustMPI::rMPI_Finalize();
}
