/// Standard library imports.
///
/// # Contains:
/// ```
/// * std: Standard library.
/// ```
use std :: { os :: raw :: c_void, process :: exit };

/// JNI library imports.
///
/// # Contains:
/// ```
/// * jni: For JNI bindings.
/// * simple_jvmti: For JVMTI bindings.
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
use simple_jvmti :: {
    jvmti_sys :: JVMTI_VERSION_1_2,
    sync :: JvmtiSupplier
};

#[no_mangle]
/// This function is called when the JVM loads the native library.
pub unsafe extern "system" fn JNI_OnLoad(vm: JavaVM, _reserved: *mut c_void) -> jint {
    // Print debug.
    println!("[ ! ] `JNI_OnLoad` called.");

    let jni_env = vm.get_env().unwrap_or_else(|_| destroy_vm(&vm));
    println!("[ ! ] Got JNI environment.");

    let jvmti_env = vm.get_jvmti_env(JVMTI_VERSION_1_2 as _);
    println!(
        "[ ! ] Got JVMTI environment | version [ {} ]",
        jvmti_env.get_version_number().unwrap()
    );

    // Return the JNI version that the library supports.
    JNI_VERSION_1_6
}

#[no_mangle]
/// This function is called from the Java side.
pub unsafe extern "system" fn Java_dev_dload_NativeLoader_bootstrap(
    mut jni_env: JNIEnv,
    caller_class: JClass,
) {
    // Print debug.
    println!("[ ! ] `Java_dev_dload_NativeLoader_bootstrap` called.");
}

#[inline(always)]
/// Kill the JVM and exit the process.
unsafe fn destroy_vm(vm: &JavaVM) -> ! {
    // Destroy the Java VM.
    let _ = vm.destroy();

    // Exit the process.
    exit(0);
}
