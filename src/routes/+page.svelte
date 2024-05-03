<script lang="ts">
import { onMount } from 'svelte'

let board: HTMLCanvasElement
let hover: HTMLCanvasElement
let pieces: HTMLCanvasElement
const ROWS: number = 10
const COLS: number = 10
const GAP: number = 60
let width: number = (COLS + 2) * GAP
let height: number = (ROWS + 2) * GAP
let boardX: number = 0
let boardY: number = 0
let ctxBoard: CanvasRenderingContext2D
let ctxHover: CanvasRenderingContext2D
let ctxPieces: CanvasRenderingContext2D
let pieceColor: string = 'black'

// 2d array for the board, black = 1, white = 2
let boardArray: number[][] = Array.from({ length: ROWS }, () => Array.from({ length: COLS }, () => 0))

onMount(() => {
    ctxBoard = board.getContext('2d')
    ctxHover = hover.getContext('2d')
    ctxHover.globalAlpha = 0.3
    ctxPieces = pieces.getContext('2d')

    // draw board
    ctxBoard.fillStyle = '#dcae6b'
    ctxBoard.fillRect(0, 0, width, height)
    ctxBoard.beginPath()

    // draw board lines
    let x: number = GAP
    let y: number = GAP 
    while (x < width || y < height) {
        ctxBoard.moveTo(x, GAP)
        ctxBoard.lineTo(x, GAP + GAP * ROWS)
        x += GAP
        ctxBoard.moveTo(GAP, y)
        ctxBoard.lineTo(GAP + GAP * COLS, y)
        y += GAP
    }
    ctxBoard.stroke()
})

// show transparent shadow of piece when hovering
function hovering(e: MouseEvent) {
    // set closest coordinates for piece
    let closestX: number = Math.round((e.clientX - boardX) / GAP) * GAP
    let closestY: number = Math.round((e.clientY - boardY) / GAP) * GAP

    // cap closestX and closestY
    closestX = Math.min(GAP + GAP * COLS, Math.max(GAP, closestX))
    closestY = Math.min(GAP + GAP * ROWS, Math.max(GAP, closestY))

    // draw shadow
    ctxHover.clearRect(0, 0, width, height)
    ctxHover.beginPath()
    ctxHover.arc(closestX, closestY, GAP / 2, 0, 2 * Math.PI)
    ctxHover.fillStyle = pieceColor
    ctxHover.fill()
}

// place piece on board when clicked
function placing(e: MouseEvent) {
    // set closest coordinates for piece
    let closestX: number = Math.round((e.clientX - boardX) / GAP) * GAP
    let closestY: number = Math.round((e.clientY - boardY) / GAP) * GAP

    // cap closestX and closestY
    closestX = Math.min(GAP + GAP * COLS, Math.max(GAP, closestX))
    closestY = Math.min(GAP + GAP * ROWS, Math.max(GAP, closestY))

    // check if move is legal
    let row: number = Math.floor((closestY - GAP) / GAP)
    let col: number = Math.floor((closestX - GAP) / GAP)
    if (boardArray[row][col] !== 0) return

    // draw piece
    ctxPieces.beginPath()
    ctxPieces.arc(closestX, closestY, GAP / 2, 0, 2 * Math.PI)
    ctxPieces.fillStyle = pieceColor
    ctxPieces.fill()

    // set piece color and array
    pieceColor = pieceColor === 'black' ? 'white' : 'black'
    boardArray[row][col] = pieceColor === 'black' ? 1 : 2
}

</script>

<canvas bind:this={board} class="absolute top-0 left-0" {width} {height}></canvas>
<canvas bind:this={hover} {width} {height} class="absolute top-0 left-0"></canvas>
<canvas bind:this={pieces} on:mousemove={hovering} on:click={placing} {width} {height} class="absolute top-0 left-0"></canvas>
