#[cfg(not(feature = "regchest"))]
use zingo_testutils::{self, scenarios};

#[cfg(feature = "ci")]
const UNIX_SOCKET: Option<&str> = Some("/Users/runner/.colima/default/docker.sock");
#[cfg(all(not(feature = "ci"), feature = "regchest"))]
const UNIX_SOCKET: Option<&str> = None;

async fn e2e_reload_while_tx_unconfirmed() {
    #[cfg(not(feature = "regchest"))]
    let (_regtest_manager, _child_process_handler) =
        scenarios::funded_orchard_mobileclient(1_000_000).await;
    #[cfg(feature = "regchest")]
    let docker =
        match regchest_utils::launch(UNIX_SOCKET, Some("funded_orchard_mobileclient")).await {
            Ok(d) => d,
            Err(e) => panic!("Failed to launch regchest docker container: {:?}", e),
        };

    let (exit_code, output, error) =
        zingomobile_utils::android_e2e_test("reload_while_tx_unconfirmed.test.js");

    #[cfg(feature = "regchest")]
    match regchest_utils::close(&docker).await {
        Ok(_) => (),
        Err(e) => panic!("Failed to close regchest docker container: {:?}", e),
    }

    println!("Exit Code: {}", exit_code);
    println!("Output: {}", output);
    println!("Error: {}", error);

    assert_eq!(exit_code, 0);
}
