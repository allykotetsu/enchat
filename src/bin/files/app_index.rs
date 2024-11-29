pub const GET: &str = r#"<!doctype HTML>
<html>
    <head>
        <title>Enchat</title>
        <link rel="stylesheet" href="/style.css"/>
        <script src="/app/script.js"></script>
    </head>
    <body>
        <header>
            <h1><b>Enchat</b></h1>
            <h1 id="id">Session ID:</h1>
            <div>
                <button id="lock" onclick="lock()">Lock</button>
                <button id="logout" onclick="logout()">Delete Session</button>
            </div>
        </header>
        <main>
            <nav id="channels">
                <div id="heading">
                    <h2>Channels</h2>
                    <button onclick="newChannel()" id="newchannel">New</button>
                </div>
            </nav>
            <div id="messenger">
                <div id="info">
                    <span id="name"></span>
                    <div>
                        <button id="invite" onclick="invite()">Invite</button>
                        <button id="leave" onclick="leave()">Leave</button>
                    </div>
                </div>
                <div id="messages"></div>
                <div id="composer">
                    <input placeholder="Send an encrypted message." id="content"/>
                    <button onclick="send()" id="send">Send</button>
                </div>
            </div>
        </main>
        <div id="popup"></div>
        <div id="loading">
            <h1>Loading...</h1>
        </div>
    </body>
</html>"#;