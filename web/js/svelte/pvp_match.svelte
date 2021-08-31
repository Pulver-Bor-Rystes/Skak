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
</script>

<p style="color:red">
    {fen}
</p>

<!-- Vis billede i div loadet fra script som objekt --> 
<!-- <div on:mousemove={handleMousemove}> --> 
<!--  --> 
<!--  --> 