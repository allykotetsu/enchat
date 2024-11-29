pub const GET: &str = r#""use strict";

// TODO if # in url then redirect to channel

let messages = [];
let _current_channel = "";
let _privkeyRsa;
let _pubkey;
let _signature;

// TODO replace
function str2ab(str) {
    const buf = new ArrayBuffer(str.length);
    const bufView = new Uint8Array(buf);
    for (let i = 0, strLen = str.length; i < strLen; i++) {
        bufView[i] = str.charCodeAt(i);
    }
    return buf;
}

function clear() {
    localStorage.clear();
    //cookieStore.delete("challenge");
    document.location.href = "/";
}

window.onload = async function() {
    _signature = localStorage.getItem("signature");

    if(_signature) {
        _pubkey = localStorage.getItem("pubkey");

        if(localStorage.getItem("locked") == "true") {
            document.getElementById("loading").innerHTML = "<h1>Locked</h1>";
            const pwd = prompt("This session is currently locked. Please enter the password you used to lock it to decrypt the info and unlock it.");
            // TODO undo everything done in lock()
        } else {
            _privkeyRsa = await window.crypto.subtle.importKey("pkcs8", str2ab(window.atob(localStorage.getItem("privkeyRsa"))), {
                name: "RSA-OAEP",
                hash: "SHA-256"
            }, true, ["unwrapKey"]);

            await loadChannels();

            const sync = new Worker("/app/sync.js");
            sync.postMessage(_signature);

            sync.onmessage = event => {
                switch(event.data[0]) {
                    case "messages": {
                        for(const item of event.data[1]) {
                            item.encrypted = true;
                            messages.push(item);
                            if(item.channel_id == _current_channel) {
                                loadMessage(item);
                            }
                        }
                        break;
                    }
                    case "keys": {
                        for(const item of event.data[1]) {
                            loadKey(item);
                        }
                        break;
                    }
                }
            };

            document.addEventListener('keydown', e => {
                e = e || window.event;
                const key = e.which || e.keyCode;

                if(document.activeElement == document.getElementById("content") && key == 13) {
                    send();
                }
            });

            document.getElementById("id").innerHTML = `Session ID: <b>${_pubkey}</b>`;
            document.getElementById("loading").style.display = "none";
        }
    } else {
        clear();
    }
};

async function loadKey(key) {
    const iv = key.iv;
    const wrappedAsAB = str2ab(window.atob(key.key));

    const unwrapKey = _privkeyRsa;
    const unwrapped = await window.crypto.subtle.unwrapKey("raw", wrappedAsAB, unwrapKey, {
        name: "RSA-OAEP",
        hash: "SHA-256"
    }, {
        name: "AES-GCM", iv: iv
    }, true, ["encrypt", "decrypt"]);

    const exported = window.btoa(String.fromCharCode.apply(null, new Uint8Array(await window.crypto.subtle.exportKey("raw", unwrapped))));
    localStorage.setItem(iv, exported);

    for(let message of messages) {
        if(message.iv = iv) {
            decryptMessage(message);
        }
    }
}

async function logout() {
    const pubkey = encodeURIComponent(_pubkey);

    const response = await fetch(`/api/session/${pubkey}`, {
        method: 'DELETE',
        headers: {
            'Accept': 'application/x-www-form-urlencoded',
            'Authorization': 'Bearer ' + _signature
        }
    });

    if(response.status == 200) {
        clear();
    } else {
        alert("There was a problem deleting this session");
    }
}

async function lock() {
    // TODO this isn't working yet

    const pwd = prompt("Locking your account will temporarily encrypt all of the stored data with a password, plesae enter it here:");
    localStorage.setItem("locked", true);

    const ikey = await window.crypto.subtle.importKey("raw", new TextEncoder().encode(pwd), { name: "PBKDF2" }, false, ["deriveKey"]);
    const salt = window.crypto.getRandomValues(new Uint8Array(16));
    const key = await window.crypto.subtle.deriveKey({
        name: "PBKDF2",
        salt,
        iterations: 600000,
        hash: "SHA-256"
    }, ikey, { name: "AES-GCM", length: 256 }, true, ["encrypt", "decrypt"]);

    const pub_iv = window.crypto.getRandomValues(new Uint8Array(12));
    localStorage.setItem("pub_iv", pub_iv);
    const priv_iv = window.crypto.getRandomValues(new Uint8Array(12));
    localStorage.setItem("priv_iv", priv_iv);
    const sig_iv = window.crypto.getRandomValues(new Uint8Array(12));
    localStorage.setItem("sig_iv", sig_iv);

    const pub_enc = await window.crypto.subtle.encrypt(
        { name: "AES-GCM", iv: pub_iv },
        key,
        new TextEncoder().encode(_pubkey)
    );
    const priv_enc = await window.crypto.subtle.encrypt(
        { name: "AES-GCM", iv: priv_iv },
        key,
        new TextEncoder().encode(localStorage.getItem("privkey"))
    );
    const sig_enc = await window.crypto.subtle.encrypt(
        { name: "AES-GCM", iv: sig_iv },
        key,
        new TextEncoder().encode(_signature)
    );

    localStorage.setItem("pubkey", new TextEncoder().encode(pub_enc));
    localStorage.setItem("privkey", new TextEncoder().encode(priv_enc));
    localStorage.setItem("signature", new TextEncoder().encode(sig_enc));
}

async function newChannel() {
    const name = prompt("What would you like the channel name to be?");
    const nick = prompt("What would you like your nickname to be?");

    let formData = new FormData();
    formData.append('name', name);
    formData.append('nick', nick);

    const response = await fetch(`/api/channels`, {
        method: 'POST',
        headers: {
            'Accept': 'application/x-www-form-urlencoded',
            'Content-type': 'application/x-www-form-urlencoded',
            'Authorization': 'Bearer ' + _signature
        },
        body: new URLSearchParams(formData)
    });

    if(response.status == 201) {
        const id = (await response.formData()).get("item");
        await loadChannels();
        switchChannel(id, name);
    }
}

async function switchChannel(id, name) {
    if(_current_channel) {
        document.getElementById(_current_channel).classList.remove("selected");
    }
    document.getElementById(id).classList.add("selected");
    _current_channel = id;

    document.getElementById("name").innerText = name;
    document.getElementById("messages").innerHTML = '';
    document.getElementById("content").disabled = false;
    document.getElementById("content").focus();
    document.getElementById("send").disabled = false;

    for(let message of messages.filter(m => m.channel_id == id).sort((a, b) => a.timestamp - b.timestamp)) {
        await loadMessage(message);
    }
}

async function decryptMessage(message) {
    let key = localStorage.getItem(message.iv);

    if(key) {
        const imported = await window.crypto.subtle.importKey("raw", str2ab(window.atob(key)), "AES-GCM", true, ["encrypt", "decrypt"]);

        message.content = new TextDecoder("utf-8").decode(await window.crypto.subtle.decrypt({
            name: "AES-GCM", iv: str2ab(window.atob(message.iv))
        }, imported, str2ab(window.atob(message.content))));
        message.encrypted = false;

        document.getElementById(message.iv).innerText = message.content;
    }
}

async function loadMessage(message) {
    let bottom = document.getElementById("messages").scrollTop == document.getElementById("messages").scrollHeight;
    bottom = true;

    if(message.encrypted) {
        let key = localStorage.getItem(message.iv);

        if(key) {
            const imported = await window.crypto.subtle.importKey("raw", str2ab(window.atob(key)), "AES-GCM", true, ["encrypt", "decrypt"]);

            message.content = new TextDecoder("utf-8").decode(await window.crypto.subtle.decrypt({
                name: "AES-GCM", iv: str2ab(window.atob(message.iv))
            }, imported, str2ab(window.atob(message.content))));
            message.encrypted = false;

            document.getElementById("messages").innerHTML += `<article>
                <h3>${message.sender}</h3>
                <span id=${message.iv}>${message.content}</span>
            </article>`;
        } else {
            document.getElementById("messages").innerHTML += `<article>
                <h3>${message.sender}</h3>
                <span id=${message.iv}><i class="semi">Decrypting...</i></span>
            </article>`;
        }
    } else {
        document.getElementById("messages").innerHTML += `<article>
            <h3>${message.sender}</h3>
            <span id=${message.iv}>${message.content}</span>
        </article>`;
    }

    if(bottom) {
        document.getElementById("messages").scrollTop = document.getElementById("messages").scrollHeight;
    }
}

async function loadChannels() {
    const response = await fetch(`/api/channels`, {
        method: 'GET',
        headers: {
            'Accept': 'application/x-www-form-urlencoded',
            'Authorization': 'Bearer ' + _signature
        }
    });

    if(response.status == 200) {
        let json = await response.json();
        let channels = document.getElementById("channels");
        channels.innerHTML = `<div id="heading">
            <h2>Channels</h2>
            <button onclick="newChannel()" id="newchannel">New</button>
        </div>`;
        if(json.item.length > 0) {
            for(const channel of json.item) {
                channels.innerHTML += `<button id="${channel.id}" onclick="switchChannel('${channel.id}', '${channel.name}')">${channel.name}</button>`;
            }
            switchChannel(json.item[0].id, json.item[0].name);
        } else {
            document.getElementById("name").innerText = "Nowhere";
            document.getElementById("messages").innerHTML = `<article>
                <h3>System</h3>
                <span>Welcome to Enchat! To get started, either create a new room, or join an existing room by invite link.</span>
            </article>`;
            document.getElementById("content").disabled = true;
            document.getElementById("send").disabled = true;
        }
    } else if(response.status == 401) {
        clear();
    }
}

async function send() {
    let content = document.getElementById("content").value;

    if(content) {
        const key = await window.crypto.subtle.generateKey({
            name: "AES-GCM",
            length: 256
        }, true, ["encrypt", "decrypt"]);

        let iv = window.crypto.getRandomValues(new Uint8Array(12));
        content = await window.crypto.subtle.encrypt({
            name: "AES-GCM", iv: iv
        }, key, new TextEncoder().encode(content));
        iv = window.btoa(String.fromCharCode.apply(null, iv));
        content = window.btoa(String.fromCharCode.apply(null, new Uint8Array(content)));

        const exported = window.btoa(String.fromCharCode.apply(null, new Uint8Array(await window.crypto.subtle.exportKey("raw", key))));

        let formData = new FormData();
        formData.append('channel_id', _current_channel);
        formData.append('content', content);
        formData.append('iv', iv);

        localStorage.setItem(iv, exported);

        const response = await fetch(`/api/message`, {
            method: 'POST',
            headers: {
                'Accept': 'application/x-www-form-urlencoded',
                'Authorization': 'Bearer ' + _signature,
                'Content-type': 'application/x-www-form-urlencoded'
            },
            body: new URLSearchParams(formData)
        });
        document.getElementById("content").value = '';
        document.getElementById("messages").scrollTop = document.getElementById("messages").scrollHeight;

        const pubkeys = (await response.formData()).getAll("item");
        for(const pubkey of pubkeys) {
            const wrapKey = await window.crypto.subtle.importKey("spki", str2ab(window.atob(pubkey)), {
                name: "RSA-OAEP",
                hash: "SHA-256"
            }, true, ["wrapKey"]);

            let wrapped = await window.crypto.subtle.wrapKey("raw", key, wrapKey, {
                name: "RSA-OAEP",
                hash: "SHA-256"
            });

            let formData2 = new FormData();
            formData2.append("rsa_pubkey", pubkey);
            formData2.append("key", window.btoa(String.fromCharCode.apply(null, new Uint8Array(wrapped))));
            formData2.append("iv", iv);

            const response2 = await fetch(`/api/key`, {
                method: 'POST',
                headers: {
                    'Accept': 'application/x-www-form-urlencoded',
                    'Authorization': 'Bearer ' + _signature,
                    'Content-type': 'application/x-www-form-urlencoded'
                },
                body: new URLSearchParams(formData2)
            });
        }
    }
}

async function invite() {
    await navigator.clipboard.writeText(`https://enchat.portfolio.allykotetsu.com/invite#${_current_channel}`);
    popup("Invite link copied to clipboard.");
}

function popup(text) {
    let pop = document.getElementById("popup");
    pop.style.display = "block"
    pop.innerText = text;
    setTimeout(() => {
        pop.classList.add("fade");
    }, 2000);
    setTimeout(() => {
        pop.classList.remove("fade");
        pop.style.display = "none";
    }, 3000);
}"#;