use jni::objects::{JObject, JString};
use jni::sys::jint;
use jni::{jni_mangle, EnvUnowned};

#[jni_mangle("me.fortibrine.greeter.RustBridge")]
pub fn init_plugin(
    mut unowned_env: EnvUnowned,
    _: JObject
) {
    let outcome = unowned_env.with_env(|_| -> Result<(), jni::errors::Error> {
        Ok(())
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}

#[jni_mangle("me.fortibrine.greeter.RustBridge")]
pub fn get_secret_key<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _: JObject,
    a: jint,
) -> JString<'caller> {
    let outcome = unowned_env.with_env(|env| -> Result<_, jni::errors::Error> {
        let secret = format!("Secret key: {}", a);
        JString::from_str(env, secret)
    });

    outcome.resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
