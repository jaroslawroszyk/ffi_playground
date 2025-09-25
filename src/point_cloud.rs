use std::os::raw::c_double;

#[repr(C)]
struct PointCloud {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn point_cloud_new() -> *mut PointCloud;
    fn point_cloud_free(pc: *mut PointCloud);
    fn point_cloud_add(pc: *mut PointCloud, x: c_double, y: c_double, z: c_double);
    fn point_cloud_centroid(pc: *mut PointCloud, out: *mut c_double);
}

fn main() {
    unsafe {
        let pc = point_cloud_new();
        if pc.is_null() { panic!("failed to allocate"); }

        point_cloud_add(pc, 1.0, 2.0, 3.0);
        point_cloud_add(pc, 4.0, 5.0, 6.0);

        let mut out = [0.0f64; 3];
        point_cloud_centroid(pc, out.as_mut_ptr());

        println!("Centroid: {:?}", out);

        point_cloud_free(pc);
    }
}