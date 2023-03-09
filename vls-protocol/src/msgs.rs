#![allow(missing_docs)]
#![allow(deprecated)]

use alloc::vec::Vec;
use as_any::AsAny;
use core::fmt::Debug;

use crate::error::{Error, Result};
use crate::io::{read_bytes, read_u16, read_u32, read_u64};
use crate::model::*;
use bolt_derive::{ReadMessage, SerBolt};
use serde::{de, ser};
use serde_bolt::{from_vec as sb_from_vec, to_vec, WireString};
use serde_bolt::{LargeOctets, Octets, Read, Write};
use serde_derive::{Deserialize, Serialize};

use log::error;

const MAX_MESSAGE_SIZE: u32 = 65536;

/// Serialize a message with a type prefix, in BOLT style
pub trait SerBolt: Debug + AsAny + Send {
    fn as_vec(&self) -> Vec<u8>;
}

pub trait DeBolt: Debug + Sized {
    const TYPE: u16;
    fn from_vec(ser: Vec<u8>) -> Result<Self>;
}

/// hsmd Init
/// CLN only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(11)]
pub struct HsmdInit {
    pub key_version: Bip32KeyVersion,
    pub chain_params: BlockId,
    pub encryption_key: Option<DevSecret>,
    pub dev_privkey: Option<DevPrivKey>,
    pub dev_bip32_seed: Option<DevSecret>,
    pub dev_channel_secrets: Option<Vec<DevSecret>>,
    pub dev_channel_secrets_shaseed: Option<Sha256>,
    pub hsm_wire_min_version: u32,
    pub hsm_wire_max_version: u32,
}

/// deprecated in CLN
#[deprecated]
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(111)]
pub struct HsmdInitReplyV1 {
    pub node_id: PubKey,
    pub bip32: ExtKey,
    pub bolt12: PubKey32,
    pub onion_reply_secret: Secret,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(113)]
pub struct HsmdInitReplyV2 {
    pub node_id: PubKey,
    pub bip32: ExtKey,
    pub bolt12: PubKey,
}

/// Signer Init for LDK
/// LDK only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1011)]
pub struct HsmdInit2 {
    pub derivation_style: u8,
    pub network_name: WireString,
    pub dev_seed: Option<DevSecret>,
    pub dev_allowlist: Vec<WireString>,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1111)]
pub struct HsmdInit2Reply {
    pub node_id: PubKey,
    pub bip32: ExtKey,
    pub bolt12: PubKey,
}

/// Get node public keys.
/// Used by the frontend
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1012)]
pub struct NodeInfo {}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1112)]
pub struct NodeInfoReply {
    pub network_name: WireString,
    pub node_id: PubKey,
    pub bip32: ExtKey,
}

/// Connect a new client
/// CLN only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(9)]
pub struct ClientHsmFd {
    pub peer_id: PubKey,
    pub dbid: u64,
    pub capabilities: u64,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(109)]
pub struct ClientHsmFdReply {
    // TODO fd handling
}

/// Sign invoice
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(8)]
pub struct SignInvoice {
    pub u5bytes: Octets,
    pub hrp: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(108)]
pub struct SignInvoiceReply {
    pub signature: RecoverableSignature,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(7)]
pub struct SignWithdrawal {
    pub utxos: Vec<Utxo>,
    pub psbt: LargeOctets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(107)]
pub struct SignWithdrawalReply {
    pub psbt: LargeOctets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1)]
pub struct Ecdh {
    pub point: PubKey,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(100)]
pub struct EcdhReply {
    pub secret: Secret,
}

/// Memleak
/// CLN only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(33)]
pub struct Memleak {}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(133)]
pub struct MemleakReply {
    pub result: bool,
}

/// CheckFutureSecret
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(22)]
pub struct CheckFutureSecret {
    pub commitment_number: u64,
    pub secret: DisclosedSecret,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(122)]
pub struct CheckFutureSecretReply {
    pub result: bool,
}

/// SignMessage
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(23)]
pub struct SignMessage {
    pub message: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(123)]
pub struct SignMessageReply {
    pub signature: RecoverableSignature,
}

/// SignBolt12
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(25)]
pub struct SignBolt12 {
    pub message_name: WireString,
    pub field_name: WireString,
    pub merkle_root: Sha256,
    pub public_tweak: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(125)]
pub struct SignBolt12Reply {
    pub signature: Signature,
}

/// PreapproveInvoice {
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(38)]
pub struct PreapproveInvoice {
    pub invstring: WireString,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(138)]
pub struct PreapproveInvoiceReply {
    pub result: bool,
}

/// PreapproveKeysend {
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(39)]
pub struct PreapproveKeysend {
    pub destination: PubKey,
    pub payment_hash: Sha256,
    pub amount_msat: u64,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(139)]
pub struct PreapproveKeysendReply {
    pub result: bool,
}

/// DeriveSecret
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(27)]
pub struct DeriveSecret {
    pub info: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(127)]
pub struct DeriveSecretReply {
    pub secret: Secret,
}

/// Sign channel update
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(3)]
pub struct SignChannelUpdate {
    pub update: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(103)]
pub struct SignChannelUpdateReply {
    pub update: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2)]
pub struct SignChannelAnnouncement {
    pub announcement: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(102)]
pub struct SignChannelAnnouncementReply {
    pub node_signature: Signature,
    pub bitcoin_signature: Signature,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(6)]
pub struct SignNodeAnnouncement {
    pub announcement: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(106)]
pub struct SignNodeAnnouncementReply {
    pub signature: Signature,
}

/// Get per-commitment point n and optionally revoke a point n-2 by releasing the secret
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(18)]
pub struct GetPerCommitmentPoint {
    pub commitment_number: u64,
}

/// Get per-commitment point
/// LDK only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1018)]
pub struct GetPerCommitmentPoint2 {
    pub commitment_number: u64,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(118)]
pub struct GetPerCommitmentPointReply {
    pub point: PubKey,
    pub secret: Option<DisclosedSecret>,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1118)]
pub struct GetPerCommitmentPoint2Reply {
    pub point: PubKey,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(31)]
pub struct ReadyChannel {
    pub is_outbound: bool,
    pub channel_value: u64,
    pub push_value: u64,
    pub funding_txid: TxId,
    pub funding_txout: u16,
    pub to_self_delay: u16,
    pub local_shutdown_script: Octets,
    pub local_shutdown_wallet_index: Option<u32>,
    pub remote_basepoints: Basepoints,
    pub remote_funding_pubkey: PubKey,
    pub remote_to_self_delay: u16,
    pub remote_shutdown_script: Octets,
    pub channel_type: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(131)]
pub struct ReadyChannelReply {}

///
/// CLN only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(35)]
pub struct ValidateCommitmentTx {
    pub tx: LargeOctets,
    pub psbt: LargeOctets,
    pub htlcs: Vec<Htlc>,
    pub commitment_number: u64,
    pub feerate: u32,
    pub signature: BitcoinSignature,
    pub htlc_signatures: Vec<BitcoinSignature>,
}

///
/// LDK only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1035)]
pub struct ValidateCommitmentTx2 {
    pub commitment_number: u64,
    pub feerate: u32,
    pub to_local_value_sat: u64,
    pub to_remote_value_sat: u64,
    pub htlcs: Vec<Htlc>,
    pub signature: BitcoinSignature,
    pub htlc_signatures: Vec<BitcoinSignature>,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(135)]
pub struct ValidateCommitmentTxReply {
    pub old_commitment_secret: Option<DisclosedSecret>,
    pub next_per_commitment_point: PubKey,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(36)]
pub struct ValidateRevocation {
    pub commitment_number: u64,
    pub commitment_secret: DisclosedSecret,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(136)]
pub struct ValidateRevocationReply {}

///
/// CLN only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(5)]
pub struct SignCommitmentTx {
    pub peer_id: PubKey,
    pub dbid: u64,
    pub tx: LargeOctets,
    pub psbt: LargeOctets,
    pub remote_funding_key: PubKey,
    pub commitment_number: u64,
}

///
/// LDK only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1005)]
pub struct SignLocalCommitmentTx2 {
    pub commitment_number: u64,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1006)]
pub struct SignGossipMessage {
    pub message: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1106)]
pub struct SignGossipMessageReply {
    pub signature: Signature,
}

///
/// CLN only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(19)]
pub struct SignRemoteCommitmentTx {
    pub tx: LargeOctets,
    pub psbt: LargeOctets,
    pub remote_funding_key: PubKey,
    pub remote_per_commitment_point: PubKey,
    pub option_static_remotekey: bool,
    pub commitment_number: u64,
    pub htlcs: Vec<Htlc>,
    pub feerate: u32,
}

/// Ping request
/// LDK only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1000)]
pub struct Ping {
    pub id: u16,
    pub message: WireString,
}

/// Ping reply
/// LDK only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1100)]
pub struct Pong {
    pub id: u16,
    pub message: WireString,
}

///
/// LDK only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1019)]
pub struct SignRemoteCommitmentTx2 {
    pub remote_per_commitment_point: PubKey,
    pub commitment_number: u64,
    pub feerate: u32,
    pub to_local_value_sat: u64,
    pub to_remote_value_sat: u64,
    pub htlcs: Vec<Htlc>,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1119)]
pub struct SignCommitmentTxWithHtlcsReply {
    pub signature: BitcoinSignature,
    pub htlc_signatures: Vec<BitcoinSignature>,
}

///
/// CLN only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(12)]
pub struct SignDelayedPaymentToUs {
    pub commitment_number: u64,
    pub tx: LargeOctets,
    pub psbt: LargeOctets,
    pub wscript: Octets,
}

///
/// CLN only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(13)]
pub struct SignRemoteHtlcToUs {
    pub remote_per_commitment_point: PubKey,
    pub tx: LargeOctets,
    pub psbt: LargeOctets,
    pub wscript: Octets,
    pub option_anchors: bool,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(16)]
pub struct SignLocalHtlcTx {
    pub commitment_number: u64,
    pub tx: LargeOctets,
    pub psbt: LargeOctets,
    pub wscript: Octets,
    pub option_anchors: bool,
}

///
/// CLN only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(21)]
pub struct SignMutualCloseTx {
    pub tx: LargeOctets,
    pub psbt: LargeOctets,
    pub remote_funding_key: PubKey,
}

///
/// LDK only
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(1021)]
pub struct SignMutualCloseTx2 {
    pub to_local_value_sat: u64,
    pub to_remote_value_sat: u64,
    pub local_script: Octets,
    pub remote_script: Octets,
    pub local_wallet_path_hint: Vec<u32>,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(105)]
pub struct SignCommitmentTxReply {
    pub signature: BitcoinSignature,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(112)]
pub struct SignTxReply {
    pub signature: BitcoinSignature,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(30)]
pub struct NewChannel {
    pub node_id: PubKey,
    pub dbid: u64,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(130)]
pub struct NewChannelReply {}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(10)]
pub struct GetChannelBasepoints {
    pub node_id: PubKey,
    pub dbid: u64,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(110)]
pub struct GetChannelBasepointsReply {
    pub basepoints: Basepoints,
    pub funding: PubKey,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(20)]
pub struct SignRemoteHtlcTx {
    pub tx: LargeOctets,
    pub psbt: LargeOctets,
    pub wscript: Octets,
    pub remote_per_commitment_point: PubKey,
    pub option_anchors: bool,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(14)]
pub struct SignPenaltyToUs {
    pub revocation_secret: DisclosedSecret,
    pub tx: LargeOctets,
    pub psbt: LargeOctets,
    pub wscript: Octets,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2002)]
pub struct TipInfo {}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2102)]
pub struct TipInfoReply {
    pub height: u32,
    pub block_hash: BlockHash,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2003)]
pub struct ForwardWatches {}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2103)]
pub struct ForwardWatchesReply {
    pub txids: Vec<TxId>,
    pub outpoints: Vec<OutPoint>,
}

#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2004)]
pub struct ReverseWatches {}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2104)]
pub struct ReverseWatchesReply {
    pub txids: Vec<TxId>,
    pub outpoints: Vec<OutPoint>,
}

#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2005)]
pub struct AddBlock {
    /// Bitcoin consensus encoded
    pub header: Octets,
    /// Bitcoin consensus encoded TXOO UnspentProof
    pub unspent_proof: Option<LargeOctets>,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2105)]
pub struct AddBlockReply {}

#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2006)]
pub struct RemoveBlock {
    /// Bitcoin consensus encoded TXOO UnspentProof
    // FIXME do we need the option?
    pub unspent_proof: Option<LargeOctets>,
}

///
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2106)]
pub struct RemoveBlockReply {}

/// Store key-value pairs to persistent storage - potentially in the cloud.
/// This message may be sent by the *signer* in response to a normal API message,
/// before the actual API response.  The front end should reply with [`PersistReply`]
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2107)]
pub struct Persist {
    /// Authentication token from client (signer) to storage service
    pub auth: Octets,
    pub kvs: Vec<(Octets, u64, LargeOctets)>,
    /// HMAC by client to authenticate the message
    pub hmac: Octets,
}

/// Result of a [`Persist`].
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2007)]
pub struct PersistReply {
    pub success: bool,
    /// HMAC by storage service to authenticate the message
    pub hmac: Octets,
    pub conflicts: Vec<(Octets, u64, LargeOctets)>,
}

/// Get a serialized signed heartbeat
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2008)]
pub struct GetHeartbeat {}

/// A serialized signed heartbeat
#[derive(SerBolt, Debug, Serialize, Deserialize)]
#[message_id(2108)]
pub struct GetHeartbeatReply {
    pub heartbeat: Octets,
}

/// An unknown message
#[derive(Debug, Serialize)]
pub struct Unknown {
    /// Message type
    pub message_type: u16,
    /// Unparsed data
    pub data: Vec<u8>,
}

/// An enum representing all messages we can read and write
#[derive(ReadMessage, Debug, Serialize)]
pub enum Message {
    Ping(Ping),
    Pong(Pong),
    HsmdInit(HsmdInit),
    #[allow(deprecated)]
    HsmdInitReplyV1(HsmdInitReplyV1),
    HsmdInitReplyV2(HsmdInitReplyV2),
    HsmdInit2(HsmdInit2),
    HsmdInit2Reply(HsmdInit2Reply),
    ClientHsmFd(ClientHsmFd),
    ClientHsmFdReply(ClientHsmFdReply),
    SignInvoice(SignInvoice),
    SignInvoiceReply(SignInvoiceReply),
    SignWithdrawal(SignWithdrawal),
    SignWithdrawalReply(SignWithdrawalReply),
    Ecdh(Ecdh),
    EcdhReply(EcdhReply),
    Memleak(Memleak),
    MemleakReply(MemleakReply),
    CheckFutureSecret(CheckFutureSecret),
    CheckFutureSecretReply(CheckFutureSecretReply),
    SignBolt12(SignBolt12),
    SignBolt12Reply(SignBolt12Reply),
    PreapproveInvoice(PreapproveInvoice),
    PreapproveInvoiceReply(PreapproveInvoiceReply),
    PreapproveKeysend(PreapproveKeysend),
    PreapproveKeysendReply(PreapproveKeysendReply),
    DeriveSecret(DeriveSecret),
    DeriveSecretReply(DeriveSecretReply),
    SignMessage(SignMessage),
    SignMessageReply(SignMessageReply),
    SignChannelUpdate(SignChannelUpdate),
    SignChannelUpdateReply(SignChannelUpdateReply),
    SignChannelAnnouncement(SignChannelAnnouncement),
    SignChannelAnnouncementReply(SignChannelAnnouncementReply),
    SignNodeAnnouncement(SignNodeAnnouncement),
    SignNodeAnnouncementReply(SignNodeAnnouncementReply),
    GetPerCommitmentPoint(GetPerCommitmentPoint),
    GetPerCommitmentPointReply(GetPerCommitmentPointReply),
    GetPerCommitmentPoint2(GetPerCommitmentPoint2),
    GetPerCommitmentPoint2Reply(GetPerCommitmentPoint2Reply),
    ReadyChannel(ReadyChannel),
    ReadyChannelReply(ReadyChannelReply),
    ValidateCommitmentTx(ValidateCommitmentTx),
    ValidateCommitmentTx2(ValidateCommitmentTx2),
    ValidateCommitmentTxReply(ValidateCommitmentTxReply),
    ValidateRevocation(ValidateRevocation),
    ValidateRevocationReply(ValidateRevocationReply),
    SignRemoteCommitmentTx(SignRemoteCommitmentTx),
    SignRemoteCommitmentTx2(SignRemoteCommitmentTx2),
    SignCommitmentTxWithHtlcsReply(SignCommitmentTxWithHtlcsReply),
    SignDelayedPaymentToUs(SignDelayedPaymentToUs),
    SignRemoteHtlcToUs(SignRemoteHtlcToUs),
    SignLocalHtlcTx(SignLocalHtlcTx),
    SignCommitmentTx(SignCommitmentTx),
    SignLocalCommitmentTx2(SignLocalCommitmentTx2),
    SignGossipMessage(SignGossipMessage),
    SignMutualCloseTx(SignMutualCloseTx),
    SignMutualCloseTx2(SignMutualCloseTx2),
    SignTxReply(SignTxReply),
    SignCommitmentTxReply(SignCommitmentTxReply),
    GetChannelBasepoints(GetChannelBasepoints),
    GetChannelBasepointsReply(GetChannelBasepointsReply),
    NewChannel(NewChannel),
    NewChannelReply(NewChannelReply),
    SignRemoteHtlcTx(SignRemoteHtlcTx),
    SignPenaltyToUs(SignPenaltyToUs),
    TipInfo(TipInfo),
    TipInfoReply(TipInfoReply),
    ForwardWatches(ForwardWatches),
    ForwardWatchesReply(ForwardWatchesReply),
    ReverseWatches(ReverseWatches),
    ReverseWatchesReply(ReverseWatchesReply),
    AddBlock(AddBlock),
    AddBlockReply(AddBlockReply),
    RemoveBlock(RemoveBlock),
    RemoveBlockReply(RemoveBlockReply),
    Persist(Persist),
    PersistReply(PersistReply),
    GetHeartbeat(GetHeartbeat),
    GetHeartbeatReply(GetHeartbeatReply),
    NodeInfo(NodeInfo),
    NodeInfoReply(NodeInfoReply),
    Unknown(Unknown),
}

fn from_vec_no_trailing<T: DeBolt>(s: &mut Vec<u8>) -> Result<T>
where
    T: de::DeserializeOwned,
{
    let res: T = sb_from_vec(s)?;
    if !s.is_empty() {
        return Err(Error::TrailingBytes(s.len(), T::TYPE));
    }
    Ok(res)
}

/// Read a length framed BOLT message of any type:
///
/// - u32 packet length
/// - u16 packet type
/// - data
pub fn read<R: Read>(reader: &mut R) -> Result<Message> {
    let len = read_u32(reader)?;
    from_reader(reader, len)
}

/// Read a specific message type from a length framed BOLT message:
///
/// - u32 packet length
/// - u16 packet type
/// - data
pub fn read_message<R: Read, T: DeBolt>(reader: &mut R) -> Result<T> {
    T::from_vec(read_raw(reader)?)
}

/// Read a raw message from a length framed BOLT message:
///
/// - u32 packet length (not returned in the result)
/// - u16 packet type
/// - data
pub fn read_raw<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
    let len = read_u32(reader)?;
    let mut data = Vec::new();
    data.resize(len as usize, 0);
    let actual = reader.read(&mut data)?;
    if actual < data.len() {
        return Err(Error::ShortRead);
    }
    Ok(data)
}

/// Read a BOLT message from a vector:
///
/// - u16 packet type
/// - data
pub fn from_vec(mut v: Vec<u8>) -> Result<Message> {
    let len = v.len();
    from_reader(&mut v, len as u32)
}

/// Read a BOLT message from a reader:
///
/// - u16 packet type
/// - data
pub fn from_reader<R: Read>(reader: &mut R, len: u32) -> Result<Message> {
    let (mut data, message_type) = message_and_type_from_reader(reader, len)?;

    Message::read_message(&mut data, message_type)
}

fn message_and_type_from_reader<R: Read>(reader: &mut R, len: u32) -> Result<(Vec<u8>, u16)> {
    let mut data = Vec::new();
    if len < 2 {
        return Err(Error::ShortRead);
    }
    if len > MAX_MESSAGE_SIZE {
        error!("message too large {}", len);
        return Err(Error::MessageTooLarge);
    }
    data.resize(len as usize - 2, 0);
    let message_type = read_u16(reader)?;
    let len = reader.read(&mut data)?;
    if len < data.len() {
        return Err(Error::ShortRead);
    }
    Ok((data, message_type))
}

#[cfg(test)]
fn read_message_and_data<R: Read>(reader: &mut R) -> Result<(Message, Vec<u8>)> {
    let len = read_u32(reader)?;
    let mut data = Vec::new();
    if len < 2 {
        return Err(Error::ShortRead);
    }
    let message_type = read_u16(reader)?;
    data.resize(len as usize - 2, 0);
    let len = reader.read(&mut data)?;
    if len < data.len() {
        return Err(Error::ShortRead);
    }
    let saved_data = data.clone();

    Message::read_message(&mut data, message_type).map(|m| (m, saved_data))
}

pub fn write<W: Write, T: ser::Serialize + DeBolt>(writer: &mut W, value: T) -> Result<()> {
    let message_type = T::TYPE;
    let mut buf = message_type.to_be_bytes().to_vec();
    let mut val_buf = to_vec(&value)?;
    buf.append(&mut val_buf);
    write_vec(writer, buf)
}

pub fn write_vec<W: Write>(writer: &mut W, buf: Vec<u8>) -> Result<()> {
    let len: u32 = buf.len() as u32;
    writer.write_all(&len.to_be_bytes())?;
    writer.write_all(&buf)?;
    Ok(())
}

/// A serial request header
#[derive(Debug)]
pub struct SerialRequestHeader {
    pub sequence: u16,
    pub peer_id: [u8; 33],
    pub dbid: u64,
}

/// Write a serial request header prefixed by two magic bytes
pub fn write_serial_request_header<W: Write>(
    writer: &mut W,
    srh: &SerialRequestHeader,
) -> Result<()> {
    writer.write_all(&0xaa55u16.to_be_bytes())?;
    writer.write_all(&srh.sequence.to_be_bytes())?;
    writer.write_all(&srh.peer_id)?;
    writer.write_all(&srh.dbid.to_be_bytes())?;
    Ok(())
}

/// Write a serial response header that includes two magic bytes and two sequence bytes
pub fn write_serial_response_header<W: Write>(writer: &mut W, sequence: u16) -> Result<()> {
    writer.write_all(&0x5aa5u16.to_be_bytes())?;
    writer.write_all(&sequence.to_be_bytes())?;
    Ok(())
}

/// Read and return the serial request header
/// Returns BadFraming if the magic is wrong.
pub fn read_serial_request_header<R: Read>(reader: &mut R) -> Result<SerialRequestHeader> {
    let magic = read_u16(reader)?;
    if magic != 0xaa55 {
        error!("bad magic {:02x}", magic);
        return Err(Error::BadFraming);
    }
    let sequence = read_u16(reader)?;
    let peer_id = read_bytes(reader)?;
    let dbid = read_u64(reader)?;
    Ok(SerialRequestHeader { sequence, peer_id, dbid })
}

/// Read the serial response header and match the expected sequence number
/// Returns BadFraming if the magic or sequence are wrong.
pub fn read_serial_response_header<R: Read>(reader: &mut R, expected_sequence: u16) -> Result<()> {
    let magic = read_u16(reader)?;
    if magic != 0x5aa5u16 {
        error!("bad magic {:02x}", magic);
        return Err(Error::BadFraming);
    }
    let sequence = read_u16(reader)?;
    if sequence != expected_sequence {
        error!("sequence {} != expected {}", sequence, expected_sequence);
        return Err(Error::BadFraming);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use regex::Regex;

    use crate::msgs::Message;

    use super::*;

    #[test]
    fn roundtrip_test() {
        let msg = SignChannelAnnouncementReply {
            node_signature: Signature([0; 64]),
            bitcoin_signature: Signature([1; 64]),
        };

        let ser = msg.as_vec();
        let dmsg = from_vec(ser).unwrap();
        if let Message::SignChannelAnnouncementReply(dmsg) = dmsg {
            assert_eq!(dmsg.node_signature.0, msg.node_signature.0);
            assert_eq!(dmsg.bitcoin_signature.0, msg.bitcoin_signature.0);
        } else {
            panic!("bad deser type")
        }
    }

    // Test the persist message, since it has more more nesting than others
    #[test]
    fn persist_test() {
        let key = Octets(b"foo".to_vec());
        let value = b"bar".to_vec();
        let version = 0x123456789;
        let msg = Persist {
            auth: Octets(vec![0x11, 0x22]),
            kvs: vec![(Octets(key.clone()), version, LargeOctets(value.clone()))],
            hmac: Octets(vec![0x33, 0x44]),
        };

        let ser = msg.as_vec();
        let dmsg = from_vec(ser).unwrap();
        if let Message::Persist(dmsg) = dmsg {
            assert_eq!(dmsg.auth, msg.auth);
            for (k, ver, v) in dmsg.kvs.into_iter() {
                assert_eq!(k, key);
                assert_eq!(ver, version);
                assert_eq!(v.0, value);
            }
            assert_eq!(dmsg.hmac, msg.hmac);
        } else {
            panic!("bad deser type")
        }
    }

    // ignore tests for now, the trace capture was not on the lightning-signer branch
    #[test]
    #[ignore]
    fn parse_read_fixtures_test() {
        assert_eq!(parse_fixture("r_3"), 16);
        assert_eq!(parse_fixture("r_5"), 1);
        assert_eq!(parse_fixture("r_6"), 39);
    }

    // ignore tests for now, the trace capture was not on the lightning-signer branch
    #[test]
    #[ignore]
    fn parse_write_fixtures_test() {
        // TODO negative message type IDs?
        // assert_eq!(parse_fixture("w_0"), 16);
        assert_eq!(parse_fixture("w_3"), 16);
        assert_eq!(parse_fixture("w_4"), 1);
        assert_eq!(parse_fixture("w_5"), 1);
        assert_eq!(parse_fixture("w_6"), 52);
    }

    fn parse_fixture(fixture: &str) -> u32 {
        println!("processing {}", fixture);
        let contents_with_whitespace =
            fs::read_to_string(format!("fixtures/{}.hex", fixture)).unwrap();
        let contents_hex = Regex::new(r"\s").unwrap().replace_all(&contents_with_whitespace, "");
        let mut contents = hex::decode(&*contents_hex).unwrap();
        let mut num_read = 0;
        loop {
            let res = read_message_and_data(&mut contents);
            match res {
                Ok((Message::Unknown(u), _)) => {
                    panic!("unknown {} {}", u.message_type, u.data.len());
                }
                Ok((msg, data)) => {
                    println!("read {:x?}", msg);
                    let encoded = to_vec(&msg).expect("encoding");
                    assert_eq!(encoded, data);
                }
                Err(Error::Eof) => {
                    println!("done");
                    break;
                }
                Err(e) => {
                    panic!("unexpected error {:?}", e);
                }
            }
            num_read += 1;
        }
        num_read
    }
}
