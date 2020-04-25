export function sayHello(name) {
    console.log(`Hello ${name}`);
}

let canvasEl = null;
let context = null;
let canvasWidth = 0;
let canvasHeight = 0;

export function init(canvasId, height, width) {
    canvasEl = document.getElementById(canvasId);
    canvasEl.height = height;
    canvasEl.width = width;

    context = canvasEl.getContext("2d");
    
    canvasWidth = width;
    canvasHeight = height;
}

export function clear(colorCode) {
    context.fillStyle = colorCode;
    context.fillRect(0, 0, canvasWidth, canvasHeight);
}

export function drawCircle(centerX, centerY, radius, colorR, colorG, colorB, colorA) {
    context.beginPath();
    context.arc(centerX, centerY, radius, 0, Math.PI * 2, false);
    context.fillStyle = `rgba(${colorR}, ${colorG}, ${colorB}, ${colorA})`;
    context.fill();
}