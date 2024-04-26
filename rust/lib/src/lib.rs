uniffi::include_scaffolding!("rustlib");

#[macro_use]
extern crate lazy_static;
extern crate android_logger;

#[cfg(target_os = "android")]
use android_logger::{Config, FilterBuilder};
use base64::engine::general_purpose::STANDARD;
#[cfg(target_os = "android")]
use log::Level;

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use zingoconfig::{construct_lightwalletd_uri, ChainType, RegtestNetwork, ZingoConfig};
use zingolib::{commands, lightclient::LightClient, wallet::WalletBase};

// We'll use a MUTEX to store a global lightclient instance,
// so we don't have to keep creating it. We need to store it here, in rust
// because we can't return such a complex structure back to JS
lazy_static! {
    static ref LIGHTCLIENT: Mutex<RefCell<Option<Arc<LightClient>>>> =
        Mutex::new(RefCell::new(None));
}

fn lock_client_return_seed(lightclient: LightClient) -> String {
    let lc = Arc::new(lightclient);
    LightClient::start_mempool_monitor(lc.clone());

    LIGHTCLIENT.lock().unwrap().replace(Some(lc));

    execute_command("seed".to_string(), "".to_string())
}

fn construct_uri_load_config(
    uri: String,
    data_dir: String,
    chain_hint: String,
    monitor_mempool: bool,
) -> Result<(ZingoConfig, http::Uri), String> {
    let lightwalletd_uri = construct_lightwalletd_uri(Some(uri));

    let chaintype = match chain_hint.as_str() {
        "main" => ChainType::Mainnet,
        "test" => ChainType::Testnet,
        "regtest" => ChainType::Regtest(RegtestNetwork::all_upgrades_active()),
        _ => return Err("Error: Not a valid chain hint!".to_string()),
    };
    let mut config = match zingoconfig::load_clientconfig(
        lightwalletd_uri.clone(),
        None,
        chaintype,
        monitor_mempool,
    ) {
        Ok(c) => c,
        Err(e) => {
            return Err(format!("Error: Config load: {}", e));
        }
    };
    config.set_data_dir(data_dir);

    Ok((config, lightwalletd_uri))
}

pub fn init_logging() -> String {
    // this is only for Android
    #[cfg(target_os = "android")]
    android_logger::init_once(
        Config::default().with_min_level(Level::Trace).with_filter(
            FilterBuilder::new()
                .parse("debug,hello::crate=zingolib")
                .build(),
        ),
    );

    "OK".to_string()
}

pub fn init_new(
    server_uri: String,
    data_dir: String,
    chain_hint: String,
    monitor_mempool: bool,
) -> String {
    let (config, lightwalletd_uri);
    match construct_uri_load_config(server_uri, data_dir, chain_hint, monitor_mempool) {
        Ok((c, h)) => (config, lightwalletd_uri) = (c, h),
        Err(s) => return s,
    }
    let latest_block_height = match zingolib::get_latest_block_height(lightwalletd_uri)
        .map_err(|e| format! {"Error: {e}"})
    {
        Ok(height) => height,
        Err(e) => return e,
    };
    let lightclient = match LightClient::new(&config, latest_block_height.saturating_sub(100)) {
        Ok(l) => l,
        Err(e) => {
            return format!("Error: {}", e);
        }
    };
    lock_client_return_seed(lightclient)
}

pub fn init_from_seed(
    server_uri: String,
    seed: String,
    birthday: u64,
    data_dir: String,
    chain_hint: String,
    monitor_mempool: bool,
) -> String {
    let (config, _lightwalletd_uri);
    match construct_uri_load_config(server_uri, data_dir, chain_hint, monitor_mempool) {
        Ok((c, h)) => (config, _lightwalletd_uri) = (c, h),
        Err(s) => return s,
    }
    let lightclient = match LightClient::create_from_wallet_base(
        WalletBase::MnemonicPhrase(seed),
        &config,
        birthday,
        false,
    ) {
        Ok(l) => l,
        Err(e) => {
            return format!("Error: {}", e);
        }
    };
    lock_client_return_seed(lightclient)
}

pub fn init_from_ufvk(
    server_uri: String,
    ufvk: String,
    birthday: u64,
    data_dir: String,
    chain_hint: String,
    monitor_mempool: bool,
) -> String {
    let (config, _lightwalletd_uri);
    match construct_uri_load_config(server_uri, data_dir, chain_hint, monitor_mempool) {
        Ok((c, h)) => (config, _lightwalletd_uri) = (c, h),
        Err(s) => return s,
    }
    let lightclient = match LightClient::create_from_wallet_base(
        WalletBase::Ufvk(ufvk),
        &config,
        birthday,
        false,
    ) {
        Ok(l) => l,
        Err(e) => {
            return format!("Error: {}", e);
        }
    };
    lock_client_return_seed(lightclient)
}

pub fn init_from_b64(
    server_uri: String,
    base64_data: String,
    data_dir: String,
    chain_hint: String,
    monitor_mempool: bool,
) -> String {
    let (config, _lightwalletd_uri);
    match construct_uri_load_config(server_uri, data_dir, chain_hint, monitor_mempool) {
        Ok((c, h)) => (config, _lightwalletd_uri) = (c, h),
        Err(s) => return s,
    }
    let decoded_bytes = match decode_with_error_failover(&base64_data) {
        Ok(vu8) => vu8,
        Err(e) => return e,
    };

    let lightclient =
        match LightClient::read_wallet_from_buffer_runtime(&config, &decoded_bytes[..]) {
            Ok(l) => l,
            Err(e) => {
                return format!("Error: {}", e);
            }
        };
    lock_client_return_seed(lightclient)
}

fn decode_with_error_failover(base64_data: &str) -> Result<Vec<u8>, String> {
    match STANDARD_NO_PAD.decode(&base64_data) {
        Ok(b) => Ok(b),
        Err(base64::DecodeError::InvalidPadding) => match STANDARD.decode(&base64_data) {
            Ok(b) => Ok(b),
            Err(e) => {
                return Err(format!("Error: Decoding Base64: {}", e));
            }
        },
        _ => todo!(),
    }
}
/*
#cfg(test)]
mod test {
    #[test]
    fn decode_b64(){

    }
}
*/
pub fn save_to_b64() -> String {
    // Return the wallet as a base64 encoded string
    let lightclient: Arc<LightClient>;
    {
        let lc = LIGHTCLIENT.lock().unwrap();

        if lc.borrow().is_none() {
            return "Error: Lightclient is not initialized".to_string();
        }

        lightclient = lc.borrow().as_ref().unwrap().clone();
    };

    match lightclient.export_save_buffer_runtime() {
        Ok(buf) => STANDARD_NO_PAD.encode(&buf),
        Err(e) => {
            format!("Error: {}", e)
        }
    }
}

pub fn execute_command(cmd: String, args_list: String) -> String {
    let resp: String;
    {
        let lightclient: Arc<LightClient>;
        {
            let lc = LIGHTCLIENT.lock().unwrap();

            if lc.borrow().is_none() {
                return "Error: Lightclient is not initialized".to_string();
            }

            lightclient = lc.borrow().as_ref().unwrap().clone();
        };

        let args = if args_list.is_empty() {
            vec![]
        } else {
            vec![args_list.as_ref()]
        };
        resp = commands::do_user_command(&cmd, &args, lightclient.as_ref()).clone();
    };

    resp
}

pub fn get_latest_block_server(server_uri: String) -> String {
    let lightwalletd_uri: http::Uri = server_uri.parse().expect("To be able to represent a Uri.");
    match zingolib::get_latest_block_height(lightwalletd_uri).map_err(|e| format! {"Error: {e}"}) {
        Ok(height) => height.to_string(),
        Err(e) => e,
    }
}
