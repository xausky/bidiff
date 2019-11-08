#![allow(unused)]
use async_std::{prelude::*, io::Read, future::try_join};
use std::pin::Pin;
use log::*;

pub fn test_oibss() {
    let input: &[u8] = &[1, 0, 0, 2, 2, 0, 0, 2, 2, 0, 1, 0];
    oipss::Workspace::new(input);
}

/// Diff two files
pub async fn diff(mut older: Pin<&mut dyn Read>, mut newer: Pin<&mut dyn Read>) -> Result<(), async_std::io::Error>
{
    // let mut obuf = Vec::new();
    // let mut nbuf = Vec::new();

    // {
    //     let a = older.read_to_end(&mut obuf);
    //     let b = newer.read_to_end(&mut nbuf);
    //     try_join!(a, b).await?;
    // }

    // info!("older is {} bytes", obuf.len());
    // info!("newer is {} bytes", nbuf.len());

    Ok(())
}
