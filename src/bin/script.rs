pub const GET: &str = r#""use strict";

window.onload = async function() {
    const signature = localStorage.getItem("signature");
    if (signature) {
        document.location.href = "/app";
    }
};

async function newSession() {
    const keyPair = await window.crypto.subtle.generateKey("Ed25519", true, ["sign", "verify"]);

    const exportedPriv = await window.crypto.subtle.exportKey("pkcs8", keyPair.privateKey);
    const exportedAsBase64Priv = window.btoa(String.fromCharCode.apply(null, new Uint8Array(exportedPriv)));
    const privkey = `-----BEGIN PRIVATE KEY-----\n${exportedAsBase64Priv}\n-----END PRIVATE KEY-----`;

    const exportedPub = await window.crypto.subtle.exportKey("spki", keyPair.publicKey);
    const exportedAsBase64Pub = window.btoa(String.fromCharCode.apply(null, new Uint8Array(exportedPub)));
    const encoded = encodeURIComponent(exportedAsBase64Pub);

    const keyPairRsa = await window.crypto.subtle.generateKey({
        name: "RSA-OAEP",
        modulusLength: 4096,
        publicExponent: new Uint8Array([1, 0, 1]),
        hash: "SHA-256"
    }, true, ["wrapKey", "unwrapKey"]);

    const exportedPrivRsa = await window.crypto.subtle.exportKey("pkcs8", keyPairRsa.privateKey);
    const privkeyRsa = window.btoa(String.fromCharCode.apply(null, new Uint8Array(exportedPrivRsa)));

    const exportedPubRsa = await window.crypto.subtle.exportKey("spki", keyPairRsa.publicKey);
    const exportedAsBase64PubRsa = window.btoa(String.fromCharCode.apply(null, new Uint8Array(exportedPubRsa)));

    let formData = new FormData();
    formData.append("item", exportedAsBase64PubRsa);

    const response = await fetch(`/api/session/${encoded}`, {
        method: 'POST',
        headers: {
            'Accept': 'application/x-www-form-urlencoded',
            'Content-type': 'application/x-www-form-urlencoded'
        },
        body: new URLSearchParams(formData)
    });

    if(response.status == 201) {
        let challenge = (await response.formData()).get("item");
        let signature = await window.crypto.subtle.sign("Ed25519", keyPair.privateKey, new TextEncoder().encode(challenge));

        localStorage.setItem("pubkey", exportedAsBase64Pub);
        localStorage.setItem("privkey", privkey);
        localStorage.setItem("signature", window.btoa(String.fromCharCode.apply(null, new Uint8Array(signature))));
        document.cookie = `challenge=${challenge}`;
        localStorage.setItem("pubkeyRsa", exportedAsBase64PubRsa);
        localStorage.setItem("privkeyRsa", privkeyRsa);

        if(location.hash) {
            const code = location.hash.substring(1);
            document.location.href = `/invite#${code}`;
        } else {
            document.location.href = "/app";
        }
    }
}"#;