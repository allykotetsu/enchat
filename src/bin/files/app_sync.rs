pub const GET: &str = r#"let latest = 0;

onmessage = async event => {
    let running = true;
    while(running) {
        await sync(event.data);
    }
};

async function sync(signature) {
    const response = await fetch(`/api/sync?ts=${latest}`, {
        method: 'GET',
        headers: {
            'Accept': 'application/json',
            'Authorization': 'Bearer ' + signature
        }
    });

    if(response.status == 200) {
        let json = await response.json();
        if(json.keys.length > 0) {
            for(const key of json.keys) {
                postMessage(["keys", json.keys]);
            }
        }
        if(json.messages.length > 0) {
            for(const item of json.messages) {
                if(item.timestamp > latest) {
                    latest = item.timestamp;
                }
            }
            postMessage(["messages", json.messages]);
        }
    } else if(response.status == 401) {
        postMessage(401);
    }
}"#;