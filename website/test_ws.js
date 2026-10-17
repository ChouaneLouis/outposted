const ws = new WebSocket('ws://127.0.0.1:8080');
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

function resizeCanvas() {
    const rect = canvas.getBoundingClientRect();
    
    const dpr = window.devicePixelRatio || 1;
    
    canvas.width = rect.width * dpr;
    canvas.height = rect.height * dpr;
}

window.addEventListener('resize', resizeCanvas);

resizeCanvas();


const scale = 2.0;
function world_to_canvas_x(x) {
    return x * scale + canvas.width * 0.5;
}

function world_to_canvas_y(y) {
    return y * scale + canvas.height * 0.5;
}

function handleGameObject(data) {
    ctx.beginPath();
    ctx.arc(world_to_canvas_x(data.x_pos), world_to_canvas_y(data.y_pos), data.radius * scale, 0, 2 * Math.PI);
    ctx.stroke();
    console.log("circle drawn")
}

function matchData (data) {
    switch (data.type) {
        case "GameObject":
            handleGameObject(data);
            break;
        default:
            console.warn("Unrecognized data type:", data);
    }
}

ws.onopen = () => {
    console.log("WebSocket connected");
};

ws.onerror = (error) => {
    console.error("WebSocket Error:", error);
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log("Message received:", data);

    if (Array.isArray(data)) {
        for (const d of data) {
            matchData(d);
        }
    } else {
        matchData(d);
    }
};

ws.onclose = (event) => {
    console.log("Closed with code:", event.code, "Reason:", event.reason);
};


