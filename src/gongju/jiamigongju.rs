use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use x25519_dalek::{PublicKey, StaticSecret};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as base64bianma;

#[allow(non_upper_case_globals)]
const nonce_changdu: usize = 12;
#[allow(non_upper_case_globals)]
const miyao_changdu: usize = 32;
#[allow(non_upper_case_globals)]
const gongyao_changdu: usize = 32;
#[allow(non_upper_case_globals)]
const hkdf_xinxi: &[u8] = b"jiamichuanshu-aes256gcm";
#[allow(non_upper_case_globals)]
pub const yanfen: &[u8] = b"jiamichuanshu-yanfen";

type Hmacsha256 = Hmac<Sha256>;

/// 生成 X25519 临时密钥对，返回 (私钥字节, 公钥字节)
pub fn shengchengyaodui() -> (Vec<u8>, Vec<u8>) {
    let siyao_zijie: [u8; 32] = rand::random();
    let siyao = StaticSecret::from(siyao_zijie);
    let gongyao = PublicKey::from(&siyao);
    (siyao.to_bytes().to_vec(), gongyao.to_bytes().to_vec())
}

/// ECDH 协商共享密钥
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

/// HKDF-SHA256 派生 AES-256 密钥
pub fn paishengyao(gongxiangyao: &[u8], yanfenzhi: &[u8]) -> Option<Vec<u8>> {
    let mut tiqu_mac = <Hmacsha256 as Mac>::new_from_slice(yanfenzhi).ok()?;
    tiqu_mac.update(gongxiangyao);
    let prk = tiqu_mac.finalize().into_bytes();
    hkdf_kuozhan(&prk, hkdf_xinxi)
}

/// AES-256-GCM 加密，返回 nonce + 密文
pub fn jiami(mingwen: &[u8], miyao: &[u8]) -> Option<Vec<u8>> {
    let miyao_shuzhu: [u8; miyao_changdu] = miyao.try_into().ok()?;
    let miqi = Aes256Gcm::new_from_slice(&miyao_shuzhu).ok()?;
    let suiji_nonce: [u8; nonce_changdu] = rand::random();
    let nonce = Nonce::from_slice(&suiji_nonce);
    let miwen = miqi.encrypt(nonce, mingwen).ok()?;
    let mut jieguo = Vec::with_capacity(nonce_changdu + miwen.len());
    jieguo.extend_from_slice(&suiji_nonce);
    jieguo.extend_from_slice(&miwen);
    Some(jieguo)
}

/// AES-256-GCM 解密，输入为 nonce + 密文
pub fn jiemi(miwen: &[u8], miyao: &[u8]) -> Option<Vec<u8>> {
    (miwen.len() > nonce_changdu).then_some(())?;
    let miyao_shuzhu: [u8; miyao_changdu] = miyao.try_into().ok()?;
    let miqi = Aes256Gcm::new_from_slice(&miyao_shuzhu).ok()?;
    let nonce = Nonce::from_slice(&miwen[..nonce_changdu]);
    miqi.decrypt(nonce, &miwen[nonce_changdu..]).ok()
}

/// 字节数组转 base64 编码
pub fn zhuanbase64(shuju: &[u8]) -> String {
    base64bianma.encode(shuju)
}

/// base64 解码为字节数组
pub fn congbase64(base64str: &str) -> Option<Vec<u8>> {
    base64bianma.decode(base64str).ok()
}
