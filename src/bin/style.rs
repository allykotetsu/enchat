pub const GET: &str = r#"@keyframes glow {
    0% {
        color: rgb(127, 127, 127)
    }
    50% {
        color: rgb(255, 255, 255)
    }
    100% {
        color: rgb(127, 127, 127)
    }
}
html body {
    background-color: rgb(23, 23, 23);
    margin: 0;
    position: fixed;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
}
html body header {
    background-color: rgb(15, 15, 15);
    padding: 0.5em;
    display: flex;
    justify-content: space-between;
    font-family: Verdana, sans-serif;
    box-shadow: 0 0 1em rgb(0, 0, 0, 0.5);
    z-index: 10;
}
html body header h1 {
    color: white;
    margin: 0;
    font-size: 1em;
    font-weight: normal;
}
html body header div {
    cursor: default;
}
html body header div button {
    background-color: transparent;
    border: none;
    font-weight: bold;
    cursor: pointer;
    transition: all ease-in-out 100ms;
    margin: 0.1em;
}
html body header div button#lock {
    color: gray;
}
html body header div button#lock:hover {
    color: lightgray;
}
html body header div button#logout {
    color: red;
}
html body header div button#logout:hover {
    color: pink;
}
html body main {
    display: flex;
    flex-direction: row;
    width: 100%;
    flex: 1;
    overflow: hidden;
}
html body main button#new {
    background-color: rgb(31, 31, 31);
    color: white;
    font-size: 4em;
    padding: 1em;
    margin: auto;
    border-radius: 2em;
    border: white 0.065em solid;
    cursor: pointer;
    box-shadow: 0 0 0.065em rgba(255,255,255,0.5);
    text-shadow: 0 0 0.065em rgba(255,255,255,0.5);
    transition: all ease-in-out 100ms;
    font-family: monospace;
}
html body main button#new:hover {
    background-color: rgb(39, 39, 39);
    box-shadow: 0 0 0.125em rgba(255,255,255,0.5);
    text-shadow: 0 0 0.125em rgba(255,255,255,0.5);
}
html body main button#new:active {
    background-color: rgb(47, 47, 47);
}
html body main nav {
    padding: 1em 0 1em 1em;
    width: 20em;
    overflow-y: scroll;
}
html body main nav div#heading {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    margin: 0 1em 0.5em 0;
}
html body main nav h2 {
    color: white;
    font-size: 1em;
    margin: 0;
    font-family: sans-serif;
    padding: 0.75em;
    cursor: default;
}
html body main nav button {
    font-size: 1em;
    display: flex;
    flex-direction: column;
    border: none;
    border-radius: 1em 0 0 1em;
    box-shadow: none;
    font-family: sans-serif;
    color: white;
    background-color: rgb(23, 23, 23);
    padding: 0.75em;
    cursor: pointer;
    text-align: left;
    width: 100%;
}
html body main nav button:hover {
    background-color: rgb(39, 39, 39);
}
html body main nav button:active, html body main nav button.selected {
    background: rgb(31, 31, 31);
}
html body main nav button#newchannel {
    font-weight: bold;
    background-color: rgb(39, 39, 39);
    width: auto;
    border-radius: 1em;
    text-align: center;
}
html body main div#messenger {
    background-color: rgb(31, 31, 31);
    width: 100%;
    display: flex;
    justify-content: space-between;
    flex-direction: column;
}
html body main div#messenger div#info {
    background-color: rgb(47, 47, 47);
    padding: 1.5em;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
}
html body main div#messenger div#info span#name {
    font-family: sans-serif;
    font-weight: bold;
    color: white;
}
html body main div#messenger div#info div button {
    cursor: default;
}
html body main div#messenger div#info div button {
    font-family: sans-serif;
    font-weight: bold;
    background-color: transparent;
    border: none;
    cursor: pointer;
    transition: all ease-in-out 100ms;
}
html body main div#messenger div#info div button#invite {
    color: gray;
}
html body main div#messenger div#info div button#invite:hover {
    color: lightgray;
}
html body main div#messenger div#info div button#leave {
    color: red;
}
html body main div#messenger div#info div button#leave:hover {
    color: pink;
}
html body main div#messenger div#messages {
    padding: 1em;
    flex: 1;
    overflow-y: scroll;
    align-content: end;
}
html body main div#messenger div#messages article {
    color: white;
    font-family: sans-serif;
    padding: 0.5em;
    border-radius: 1em;
    transition: all ease-in-out 100ms;
}
html body main div#messenger div#messages article:hover {
    background-color: rgb(47, 47, 47);
}
html body main div#messenger div#messages article h3 {
    font-size: 1em;
    font-weight: bold;
    margin: 0;
}
html body main div#messenger div#messages article span {
    font-size: 1em;
}
html body main div#messenger div#composer {
    padding: 1em;
    display: flex;
}
html body main div#messenger div#composer input#content {
    background-color: rgb(47, 47, 47);
    color: white;
    padding: 1em;
    border: none;
    border-radius: 1em;
    font-size: 1em;
    transition: all ease-in-out 100ms;
    flex: 1;
}
html body main div#messenger div#composer input#content:hover {
    background-color: rgb(55, 55, 55);
}
html body main div#messenger div#composer input#content[disabled] {
    background-color: rgb(39, 39, 39);
    color: rgb(127, 127, 127);
    cursor: not-allowed;
}
html body main div#messenger div#composer button#send {
    background-color: rgb(47, 47, 47);
    color: white;
    padding: 1em;
    border: none;
    border-radius: 1em;
    font-size: 1em;
    cursor: pointer;
    transition: all ease-in-out 100ms;
    font-weight: bold;
    margin-left: 0.5em;
}
html body main div#messenger div#composer button#send:hover {
    background-color: rgb(55, 55, 55);
}
html body main div#messenger div#composer button#send:active {
    background-color: rgb(63, 63, 63);
}
html body main div#messenger div#composer button#send[disabled] {
    background-color: rgb(39, 39, 39);
    color: rgb(127, 127, 127);
    cursor: not-allowed;
}
html body div#popup {
    position: fixed;
    margin: auto;
    z-index: 20;
    width: fit-content;
    top: 4em;
    display: none;
    text-align: center;
    color: white;
    background-color: rgb(47, 47, 47);
    padding: 1em;
    border-radius: 1em;
    box-shadow: 0 0 1em rgb(0, 0, 0, 0.5);
    font-family: sans-serif;
    left: 0;
    right: 0;
    opacity: 100%;
    transition: opacity 1000ms ease-in-out
}
html body div#popup.fade {
    opacity: 0;
}
html body div#loading {
    position: fixed;
    background-color: rgb(23, 23, 23);
    margin: 0;
    width: 100%;
    height: 100%;
    padding-top: 25%;
    z-index: 30;
}
html body div#loading h1 {
    text-align: center;
    margin: auto;
    line-height: 0;
    color: white;
    font-family: monospace;
    animation: glow ease-in-out 1s infinite;
    text-shadow: 0 0 0.065em rgba(255,255,255,0.5);
}
.semi {
    color: rgb(127, 127, 127);
}"#;
