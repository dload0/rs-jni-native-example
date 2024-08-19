/// Standard library imports.
///
/// # Contains:
/// ```
/// * std: Standard library.
/// ```
use std :: os :: raw :: c_void;

/// JNI library imports.
///
/// # Contains:
/// ```
/// * jni: For JNI bindings.
/// ```
use jni :: {
    objects :: JClass,
    sys :: {
        jint,
        JNI_VERSION_1_6,
    },
    JNIEnv,
    JavaVM,
};

#[no_mangle]
/// This function is called when the JVM loads the native library.
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *mut c_void) -> jint {
    // Print debug. 
    println!("[ ! ] `JNI_OnLoad` called.");

    // Return the JNI version that the library supports.
    JNI_VERSION_1_6
}

#[no_mangle]
/// This function is called from the Java side.
pub unsafe extern "system" fn Java_dev_dload_NativeLoader_bootstrap(mut jni_env: JNIEnv, caller_class: JClass) {
    // Print debug. 
    println!("[ ! ] `Java_dev_dload_NativeLoader_bootstrap` called.");
}
