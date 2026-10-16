console.log("Hello world");

const ws = new WebSocket('ws://127.0.0.1:8080');

ws.onopen = () => {
    console.log("WebSocket connected");
};

ws.onerror = (error) => {
    console.error("WebSocket Error:", error);
};

ws.onclose = (event) => {
    console.log("Closed with code:", event.code, "Reason:", event.reason);
};
