extern crate libc;
use self::libc::{c_int, c_float, c_double, size_t};
use slice::raw;

pub fn test() {
    println!("It works!");
}

extern "C" {
    pub fn rMPI_Init(argc: &[int], argv: &[[char]char]) -> int {
        unsafe {
            let pargc = argc.as_ptr();
            let pargv = argv.as_mut_prt();
            let ret = MPI_Init(pargc, pargv);
            return ret;
        }
    }
    pub fn rMPI_Finalize() -> int {
        unsafe {
            let ret = MPI_Finalize();
            return ret;
        }
    }
}
