#[allow(non_snake_case_functions)]
extern crate libc;
pub use self::libc::{c_int,c_float,c_double,c_char};
use std::vec;
use std::os;
use std::ptr;

pub fn test() {
    println!("It works!");
}
#[link(name = "pthread")]
#[link(name = "mpi")]
//Uncomment the rest for compiling on Rhea
//#[link(name = "osmcomp")]
//#[link(name = "rdmacm")]
//#[link(name = "ibverbs")]
//#[link(name = "rt")]
//#[link(name = "nsl")]
//#[link(name = "util")]
//#[link(name = "psm_infinipath")]
//#[link(name = "torque")]
//#[link(name = "dl")]
//#[link(name = "m")]
//#[link(name = "numa")]
//#[link(name = "rt")]
//#[link(name = "nsl")]
//#[link(name = "util")]
extern "C" {
    fn MPI_Init(argc: *const c_int, argv: *const c_char) -> int;
    fn MPI_Finalize() -> int;
    fn MPI_Comm_rank(comm: c_int, rank: &int) -> int;
}

pub static MPI_COMM_WORLD: int = 9;

pub fn rMPI_Init() -> int {
    unsafe {
        let argc = ptr::null();
        let argv = ptr::null();
        let ret = MPI_Init(argc, argv);
        return ret;
    }
}

pub fn rMPI_Comm_rank(comm: int, rank: &int) -> int {
    unsafe {
        let c_comm:c_int = (comm as c_int);
        let c_rank:&int = rank;
        let ret: int = MPI_Comm_rank(c_comm,c_rank);
        return ret;
    }
}

pub fn rMPI_Finalize() -> int {
    unsafe {
        let ret = MPI_Finalize();
        return ret;
    }
}
