import './App.css';
import React, { useState, useEffect, useLayoutEffect } from 'react';
// import { CSSTransition, TransitionGroup } from 'react-transition-group';

const COPYRIGHT_STR = "Copyright @ 2020 Marcus Xu";

function Header({ subtitle }) {
    return (
        <div className="header">
            <h1>Othello</h1>
            <p>{ subtitle }</p>
        </div>
    );
}

function Boxed(element) {
    return <div className="box">{ element }</div>;
}

function Board({ board, handleClick }) {
    const blacks = board.get_blacks(); 
    const whites = board.get_whites(); 
    const moves = board.get_moves();
    var arr = new Array(64);
    for (let i = 0n; i < 64n; i++) {
        const mask = (1n << (63n - i));
        if (mask & moves) {
            arr.push(Boxed(
                <div className="circle is-empty"
                onClick={() => handleClick(i)}></div>
            ));
        } else if (mask & blacks) {
            arr.push(Boxed(
                <div className="circle is-black"></div>
            ));
        } else if (mask & whites) {
            arr.push(Boxed(
                <div className="circle is-white"></div>
            ));
        } else {
            arr.push(Boxed(null));
        }
    }
    return arr;
}

const App = () => {
    const [wasm, setWasm] = useState(null);
    const [board, setBoard] = useState(null);
    const [subtitle, setSubtitle] = useState(COPYRIGHT_STR);
    const [humanSkipped, setHumanSkipped] = useState(false);
    useEffect(async () => {
        const wasm_ = await import("wasm");
        setWasm(wasm_);
        setBoard(new wasm_.JSBoard());
    }, [/* run once */]);

    const handleClick = (i) => {
        setHumanSkipped(false);
        let board_t = board.place(1n << (63n - i));
        if (board_t.is_human_turn()) {
            alert("AI has to pass.");
        }
        setBoard(board_t);
    };

    useLayoutEffect(() => {
        if (board === null) return;
        if (board.is_human_turn()) {
            return;
        } else if (humanSkipped) {
            alert("You have to pass.");
        }
        if (board.get_moves()) {
            setHumanSkipped(true);
            setTimeout(() => {
                const fromAI = board.run_ai(1500);
                setBoard(fromAI.get_board());
                const prob = Math.round(fromAI.get_score() * 100.0);
                const count = fromAI.get_count();
                setSubtitle(`AI Prob: ${prob}%; Nodes: ${count}`);
            }, 10);    
        } else {
            const diff = board.count_blacks() - board.count_whites();
            alert(diff > 0 ? "You win..." : (diff < 0 ? "You sucks!" : "It's a draw"));
        }    
    }, [board]);

    if (board) {
        return (
            <div className="container">
                <div className="counter is-black">
                    { board.count_blacks() }
                </div>
                <Header subtitle={subtitle}></Header>
                <div className="counter is-white">
                    { board.count_whites() }
                </div>
                    <Board
                    board={board}
                    handleClick={handleClick}>
                    </Board>
            </div>
        );
    } else {
        return <h1>Detecting WASM...</h1>;
    }
}

export default App;
