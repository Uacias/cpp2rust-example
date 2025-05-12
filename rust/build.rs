fn main() {
    cc::Build::new()
        .cpp(true) // enable C++ linkage
        .include("../cpp/include") // where to find native_lib.h
        .file("../cpp/src/native_lib.cpp") // compile this file
        .flag_if_supported("-std=c++17")
        .compile("native_lib"); // output libnative_lib.a
}
