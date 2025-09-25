fn main() {
    cc::Build::new()
        .cpp(true)
        .file("c_libary/point_cloud.cpp")
        .compile("pointcloud");
}
