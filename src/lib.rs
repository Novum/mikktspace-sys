#[allow(clippy::too_many_arguments)]
pub trait MikkTSpaceInterface {
    fn get_num_faces(&self) -> usize;
    fn get_num_vertices_of_face(&self, face: usize) -> usize;
    fn get_position(&self, face: usize, vert: usize) -> [f32; 3];
    fn get_normal(&self, face: usize, vert: usize) -> [f32; 3];
    fn get_tex_coord(&self, face: usize, vert: usize) -> [f32; 2];
    fn set_tspace_basic(&mut self, _tangent: [f32; 3], _sign: f32, _face: usize, _vert: usize) {}
    fn set_tspace(
        &mut self,
        _tangent: [f32; 3],
        _bi_tangent: [f32; 3],
        _mag_s: f32,
        _mag_t: f32,
        _is_orientation_preserving: bool,
        _face: usize,
        _vert: usize,
    ) {
    }
}

mod ffi {
    use std::os::raw::{c_float, c_int};

    #[repr(C)]
    pub(super) struct SMikkTSpaceContext {
        interface: *const SMikkTSpaceInterface,
        user_data: *mut std::ffi::c_void,
    }

    #[repr(C)]
    pub(super) struct SMikkTSpaceInterface {
        get_num_faces: extern "C" fn(context: *const SMikkTSpaceContext) -> c_int,
        get_num_vertices_of_face:
            extern "C" fn(context: *const SMikkTSpaceContext, face: c_int) -> c_int,
        get_position: extern "C" fn(
            context: *const SMikkTSpaceContext,
            pos_out: *mut c_float,
            face: c_int,
            vert: c_int,
        ),
        get_normal: extern "C" fn(
            context: *const SMikkTSpaceContext,
            norm_out: *mut c_float,
            face: c_int,
            vert: c_int,
        ),
        get_tex_coord: extern "C" fn(
            context: *const SMikkTSpaceContext,
            texc_out: *mut c_float,
            face: c_int,
            vert: c_int,
        ),
        set_tspace_basic: extern "C" fn(
            context: *const SMikkTSpaceContext,
            tangent: *const c_float,
            sign: c_float,
            face: c_int,
            vert: c_int,
        ),
        set_tspace: extern "C" fn(
            context: *const SMikkTSpaceContext,
            tangent: *const c_float,
            bi_tangent: *const c_float,
            mag_s: c_float,
            mag_t: c_float,
            is_orientation_preserving: c_int,
            face: c_int,
            vert: c_int,
        ),
    }

    extern "C" {
        pub(super) fn genTangSpaceDefault(context: *const SMikkTSpaceContext) -> c_int;
        pub(super) fn genTangSpace(
            context: *const SMikkTSpaceContext,
            angular_threshold: c_float,
        ) -> c_int;
    }

    extern "C" fn get_num_faces_callback(context: *const SMikkTSpaceContext) -> c_int {
        unsafe {
            let interface = &(*((*context).user_data as *const InterfaceWrapper)).interface;
            interface.get_num_faces() as c_int
        }
    }

    extern "C" fn get_num_vertices_of_face_callback(
        context: *const SMikkTSpaceContext,
        face: c_int,
    ) -> c_int {
        unsafe {
            let interface = &(*((*context).user_data as *const InterfaceWrapper)).interface;
            interface.get_num_vertices_of_face(face as usize) as c_int
        }
    }

    extern "C" fn get_position_callback(
        context: *const SMikkTSpaceContext,
        pos_out: *mut c_float,
        face: c_int,
        vert: c_int,
    ) {
        unsafe {
            let interface = &(*((*context).user_data as *const InterfaceWrapper)).interface;
            let pos = interface.get_position(face as usize, vert as usize);
            *pos_out.offset(0) = pos[0];
            *pos_out.offset(1) = pos[1];
            *pos_out.offset(2) = pos[2];
        }
    }

    extern "C" fn get_normal_callback(
        context: *const SMikkTSpaceContext,
        norm_out: *mut c_float,
        face: c_int,
        vert: c_int,
    ) {
        unsafe {
            let interface = &(*((*context).user_data as *const InterfaceWrapper)).interface;
            let normal = interface.get_normal(face as usize, vert as usize);
            *norm_out.offset(0) = normal[0];
            *norm_out.offset(1) = normal[1];
            *norm_out.offset(2) = normal[2];
        }
    }

    extern "C" fn get_tex_coord_callback(
        context: *const SMikkTSpaceContext,
        texc_out: *mut c_float,
        face: c_int,
        vert: c_int,
    ) {
        unsafe {
            let interface = &(*((*context).user_data as *const InterfaceWrapper)).interface;
            let tex_coord = interface.get_tex_coord(face as usize, vert as usize);
            *texc_out.offset(0) = tex_coord[0];
            *texc_out.offset(1) = tex_coord[1];
        }
    }

    extern "C" fn set_tspace_basic_callback(
        context: *const SMikkTSpaceContext,
        tangent: *const c_float,
        sign: c_float,
        face: c_int,
        vert: c_int,
    ) {
        unsafe {
            let interface = &mut (*((*context).user_data as *mut InterfaceWrapper)).interface;
            let tangent_arr = [*tangent.offset(0), *tangent.offset(1), *tangent.offset(2)];
            interface.set_tspace_basic(tangent_arr, sign, face as usize, vert as usize);
        }
    }

    extern "C" fn set_tspace_callback(
        context: *const SMikkTSpaceContext,
        tangent: *const c_float,
        bi_tangent: *const c_float,
        mag_s: c_float,
        mag_t: c_float,
        is_orientation_preserving: c_int,
        face: c_int,
        vert: c_int,
    ) {
        unsafe {
            let interface = &mut (*((*context).user_data as *mut InterfaceWrapper)).interface;
            let tangent_arr = [*tangent.offset(0), *tangent.offset(1), *tangent.offset(2)];
            let bi_tangent_arr = [
                *bi_tangent.offset(0),
                *bi_tangent.offset(1),
                *bi_tangent.offset(2),
            ];
            interface.set_tspace(
                tangent_arr,
                bi_tangent_arr,
                mag_s,
                mag_t,
                is_orientation_preserving != 0,
                face as usize,
                vert as usize,
            );
        }
    }

    const MIKK_INTERFACE: SMikkTSpaceInterface = SMikkTSpaceInterface {
        get_num_faces: get_num_faces_callback,
        get_num_vertices_of_face: get_num_vertices_of_face_callback,
        get_position: get_position_callback,
        get_normal: get_normal_callback,
        get_tex_coord: get_tex_coord_callback,
        set_tspace_basic: set_tspace_basic_callback,
        set_tspace: set_tspace_callback,
    };

    pub(super) struct InterfaceWrapper<'a> {
        pub(super) interface: &'a mut dyn super::MikkTSpaceInterface,
    }

    pub(super) fn create_context(interface_wrapper: &InterfaceWrapper) -> SMikkTSpaceContext {
        SMikkTSpaceContext {
            interface: &MIKK_INTERFACE as *const _,
            user_data: interface_wrapper as *const _ as *mut _,
        }
    }
}

pub fn gen_tang_space_default<I>(interface: &mut I) -> bool
where
    I: MikkTSpaceInterface,
{
    let interface_wrapper = ffi::InterfaceWrapper { interface };
    let context = ffi::create_context(&interface_wrapper);
    unsafe { ffi::genTangSpaceDefault(&context) != 0 }
}

pub fn gen_tang_space<I>(interface: &mut I, angular_threshold: f32) -> bool
where
    I: MikkTSpaceInterface,
{
    let interface_wrapper = ffi::InterfaceWrapper { interface };
    let context = ffi::create_context(&interface_wrapper);
    unsafe { ffi::genTangSpace(&context, angular_threshold) != 0 }
}

#[link(name = "mikktspace")]
extern "C" {}
