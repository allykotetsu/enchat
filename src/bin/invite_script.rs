pub const GET: &str = r#""use strict";

window.onload = async function() {
    const pubkey = localStorage.getItem("pubkey");
    const code = location.hash.substring(1);
    if(pubkey) {
        const nick = prompt("What would you like your nickname to be?");
        let formData = new FormData();
        formData.append("channel", code);
        formData.append("nick", nick);

        const response = await fetch(`/api/join`, {
            method: 'POST',
            headers: {
                'Content-type': 'application/x-www-form-urlencoded',
                'Authorization': 'Bearer ' + localStorage.getItem("signature")
            },
            body: new URLSearchParams(formData)
        });

        if(response.status == 201) {
            document.location.href = `/app#${code}`;
        } else {
            //document.location.href = "/";
        }
    } else {
        document.location.href = `/#${code}`;
    }
}"#;