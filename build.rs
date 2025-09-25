fn main() {
    cc::Build::new()
        .cpp(true)
        .file("c_library/fft.cpp")
        .file("c_library/lib.cpp")
        .compile("ftt");
}
