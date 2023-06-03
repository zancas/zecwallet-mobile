#![allow(non_snake_case)]

extern crate android_logger;
extern crate log;

use android_logger::{Config, FilterBuilder};
use log::Level;

use jni::objects::{JObject, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use std::ffi::{CStr, CString};

unsafe fn unsafe_unpack_ptr_to_string(j_config_hint: JString, env: &JNIEnv) -> String {
    unsafe {
        CString::from(CStr::from_ptr(
            env.get_string(j_config_hint).unwrap().as_ptr(),
        ))
        .into_string()
        .unwrap()
    }
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_ZingoLabs_Zingo_RustFFI_00024Companion_initlogging(
    env: JNIEnv,
    _: JObject,
) -> jstring {
    android_logger::init_once(
        Config::default().with_min_level(Level::Trace).with_filter(
            FilterBuilder::new()
                .parse("debug,hello::crate=zingolib")
                .build(),
        ),
    );
    env.new_string("OK").unwrap().into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_ZingoLabs_Zingo_RustFFI_00024Companion_initnew(
    env: JNIEnv,
    _: JObject,
    j_serveruri: JString,
    j_data_dir: JString,
    j_chain_hint: JString,
) -> jstring {
    let chain_hint = CString::from(CStr::from_ptr(
        env.get_string(j_chain_hint).unwrap().as_ptr(),
    ))
    .into_string()
    .unwrap();

    let server_uri = CString::from(CStr::from_ptr(
        env.get_string(j_serveruri).unwrap().as_ptr(),
    ))
    .into_string()
    .unwrap();

    let data_dir = CString::from(CStr::from_ptr(env.get_string(j_data_dir).unwrap().as_ptr()))
        .into_string()
        .unwrap();

    let seed = rustlib::init_new(server_uri, data_dir, &chain_hint);

    let output = env.new_string(seed.as_str()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_ZingoLabs_Zingo_RustFFI_00024Companion_initfromseed(
    env: JNIEnv,
    _: JObject,
    j_serveruri: JString,
    j_seed: JString,
    j_birthday: JString,
    j_data_dir: JString,
    j_chain_hint: JString,
) -> jstring {
    let chain_hint = CString::from(CStr::from_ptr(
        env.get_string(j_chain_hint).unwrap().as_ptr(),
    ))
    .into_string()
    .unwrap();
    let server_uri = CString::from(CStr::from_ptr(
        env.get_string(j_serveruri).unwrap().as_ptr(),
    ))
    .into_string()
    .unwrap();

    let seed_tmp = CString::from(CStr::from_ptr(env.get_string(j_seed).unwrap().as_ptr()))
        .into_string()
        .unwrap();

    let birthday = CString::from(CStr::from_ptr(env.get_string(j_birthday).unwrap().as_ptr()))
        .into_string()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let data_dir = CString::from(CStr::from_ptr(env.get_string(j_data_dir).unwrap().as_ptr()))
        .into_string()
        .unwrap();

    let seed = rustlib::init_from_seed(server_uri, seed_tmp, birthday, data_dir, &chain_hint);

    let output = env.new_string(seed.as_str()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_ZingoLabs_Zingo_RustFFI_00024Companion_initfromufvk(
    env: JNIEnv,
    _: JObject,
    j_serveruri: JString,
    j_ufvk: JString,
    j_birthday: JString,
    j_data_dir: JString,
    j_chain_hint: JString,
) -> jstring {
    let chain_hint = CString::from(CStr::from_ptr(
        env.get_string(j_chain_hint).unwrap().as_ptr(),
    ))
    .into_string()
    .unwrap();
    let server_uri = CString::from(CStr::from_ptr(
        env.get_string(j_serveruri).unwrap().as_ptr(),
    ))
    .into_string()
    .unwrap();

    let ufvk_tmp = CString::from(CStr::from_ptr(env.get_string(j_ufvk).unwrap().as_ptr()))
        .into_string()
        .unwrap();

    let birthday = CString::from(CStr::from_ptr(env.get_string(j_birthday).unwrap().as_ptr()))
        .into_string()
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let data_dir = CString::from(CStr::from_ptr(env.get_string(j_data_dir).unwrap().as_ptr()))
        .into_string()
        .unwrap();

    let _no_seed_warning =
        rustlib::init_from_ufvk(server_uri, ufvk_tmp, birthday, data_dir, &chain_hint);

    let output = env
        .new_string("Wallet created from ufvk, no seed available")
        .unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_ZingoLabs_Zingo_RustFFI_00024Companion_initfromb64(
    env: JNIEnv,
    _: JObject,
    j_serveruri: JString,
    j_base64: JString,
    j_data_dir: JString,
    j_chain_hint: JString,
) -> jstring {
    let chain_hint = CString::from(CStr::from_ptr(
        env.get_string(j_chain_hint).unwrap().as_ptr(),
    ))
    .into_string()
    .unwrap();
    let base64 = CString::from(CStr::from_ptr(env.get_string(j_base64).unwrap().as_ptr()))
        .into_string()
        .unwrap();

    let server_uri = CString::from(CStr::from_ptr(
        env.get_string(j_serveruri).unwrap().as_ptr(),
    ))
    .into_string()
    .unwrap();

    let data_dir = CString::from(CStr::from_ptr(env.get_string(j_data_dir).unwrap().as_ptr()))
        .into_string()
        .unwrap();

    let seed = rustlib::init_from_b64(server_uri, base64, data_dir, &chain_hint);

    let output = env.new_string(seed.as_str()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_ZingoLabs_Zingo_RustFFI_00024Companion_save(
    env: JNIEnv,
    _: JObject,
) -> jstring {
    let encoded = rustlib::save_to_b64();
    let output = env.new_string(encoded.as_str()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_ZingoLabs_Zingo_RustFFI_00024Companion_execute(
    env: JNIEnv,
    _: JObject,
    j_command: JString,
    j_argslist: JString,
) -> jstring {
    let cmd = CString::from(CStr::from_ptr(env.get_string(j_command).unwrap().as_ptr()))
        .into_string()
        .unwrap();

    let args_list = CString::from(CStr::from_ptr(env.get_string(j_argslist).unwrap().as_ptr()))
        .into_string()
        .unwrap();

    let resp = rustlib::execute(cmd, args_list);

    let output = env.new_string(resp.as_str()).unwrap();
    output.into_inner()
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_ZingoLabs_Zingo_RustFFI_00024Companion_getlatestblock(
    env: JNIEnv,
    _: JObject,
    j_serveruri: JString,
) -> jstring {
    let server_uri = CString::from(CStr::from_ptr(
        env.get_string(j_serveruri).unwrap().as_ptr(),
    ))
    .into_string()
    .unwrap();

    let resp = rustlib::get_latest_block(server_uri);

    let output = env.new_string(resp.as_str()).unwrap();
    output.into_inner()
}
