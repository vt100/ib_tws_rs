#[macro_use]
extern crate tracing;

use std::collections::HashSet;
use std::string::ToString;
use std::time::Duration;

use futures::StreamExt;
use ib_tws_core::domain;
use ib_tws_core::domain::market_data::GenericTick;
use ib_tws_core::message::{request::*, Response};
use miette::IntoDiagnostic;
use sugars::hset;

#[tokio::main]
async fn main() -> miette::Result<()> {
    tracing_subscriber::fmt::init();

    let client = {
        let port = std::env::args()
            .nth(1)
            .and_then(|p| p.parse::<u32>().ok())
            .unwrap_or(4001);
        let transport = ib_tws_tokio::Transport::connect(
            format!("127.0.0.1:{port}").parse().unwrap(),
            Duration::from_secs(5),
        )
        .await
        .into_diagnostic()?;
        ib_tws_core::AsyncClient::setup(transport, 1).await?
    };
    info!(version = client.server_version(), "connected to client");

    let contract = domain::contract::Contract::new_stock("LKE", "ASX", "AUD").unwrap();
    let stock_request = ReqMktData::new(
        contract.clone(),
        hset!{GenericTick::MiscellaneousStats},
        true,
        false,
        Vec::new(),
    );

    let response = client.request_contract_details(ReqContractDetails::new(contract)).await?;
    info!(?response);
    let response = client.request_market_depth_exchanges().await?;
    info!(?response);
    let stream = client.request_market_data(stock_request).await?;
    stream
        .for_each(move |response| async move {
            info!(?response);
        })
        .await;

    client.response_stream()
        .for_each(move |buf| async move {
            match buf {
                Response::ErrMsgMsg(msg) => warn!("{:#?}", msg),
                buf => info!("buf: {:?}", buf),
            }
        })
        .await;
    Ok(())
}
