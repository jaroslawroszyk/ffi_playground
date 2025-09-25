fn main() {
    cc::Build::new()
        .file("c_libary/lib.c") // poprawiona ścieżka
        .compile("hello");
}
