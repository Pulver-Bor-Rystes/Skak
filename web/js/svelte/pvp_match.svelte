<script>
    import * as data from './fen.json';
    const fen = data.fen;
    console.log(fen); // output 'testing'

    


    class Pawn {
        
        constructor(x,y) {
            this.x = x
            this.y = y
        }
    }

    var boardPlacement = 0;
    var boardSize = 400
    var zoomDown = false
    var downCoord = 0;
    var tooBig = 600
    var tooSmall = 300
    var smallestEvent = -Infinity
    var biggestEvent = Infinity
    function zoomDownActivate(event) {
        zoomDown = true
        downCoord = event.y - (boardPlacement.top+boardSize)
    }
    
    function zoomDownDeactivate() {
        if(zoomDown) zoomDown = false
    }

    function changeSize(event) {
        if(zoomDown) {
            console.log(event.y + ", " + smallestEvent)
            if(event.y > boardPlacement.top + tooSmall && event.y < boardPlacement.top + tooBig) {
                boardSize = event.y - boardPlacement.top - downCoord
            }
        }

    }
    


    
</script>



<!-- Vis billede i div loadet fra script som objekt --> 
<!-- <div on:mousemove={handleMousemove}> --> 
<!--  --> 
<!--  --> 
<window on:mousemove={changeSize} on:mouseup={zoomDownDeactivate}>

<div class="parent" >
    <img id="pee" class="image1" src="https://images.chesscomfiles.com/chess-themes/boards/dash/150.png" draggable="false" alt="Board" style="width: {boardSize}px;" on:load={
        boardPlacement = document.getElementById("pee").getBoundingClientRect()
    }>

    <img class="image2" src="https://via.placeholder.com/50" draggable="false" alt="Piece" style="width: {boardSize/8}px;">
    <img on:mousedown={zoomDownActivate}  class="image3" src="https://via.placeholder.com/50" alt="Scroll" style="width: {boardSize/15}px;" draggable="false">
  </div>
  <canvas id="visuals">

  </canvas>
</window>

  <style>
    .parent {
        position:absolute;
        top:100px;
        background-color: aliceblue;
        
    }
    .image1 {
        position: relative;
        top: 0;
        left: 0;
        border: 1px red solid;
        
    }
    .image2 {
        position: absolute;
        top: 0;
        left: 0;
        border: 1px green solid;
    }
    .image3 {
        position: relative;
        top: 0;
        left: 0;
        border: 1px green solid;
    }
  </style>