use ntp_daemon::ObservablePeerState;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let stream = tokio::net::UnixStream::connect("/run/ntpd-rs/observe").await?;

    stream.readable().await?;

    let mut msg = Vec::with_capacity(16 * 1024);

    loop {
        // Wait for the socket to be readable
        stream.readable().await?;

        // Try to read data, this may still fail with `WouldBlock`
        // if the readiness event is a false positive.
        // will allocate more space if needed.
        match stream.try_read_buf(&mut msg) {
            Ok(n) => {
                msg.truncate(n);
                break;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }

    let output: Vec<ObservablePeerState> = serde_json::from_slice(&msg).unwrap();

    dbg!(output);

    Ok(())
}