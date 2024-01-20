use std::{ffi::CString, ptr::null_mut};

use glfw_sys::{
    glfwCreateWindow, glfwInit, glfwMakeContextCurrent, glfwPollEvents, glfwSetKeyCallback,
    glfwSetWindowShouldClose, glfwSwapBuffers, glfwTerminate, glfwWindowHint,
    glfwWindowShouldClose, GLFWwindow, GLFW_CONTEXT_VERSION_MAJOR, GLFW_CONTEXT_VERSION_MINOR,
    GLFW_KEY_ESCAPE, GLFW_OPENGL_CORE_PROFILE, GLFW_OPENGL_PROFILE, GLFW_PRESS, GLFW_TRUE,
    GLFW_VERSION_MAJOR, GLFW_VERSION_MINOR, GLFW_VERSION_REVISION,
};

unsafe extern "C" fn key_callback(
    window: *mut GLFWwindow,
    key: ::std::os::raw::c_int,
    _scancode: ::std::os::raw::c_int,
    action: ::std::os::raw::c_int,
    _mods: ::std::os::raw::c_int,
) {
    if key == GLFW_KEY_ESCAPE && action == GLFW_PRESS {
        glfwSetWindowShouldClose(window, GLFW_TRUE)
    }
}

fn main() {
    println!("Hello, glfw_sys!");
    println!(
        "Using glfw version {}.{}.{}",
        GLFW_VERSION_MAJOR, GLFW_VERSION_MINOR, GLFW_VERSION_REVISION
    );

    unsafe {
        glfwInit();
        glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
        glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 5);
        glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

        let c_str = CString::new("Hello from glfw_sys!").unwrap();
        let window = glfwCreateWindow(640, 480, c_str.as_ptr(), null_mut(), null_mut());
        glfwMakeContextCurrent(window);

        glfwSetKeyCallback(window, Some(key_callback));

        while !(glfwWindowShouldClose(window) == GLFW_TRUE) {
            glfwPollEvents();
            glfwSwapBuffers(window);
        }

        glfwTerminate();
    };
}
