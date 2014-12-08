var c = document.querySelector('#canvas');

CanvasRenderingContext2D.prototype.drawCircle = function(pos, size) {
    this.beginPath();
    this.arc(pos.x, pos.y, size/2, 0, Math.PI*2, true);
    this.fill();
}

var requestAnimationFrame = window.requestAnimationFrame || 
                            window.mozRequestAnimationFrame ||
                            window.webkitRequestAnimationFrame ||
                            window.msRequestAnimationFrame;

c.style.webkitFilter = "blur(10px)";
c.style.filter = "blur(10px)";

ctx = c.getContext('2d');

initCanvas();

function initCanvas() {   
    var w = window.innerWidth;
    var h = window.innerHeight;

    c.width = w;
    c.height = h;
 
    for (var i = 0; i < 500; i++) {
        var size = getRandom(5,100);
        var x = getRandom(size,w-size);
        var y = getRandom(size,h-size);
        ctx.drawCircle({x: x, y: y}, size);
    }

    requestAnimationFrame(initCanvas);
}

function getRandom(min, max) {
    return Math.random() * (max - min) + min;
}

window.addEventListener('resize', initCanvas, false);
