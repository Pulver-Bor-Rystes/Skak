<script>
    import * as data from './fen.json';
    const fen = data.fen;

    var l = function () {
        console.log.apply(console, arguments);
    }
    
    l(fen); // output 'testing'

    const bn ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/bn.png"
    const bb ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/bb.png"
    const br ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/br.png"
    const bq ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/bq.png"
    const bk ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/bk.png"
    const bp ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/bp.png"
    const wn ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/wn.png"
    const wb ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/wb.png"
    const wr ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/wr.png"
    const wq ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/wq.png"
    const wk ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/wk.png"
    const wp ="https://images.chesscomfiles.com/chess-themes/pieces/wood/150/wp.png"

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

    const zoomDownActivate = event => {
        zoomDown = true
        downCoord = event.y - (boardPlacement.top+boardSize)
    }
    
    const zoomDownDeactivate = () => {
        if(zoomDown) zoomDown = false
    }

    const changeSize = event => {
        if(zoomDown) {
            l(event.y + ", " + smallestEvent)
            if(event.y > boardPlacement.top + tooSmall && event.y < boardPlacement.top + tooBig) {
                boardSize = event.y - boardPlacement.top - downCoord
            }
        }

    }

    
    //Only works onclick not onload or immediately?
    const peewee = () => {
        var pieceDivs = document.querySelectorAll(".chessPiece")
        l(pieceDivs)
        for (let i = 0; i < pieceDivs.length; i++) {
            const piece = pieceDivs[i]
            piece.onclick = () => {
                console.log("pee")
            }
        }
    }
        

</script>



<!-- Vis billede i div loadet fra script som objekt --> 
<!-- <div on:mousemove={handleMousemove}> --> 
<!--  --> 
<!--  --> 
<window on:mousemove={changeSize} on:mouseup={zoomDownDeactivate} onload={peewee}>

<div class="parent">
    <img id="pee" class="image1" src="https://images.chesscomfiles.com/chess-themes/boards/dash/150.png" draggable="false" alt="Board" style="width: {boardSize}px;" on:load={
        boardPlacement = document.getElementById("pee").getBoundingClientRect()
    }>


    <div class="chessBoard" style="grid-template-columns: {boardSize/8}px {boardSize/8}px {boardSize/8}px {boardSize/8}px {boardSize/8}px {boardSize/8}px {boardSize/8}px {boardSize/8}px;">
        <img class="chessPiece" src="{br}" alt="1" draggable="false">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
        <img class="chessPiece" alt="2">
 
    </div>
    <img on:mousedown={zoomDownActivate}  class="zoomer" src="https://via.placeholder.com/50" alt="Scroll" style="width: {boardSize/15}px;" draggable="false">
    <img on:mousedown={peewee}  class="zoomer" src="https://via.placeholder.com/50" alt="Scroll" style="width: {boardSize/15}px;" draggable="false">
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
    .chessBoard {
        display: grid;
        position: absolute;
        top: 0;
        left: 0;
        border: 1px green solid;
    }
    .chessPiece {
        display: relative;
        border: 1px green solid;
    }
    .zoomer {
        position: relative;
        top: 0;
        left: 0;
        border: 1px green solid;
    }
  </style>