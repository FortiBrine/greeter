use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::EnvUnowned;

#[unsafe(no_mangle)]
pub extern "system" fn Java_me_fortibrine_greeter_RustBridge_getSecretKey<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _class: JClass,
    a: jint,
) -> JString<'caller> {
    let outcome = unowned_env.with_env(|env| -> Result<_, jni::errors::Error> {
        let secret = format!("Secret key: {}", a);
        JString::from_str(env, secret)
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}