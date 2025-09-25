fn main() {
    cc::Build::new()
        .cpp(true)
        .file("c_libary/fft.cpp")
        .compile("ftt");
}
