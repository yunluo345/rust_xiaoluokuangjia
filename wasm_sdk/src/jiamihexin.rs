use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use x25519_dalek::{PublicKey, StaticSecret};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64bianma;

const nonce_changdu: usize = 12;
const miyao_changdu: usize = 32;
const gongyao_changdu: usize = 32;
const hkdf_xinxi: &[u8] = b"jiamichuanshu-aes256gcm";
pub const yanfen: &[u8] = b"jiamichuanshu-yanfen";

type Hmacsha256 = Hmac<Sha256>;

pub fn shengchengyaodui() -> (Vec<u8>, Vec<u8>) {
    let mut zijie = [0u8; 32];
    getrandom::getrandom(&mut zijie).unwrap();
    let siyao = StaticSecret::from(zijie);
    let gongyao = PublicKey::from(&siyao);
    (siyao.to_bytes().to_vec(), gongyao.to_bytes().to_vec())
}

pub fn xieshanggongxiangyao(benfangsiyao: &[u8], duifanggongyao: &[u8]) -> Option<Vec<u8>> {
    let siyao_shuzhu: [u8; miyao_changdu] = benfangsiyao.try_into().ok()?;
    let gongyao_shuzhu: [u8; gongyao_changdu] = duifanggongyao.try_into().ok()?;
    let siyao = StaticSecret::from(siyao_shuzhu);
    let gongyao = PublicKey::from(gongyao_shuzhu);
    Some(siyao.diffie_hellman(&gongyao).as_bytes().to_vec())
}

fn hkdf_kuozhan(prk: &[u8], xinxi: &[u8]) -> Option<Vec<u8>> {
    let mut mac = <Hmacsha256 as Mac>::new_from_slice(prk).ok()?;
    mac.update(xinxi);
    mac.update(&[1u8]);
    Some(mac.finalize().into_bytes().to_vec())
}

pub fn paishengyao(gongxiangyao: &[u8], yanfenzhi: &[u8]) -> Option<Vec<u8>> {
    let mut tiqu_mac = <Hmacsha256 as Mac>::new_from_slice(yanfenzhi).ok()?;
    tiqu_mac.update(gongxiangyao);
    let prk = tiqu_mac.finalize().into_bytes();
    hkdf_kuozhan(&prk, hkdf_xinxi)
}

pub fn jiami(mingwen: &[u8], miyao: &[u8]) -> Option<Vec<u8>> {
    let miyao_shuzhu: [u8; miyao_changdu] = miyao.try_into().ok()?;
    let miqi = Aes256Gcm::new_from_slice(&miyao_shuzhu).ok()?;
    let mut suiji_nonce = [0u8; nonce_changdu];
    getrandom::getrandom(&mut suiji_nonce).ok()?;
    let nonce = Nonce::from_slice(&suiji_nonce);
    let miwen = miqi.encrypt(nonce, mingwen).ok()?;
    let mut jieguo = Vec::with_capacity(nonce_changdu + miwen.len());
    jieguo.extend_from_slice(&suiji_nonce);
    jieguo.extend_from_slice(&miwen);
    Some(jieguo)
}

pub fn jiemi(miwen: &[u8], miyao: &[u8]) -> Option<Vec<u8>> {
    (miwen.len() > nonce_changdu).then_some(())?;
    let miyao_shuzhu: [u8; miyao_changdu] = miyao.try_into().ok()?;
    let miqi = Aes256Gcm::new_from_slice(&miyao_shuzhu).ok()?;
    let nonce = Nonce::from_slice(&miwen[..nonce_changdu]);
    miqi.decrypt(nonce, &miwen[nonce_changdu..]).ok()
}

pub fn zhuanbase64(shuju: &[u8]) -> String {
    base64bianma.encode(shuju)
}

pub fn congbase64(base64str: &str) -> Option<Vec<u8>> {
    base64bianma.decode(base64str).ok()
}
