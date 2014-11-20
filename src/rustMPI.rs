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
#[link(name = "osmcomp")]
#[link(name = "rdmacm")]
#[link(name = "ibverbs")]
#[link(name = "rt")]
#[link(name = "nsl")]
#[link(name = "util")]
#[link(name = "psm_infinipath")]
#[link(name = "torque")]
#[link(name = "dl")]
#[link(name = "m")]
#[link(name = "numa")]
#[link(name = "rt")]
#[link(name = "nsl")]
#[link(name = "util")]
extern "C" {
    fn MPI_Init(argc: *const c_int, argv: *const c_char) -> int;
    fn MPI_Finalize() -> int;
}

#[fixed_stack_segment]
pub fn rMPI_Init() -> int {
    unsafe {
        let argc = ptr::null();
        let argv = ptr::null();
        let ret = MPI_Init(argc, argv);
        return ret;
    }
}

#[fixed_stack_segment]
pub fn rMPI_Finalize() -> int {
    unsafe {
        let ret = MPI_Finalize();
        return ret;
    }
}
