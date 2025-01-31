# Dangling Pointer Bug in Rust

This repository demonstrates a common error in Rust involving dangling pointers when working with unsafe code and vectors. The code attempts to dereference a pointer after the vector it points to has been dropped, resulting in undefined behavior.  The solution shows how to avoid this issue using safe Rust practices.